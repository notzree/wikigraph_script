use std::fs::File;
mod models;
mod multipeek;
mod parser;
mod schema;
use parser::Parser;
extern crate chrono;
use diesel::pg::PgConnection;
use diesel::{connection, prelude::*};
use std::time::Instant;

const FILE_PATH: &str = "raw_data/enwiki-latest-pages-articles.xml";
const BINARY_GRAPH_PATH: &str = "raw_data/binary_graph.bin";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut parser = Parser::new(File::open(FILE_PATH)?, BINARY_GRAPH_PATH.to_owned(), db_url);
    // let mut connection = PgConnection::establish(&db_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    // parser.lookup_with_redirects("", &mut connection);

    let start = Instant::now();
    parser.create_graph();
    let duration = start.elapsed();
    println!("Graph creation time: {:?}", duration);

    Ok(())
}
