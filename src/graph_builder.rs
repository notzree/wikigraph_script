//move create_graph code here

use byteorder::{LittleEndian, WriteBytesExt};
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Seek, Write},
};

pub trait GraphBuilder {
    fn write_file_header(&mut self);
    fn write_node_header(&mut self, num_links: i32);
    fn get_current_position(&mut self) -> u64;
    fn write_value(&mut self, value: i32);
    fn flush_writer(&mut self);
}
pub struct WikiBinaryGraphBuilder {
    graph_buf_writer: BufWriter<File>,
    count: i32,
    version: i32,
}

impl WikiBinaryGraphBuilder {
    pub fn new(binary_graph_path: String, count: i32, version: i32) -> Self {
        let graph = OpenOptions::new()
            .write(true)
            .create(true)
            .open(binary_graph_path)
            .unwrap();

        let graph_buf_writer = BufWriter::new(graph);
        WikiBinaryGraphBuilder {
            graph_buf_writer,
            count,
            version,
        }
    }
}
impl GraphBuilder for WikiBinaryGraphBuilder {
    fn write_node_header(&mut self, num_links: i32) {
        //3 integers are unused. The number of links is the 4th integer. first integer is used for traversal.
        self.graph_buf_writer.write_i32::<LittleEndian>(0).unwrap();
        self.graph_buf_writer.write_i32::<LittleEndian>(0).unwrap();
        self.graph_buf_writer.write_i32::<LittleEndian>(0).unwrap();
        self.graph_buf_writer
            .write_i32::<LittleEndian>(num_links)
            .unwrap();
    }
    fn write_file_header(&mut self) {
        self.graph_buf_writer.write_i32::<LittleEndian>(0).unwrap();
        self.graph_buf_writer.write_i32::<LittleEndian>(0).unwrap();
        self.graph_buf_writer
            .write_i32::<LittleEndian>(self.version)
            .unwrap();
        self.graph_buf_writer
            .write_i32::<LittleEndian>(self.count)
            .unwrap();
    }
    fn get_current_position(&mut self) -> u64 {
        self.graph_buf_writer.stream_position().unwrap()
    }
    fn write_value(&mut self, value: i32) {
        self.graph_buf_writer
            .write_i32::<LittleEndian>(value)
            .unwrap();
    }
    fn flush_writer(&mut self) {
        self.graph_buf_writer.flush().unwrap();
    }
}
