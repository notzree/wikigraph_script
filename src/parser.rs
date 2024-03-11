use crate::adj_list_handler::{AdjacencyListHandler, WikigraphAdjacencyListHandler};
use crate::database_handler::{DatabaseHandler, PostgresDatabaseHandler};
use crate::graph_builder::{GraphBuilder, WikiBinaryGraphBuilder};
use crate::link_handler::{LinkHandler, WikiLinkHandler};
use crate::models::{LookupEntry, RedirectEntry};
use crate::utils::sanitize_string;
use core::panic;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fmt::Write as fmtWrite;
use std::fs::File;
use std::io::{BufRead, BufReader};

//All sizes are in bytes. ie: 4 * 4 = 16 bytes = 4 integers.
const FILE_HEADER_SIZE: usize = 4 * 4;
const NODE_HEADER_SIZE: usize = 4 * 4;
const LINK_SIZE: usize = 4;

const ADJACENCY_LIST_PATH: &str = "adjacency_list.txt";
const NUM_ARTICLES: u64 = 9030425;

pub struct Parser {
    file_reader: quick_xml::Reader<std::io::BufReader<File>>,
    count: i32,
    link_handler: WikiLinkHandler,
    database_handler: PostgresDatabaseHandler,
    adj_list_handler: WikigraphAdjacencyListHandler,
    graph_builder: WikiBinaryGraphBuilder,
}

impl Parser {
    pub fn new(
        file: std::fs::File,
        link_handler: WikiLinkHandler,
        database_handler: PostgresDatabaseHandler,
        adj_list_handler: WikigraphAdjacencyListHandler,
        graph_builder: WikiBinaryGraphBuilder,
    ) -> Parser {
        let mut file_reader = Reader::from_reader(BufReader::new(file));
        file_reader.trim_text(true);
        Parser {
            file_reader,
            count: 0,
            link_handler,
            database_handler,
            adj_list_handler,
            graph_builder,
        }
    }
    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }
    //First pass to generate lookup table with computed byte offsets + create text file with adjacency list
    pub fn pre_process_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bar = ProgressBar::new(NUM_ARTICLES);
        bar.set_style(
            ProgressStyle::with_template(
                "[{wide_bar:.cyan/blue}] [{elapsed_precise}] {pos:>7}/{len:7} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn fmtWrite| {
                write!(w, "{:.1}hrs", state.eta().as_secs_f64() / 3600.0).unwrap()
            })
            .progress_chars("#>-"),
        );
        // let mut adj_list = File::create(ADJACENCY_LIST_PATH).unwrap();

        let mut buf: Vec<u8> = Vec::new();
        let mut prev_offset: usize = FILE_HEADER_SIZE;
        let mut prev_length: usize = 0;
        let mut count = 0;

        loop {
            match self.file_reader.read_event_into(&mut buf) {
                Err(e) => panic!(
                    "Error at position {}: {:?}",
                    self.file_reader.buffer_position(),
                    e
                ),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => {
                    self.set_count(count);
                    bar.finish();
                    return Ok(());
                }
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
                                            if e.unescape()
                                                .unwrap()
                                                .into_owned()
                                                .contains("Wikipedia:")
                                            {
                                                break;
                                            }
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
                        if page_title.is_empty()
                            || page_txt.is_empty()
                            || page_title.contains("Template:")
                            || page_title.contains("Wikipedia:")
                            || page_title.contains("File:")
                            || page_title.contains("WP:")
                            || page_title.contains("User:")
                            || page_title.contains("Help:")
                            || page_title.contains("Draft:")
                            || page_title.len() > 255
                            || page_title.contains("(disambiguation)")
                            || page_txt.contains("{{disambiguation}}")
                            || page_txt.contains("{{disambig")
                            || page_title.contains("MOS:")
                        {
                            continue;
                        }
                        let sanitized_page_title = sanitize_string(&page_title);
                        if is_redirect {
                            let links = self.link_handler.extract_links(page_txt);
                            if links.is_empty() {
                                continue;
                            }
                            let sanitized_redirect_output = sanitize_string(&links[0]);
                            let redirect_entry = RedirectEntry {
                                redirect_from: sanitized_page_title,
                                redirect_to: sanitized_redirect_output,
                            };
                            self.database_handler
                                .add_redirect_entry(&redirect_entry)
                                .unwrap();
                            continue;
                        }
                        let links = self.link_handler.extract_links(page_txt);
                        let curr_length = self.compute_length(links.len());
                        prev_offset = self.compute_byte_offset(prev_offset, prev_length);
                        let lookup_entry = LookupEntry {
                            title: sanitized_page_title,
                            byteoffset: prev_offset.try_into().unwrap(), // in bytes
                            length: curr_length.try_into().unwrap(),
                        };
                        self.database_handler
                            .add_lookup_entry(&lookup_entry)
                            .unwrap();
                        self.adj_list_handler
                            .add_to_adj_list(&page_title, links)
                            .unwrap();

                        bar.inc(1);
                        prev_length = curr_length;
                        count += 1;
                    }
                }
                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
    }
    //Second pass to take adjacency list + lookup table -> graph in binary format.
    pub fn create_graph(&mut self) {
        const FILE_VERSION: i32 = 1;
        let bar = ProgressBar::new(NUM_ARTICLES);
        bar.set_style(
            ProgressStyle::with_template(
                "[{wide_bar:.cyan/blue}] [{elapsed_precise}] {pos:>7}/{len:7} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn fmtWrite| {
                write!(w, "{:.1}hrs", state.eta().as_secs_f64() / 3600.0).unwrap()
            })
            .progress_chars("#>-"),
        );

        self.graph_builder.write_file_header();

        for line in self.adj_list_handler.iter() {
            match line {
                Ok(line) => {
                    let mut split = line.split('|');
                    let t = split.next().unwrap();
                    let current_position = self.graph_builder.get_current_position();
                    let lookup_entry = self.database_handler.lookup_with_redirects(t).unwrap();
                    if current_position != lookup_entry.byteoffset as u64 {
                        panic!(
                            "Byteoffset mismatch. Expected: {}, Got: {}",
                            lookup_entry.byteoffset, current_position
                        );
                    }
                    let links = split.next().unwrap().split(',');
                    let num_links = links.clone().count() as i32;
                    self.graph_builder.write_node_header(num_links);
                    for link in links {
                        let link = link.to_string();
                        //Link does not work because of capitalization. Have to think of a way to fix this. Issue is some links are case sensitive, other links are not. Maybe the play is to just remove cases entirely? but idk
                        let lookup_entry =
                            self.database_handler.lookup_with_redirects(&link).unwrap();
                        self.graph_builder.write_value(lookup_entry.byteoffset);
                    }
                    // println!("{} done", t);
                    self.graph_builder.flush_writer();
                }
                Err(e) => panic!("Error reading line: {:?}", e),
            }
            bar.inc(1);
        }
        bar.finish();
    }

    fn compute_byte_offset(&self, prev_offset: usize, prev_length: usize) -> usize {
        prev_offset + prev_length
    }

    fn compute_length(&self, num_links: usize) -> usize {
        NODE_HEADER_SIZE + num_links * LINK_SIZE
    }
}
