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
    let start = Utc::now();
    parser.pre_process_file().unwrap();
    // parser.create_graph();
    let end = Utc::now();

    // Calculate and print the duration
    let duration = end.signed_duration_since(start);
    println!(
        "Function execution took {} seconds.",
        duration.num_seconds()
    );

    Ok(())
}
