use std::fs::File;
mod models;
mod multipeek;
mod parser;
mod schema;
use parser::Parser;
extern crate chrono;
use std::time::Instant;

const FILE_PATH: &str = "/wikigraph/raw_data/enwiki-latest-pages-articles.xml";
const BINARY_GRAPH_PATH: &str = "/wikigraph/raw_data/binary_graph.bin";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut parser = Parser::new(File::open(FILE_PATH)?, BINARY_GRAPH_PATH.to_owned(), db_url);
    let mut start = Instant::now();
    parser.pre_process_file().unwrap();
    let mut duration = start.elapsed();
    println!("Pre-process time: {:?}", duration);
    start = Instant::now();
    parser.set_count(9036689);
    parser.create_graph();
    duration = start.elapsed();
    println!("Graph creation time: {:?}", duration);
    // I was thinking I can apply parallel processing by having 1 process add shit into a queue and have like multiple other parser nodes handle the actual parsing,
    // I would need to use a mutex lock to not fuck up writing into the adj_list file though...
    Ok(())
}
