use crate::utils::sanitize_string;
use std::{
    fs::{File, OpenOptions},
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
        let adj_list = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // This will create the file if it doesn't exist.
            .append(true) // This ensures that data is appended to the file.
            .open(file_path)
            .unwrap();
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
