use dotenv::dotenv;

use std::fs::File;
use std::io::BufReader;
mod models;
mod parser;
mod schema;
use parser::Parser;

const FILE_PATH: &str =
    "/Users/notzree/Documents/Coding_Projects/rust_projects/wikigraph/raw_data/enwiki-latest-pages-articles.xml";
const BINARY_GRAPH_PATH: &str =
    "/Users/notzree/Documents/Coding_Projects/rust_projects/wikigraph/raw_data/binary_graph.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
    let parser = Parser::new(
        File::open(FILE_PATH)?,
        File::create(BINARY_GRAPH_PATH)?,
        &format!(
            "postgres://{}:{}@localhost:5432/{}",
            db_user, db_password, db_name
        ),
    );

    Ok(())
}
