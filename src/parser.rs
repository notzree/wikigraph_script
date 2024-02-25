use crate::models::LookupEntry;
use crate::schema;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::{connection, prelude::*};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::{self, File};
use std::io::{BufReader, Error, Write};
//All sizes are in bytes. ie: 4 * 4 = 16 bytes = 4 integers.
const FILE_HEADER_SIZE: usize = 4 * 4;
const NODE_HEADER_SIZE: usize = 4 * 4;
const LINK_SIZE: usize = 4;
const LEFT_BRACE: char = '[';
const RIGHT_BRACE: char = ']';

pub struct Parser {
    file_reader: quick_xml::Reader<std::io::BufReader<File>>,
    output_file: std::fs::File,
    connection_string: String,
}

impl Parser {
    pub fn new(file: std::fs::File, output_file: std::fs::File, db_url: String) -> Parser {
        let mut file_reader = Reader::from_reader(BufReader::new(file));
        file_reader.trim_text(true);
        Parser {
            file_reader,
            output_file,
            connection_string: db_url,
        }
    }
    //First pass to generate lookup table with computed byte offsets + create text file with adjacency list
    pub fn pre_process_file(&mut self) {
        let mut adj_list = File::create("adjacency_list.txt").unwrap();

        let mut connection = PgConnection::establish(&self.connection_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.connection_string));
        let mut buf: Vec<u8> = Vec::new();
        let mut itr = 0;
        let mut prev_offset: usize = 16;
        let mut prev_length: usize = 0;

        loop {
            if itr > 115 {
                break;
            }
            match self.file_reader.read_event_into(&mut buf) {
                Err(e) => panic!(
                    "Error at position {}: {:?}",
                    self.file_reader.buffer_position(),
                    e
                ),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"page" {
                        let mut page_title = String::new();
                        let mut page_txt: String = String::new();
                        let mut is_redirect: bool = false;
                        buf.clear();
                        loop {
                            match self.file_reader.read_event_into(&mut buf) {
                                Ok(Event::Start(e)) => {
                                    if e.name().as_ref() == b"title" {
                                        let text_event = self.file_reader.read_event_into(&mut buf);
                                        if let Ok(Event::Text(e)) = text_event {
                                            page_title = e.unescape().unwrap().into_owned();
                                        }
                                        continue;
                                    }
                                    if e.name().as_ref() == b"text" {
                                        let text_event = self.file_reader.read_event_into(&mut buf);
                                        if let Ok(Event::Text(e)) = text_event {
                                            page_txt = e.unescape().unwrap().into_owned();
                                        }
                                    }
                                    continue;
                                }
                                Ok(Event::End(e)) => {
                                    if e.name().as_ref() == b"page" {
                                        //Reached </page> tag
                                        break;
                                    }
                                }
                                Ok(Event::Eof) => break,
                                Ok(Event::Empty(e)) => {
                                    if e.name().as_ref() == b"redirect" {
                                        is_redirect = true;
                                        continue;
                                    }
                                }
                                _ => (),
                            }
                            buf.clear();
                        }
                        if is_redirect {
                            continue;
                        }
                        let links = self.extract_links_from_text(page_txt);
                        let curr_length = self.compute_length(links.len());
                        prev_offset = self.compute_byte_offset(prev_offset, prev_length);
                        //write to adjacency list + database
                        // match self.add_to_look_up_table(
                        //     &page_title,
                        //     &mut connection,
                        //     prev_offset,
                        //     curr_length,
                        // ) {
                        //     Ok(_) => (),
                        //     Err(e) => panic!("Error adding to lookup table: {:?}", e),
                        // }
                        // match self.add_to_adj_list(&page_title, links, &mut adj_list) {
                        //     Ok(_) => (),
                        //     Err(e) => panic!("Error adding to lookup table: {:?}", e),
                        // }
                        println!(
                            "title:{} | links: {} offset: {} length: {}, prev_length: {}",
                            page_title,
                            links.len(),
                            prev_offset,
                            curr_length,
                            prev_length
                        );
                        //update prev_offset and prev_length

                        prev_length = curr_length;
                    }
                }

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
            itr += 1;
        }
    }
    //Second pass to take adjacency list + lookup table -> graph in binary format.
    pub fn create_graph(&self) {}
    fn extract_links_from_text(&self, text: String) -> Vec<String> {
        //TODO: Fix the function properly to exclude file aliases (see the "a" )
        let mut links: Vec<String> = Vec::new();
        let mut current_link = String::new();
        let mut inside_link = false;

        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '[' if chars.peek() == Some(&'[') => {
                    // Detect starting "[["
                    chars.next(); // Skip the next '[' as it's part of the marker

                    inside_link = true;
                    current_link.clear();
                }
                ']' if chars.peek() == Some(&']') => {
                    // Detect ending "]]"
                    chars.next(); // Skip the next ']' as it's part of the marker
                    if inside_link {
                        if current_link.contains('|') {
                            let mut split = current_link.split('|');
                            let link = split.next().unwrap();
                            current_link = link.to_string();
                        }
                        links.push(current_link.clone());
                        inside_link = false;
                    }
                }
                _ if inside_link => {
                    current_link.push(c);
                    if current_link == "File:"
                        || current_link == "Wikipedia:"
                        || current_link == "WP:"
                    {
                        // we realize that we are in either a file, template, or wikipedia article namespace. We reseet
                        inside_link = false;
                        current_link.clear();
                    }
                }
                _ => {}
            }
        }

        links
    }

    fn add_to_look_up_table(
        &self,
        title: &str,
        connection: &mut PgConnection,
        byteoffset: usize,
        num_links: usize,
    ) -> Result<(), diesel::result::Error> {
        let lookup_entry = LookupEntry {
            title: title.to_string(),
            byteoffset: byteoffset.try_into().unwrap(), // in bytes
            length: (4 + 4 * num_links).try_into().unwrap(), //4 bytes for node header + 4 bytes (1 int) for each link
        };
        match insert_into(schema::lookup::table)
            .values(&lookup_entry)
            .execute(connection)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    fn add_to_adj_list(
        &self,
        title: &str,
        links: Vec<String>,
        file: &mut File,
    ) -> Result<(), std::io::Error> {
        let mut line = title.to_string() + "|";
        for link in links.iter() {
            line.push_str(link);
            line.push(',');
        }
        line.push('\n');
        let _ = match file.write_all(line.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };

        Ok(())
    }
    fn compute_byte_offset(&self, prev_offset: usize, prev_length: usize) -> usize {
        prev_offset + prev_length
    }
    fn compute_length(&self, num_links: usize) -> usize {
        NODE_HEADER_SIZE + num_links * LINK_SIZE
    }
}
