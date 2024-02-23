use dotenv::dotenv;

use std::fs::File;
use std::io::BufReader;
mod models;
mod parser;
mod schema;
use parser::Parser;

const FILE_PATH: &str = "/wikigraph/raw_data/enwiki-latest-pages-articles.xml";
const BINARY_GRAPH_PATH: &str = "/wikigraph/raw_data/binary_graph.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut parser = Parser::new(
        File::open(FILE_PATH)?,
        File::create(BINARY_GRAPH_PATH)?,
        db_url,
    );
    parser.pre_process_file(); //
    let link = parser.extract_links("[[File:WilhelmWeitling.jpg|thumb|[[Wilhelm Weitling]] is an example of a writer who added to anarchist theory without using the exact term.{{sfn|Carlson|1972|pp=22–23}}]]");
    println!("{:?}", link);

    Ok(())
}
