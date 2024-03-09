use crate::utils::sanitize_string;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};
pub trait AdjacencyListHandler {
    fn add_to_adj_list(
        &mut self,
        title: &str,
        links: Vec<String>,
    ) -> Result<(), diesel::result::Error>;
    fn iter(&self) -> std::io::Lines<std::io::BufReader<&File>>;
}
pub struct WikigraphAdjacencyListHandler {
    adj_list: File,
}

impl WikigraphAdjacencyListHandler {
    pub fn new(file_path: &str) -> Self {
        let adj_list = File::create(file_path).unwrap();
        WikigraphAdjacencyListHandler { adj_list }
    }
}
impl AdjacencyListHandler for WikigraphAdjacencyListHandler {
    fn add_to_adj_list(
        &mut self,
        title: &str,
        links: Vec<String>,
    ) -> Result<(), diesel::result::Error> {
        let mut line = sanitize_string(title) + "|";
        for link in links.iter() {
            line.push_str(link);
            line.push(',');
        }
        line.push('\n');
        let _ = match self.adj_list.write_all(line.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };

        Ok(())
    }
    fn iter(&self) -> std::io::Lines<std::io::BufReader<&File>> {
        BufReader::new(&self.adj_list).lines()
    }
}
