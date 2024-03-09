use std::fs::File;
mod adj_list_handler;
mod database_handler;
mod graph_builder;
mod link_handler;
mod models;
mod parser;
mod schema;
mod utils;
use parser::Parser;
extern crate chrono;
use std::time::Instant;

const FILE_PATH: &str = "raw_data/enwiki-latest-pages-articles.xml";
const BINARY_GRAPH_PATH: &str = "raw_data/binary_graph.bin";
const VERSION: i32 = 1;
const ADJ_LIST_PATH: &str = "adjacency_list.txt";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_handler = database_handler::PostgresDatabaseHandler::new(&db_url)?;
    let graph_builder =
        graph_builder::WikiBinaryGraphBuilder::new(BINARY_GRAPH_PATH.to_owned(), 0, VERSION);
    let link_handler = link_handler::WikiLinkHandler;
    let adj_list_handler = adj_list_handler::WikigraphAdjacencyListHandler::new(ADJ_LIST_PATH);
    let mut parser = Parser::new(
        File::open(FILE_PATH)?,
        link_handler,
        database_handler,
        adj_list_handler,
        graph_builder,
    );
    let start = Instant::now();
    parser.pre_process_file().unwrap();
    let duration = start.elapsed();
    println!("Graph creation time: {:?}", duration);

    Ok(())
}
