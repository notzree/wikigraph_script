use std::fs::File;
mod models;
mod multipeek;
mod parser;
mod schema;
use parser::Parser;
extern crate chrono;
use chrono::prelude::*;

const FILE_PATH: &str = "/wikigraph/raw_data/enwiki-latest-pages-articles.xml";
const BINARY_GRAPH_PATH: &str = "/wikigraph/raw_data/binary_graph.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut parser = Parser::new(File::open(FILE_PATH)?, BINARY_GRAPH_PATH.to_owned(), db_url);
    println!("Pre-processing");
    parser.pre_process_file().unwrap();
    println!("Creating graph");
    parser.create_graph();

    Ok(())
}
