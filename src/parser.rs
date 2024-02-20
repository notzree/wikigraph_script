use crate::models::LookupEntry;
use crate::schema;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::{connection, prelude::*};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::BufReader;
//All sizes are in bytes. ie: 4 * 4 = 16 bytes = 4 integers.
const FILE_HEADER_SIZE: usize = 4 * 4;
const NODE_HEADER_SIZE: usize = 4 * 4;
const LINK_SIZE: usize = 4;

pub struct Parser {
    file_reader: quick_xml::Reader<std::io::BufReader<File>>,
    output_file: std::fs::File,
    db_conn: PgConnection,
}

impl Parser {
    pub fn new(file: std::fs::File, output_file: std::fs::File, db_url: &str) -> Parser {
        let connection = PgConnection::establish(db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
        let mut file_reader = Reader::from_reader(BufReader::new(file));
        file_reader.trim_text(true);
        Parser {
            file_reader,
            output_file,
            db_conn: connection,
        }
    }
    //First pass to generate lookup table with computed byte offsets + create text file with adjacency list
    pub fn pre_process_file(&mut self) {
        let mut adj_list = File::create("adjacency_list.txt").unwrap();
        let mut connection = &self.db_conn;
        let mut buf: Vec<u8> = Vec::new();
        let mut itr = 0;
        loop {
            if itr > 300 {
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
                        let mut page_txt: Vec<String> = Vec::new();
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
                                            page_txt.push(e.unescape().unwrap().into_owned());
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
                        println!("Title: {}", page_title);
                        for l in links.iter() {
                            println!("{}", l);
                        }
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
    fn add_to_look_up_table(
        title: &str,
        mut connection: PgConnection,
        byteoffset: i32,
        num_links: i32,
    ) {
        let LookupEntry = LookupEntry {
            title: title.to_string(),
            byteoffset,                // in bytes
            length: 4 + 4 * num_links, //4 bytes for node header + 4 bytes (1 int) for each link
        };
    }

    fn compute_byte_offset(num_links: i32) -> i32 {
        //todo: implement
    }
    fn compute_length(num_links: i32) -> i32 {
        //todo: implement
    }
    fn extract_links_from_text(&self, mut page_text: Vec<String>) -> Vec<String> {
        let mut links: Vec<String> = Vec::new();
        for line in page_text.iter() {
            let words = line.split_whitespace();
            for word in words {
                if word.starts_with("[[") && word.ends_with("]]") {
                    links.push(word.trim_matches(|c| c == '[' || c == ']').into());
                }
            }
        }
        links
    }
}
