use diesel::pg::PgConnection;
use diesel::prelude::*;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::BufReader;
//All sizes are in bytes. ie: 4 * 4 = 16 bytes = 4 integers.
const FILE_HEADER_SIZE: usize = 4 * 4;
const NODE_HEADER_SIZE: usize = 4 * 4;
const LINK_SIZE: usize = 4;

pub struct Parser {
    file_reader: quick_xml::Reader<std::io::BufReader<File>>,
    buffer: Vec<u8>,
    output_file: std::fs::File,
    db_conn: PgConnection,
}

impl Parser {
    pub fn new(file: std::fs::File, output_file: std::fs::File, db_url: &str) -> Parser {
        let mut connection =
            PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));
        let mut file_reader = Reader::from_reader(BufReader::new(file));
        file_reader.trim_text(true);
        Parser {
            file_reader,
            buffer: Vec::new(),
            output_file,
            db_conn: connection,
        }
    }
    //First pass to generate lookup table with computed byte offsets + create text file with adjacency list
    pub fn pre_process_file(&self) {
        loop {
            if itr > 300 {
                break;
            }
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"page" {
                        let mut page_title = String::new();
                        let mut page_txt: Vec<String> = Vec::new();
                        buf.clear();
                        loop {
                            match reader.read_event_into(&mut buf) {
                                Ok(Event::Start(e)) => {
                                    if e.name().as_ref() == b"title" {
                                        let text_event = reader.read_event_into(&mut buf);
                                        if let Ok(Event::Text(e)) = text_event {
                                            page_title = e.unescape().unwrap().into_owned();
                                        }
                                        buf.clear();
                                        continue;
                                    }
                                    if e.name().as_ref() == b"text" {
                                        let text_event = reader.read_event_into(&mut buf);
                                        if let Ok(Event::Text(e)) = text_event {
                                            page_txt.push(e.unescape().unwrap().into_owned());
                                        }
                                        buf.clear();
                                    }
                                }
                                Ok(Event::End(e)) => {
                                    if e.name().as_ref() == b"page" {
                                        //Reached </page> tag
                                        break;
                                    }
                                }
                                Ok(Event::Eof) => break,
                                _ => (),
                            }
                            buf.clear();
                        }
                        println!("Title: {:?}", page_title);
                        for line in page_txt {
                            println!("{}", line);
                        }
                        add_to_look_up_table(title)
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
    pub fn create_graph(&self){

    }
    fn compute_byte_offset(num_links: i32)}
    fn add_to_look_up_table(title: &str, previous_offset: i32, num_links: i32) {}
    fn extract_links_from_text() {}
}
