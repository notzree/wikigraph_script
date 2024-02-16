use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::BufReader;

const FILE_PATH: &str =
    "/Users/notzree/Documents/Coding_Projects/rust_projects/wikigraph/raw_data/enwiki-latest-pages-articles.xml";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(FILE_PATH)?;
    let file = BufReader::new(file);
    let mut reader = Reader::from_reader(file);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    let mut itr = 0;

    loop {
        if itr > 300 {
            break;
        }
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"page" {
                    loop {
                        match reader.read_event_into(&mut buf) {
                            Ok(Event::Start(e)) => {
                                if e.name().as_ref() == b"title" {
                                    println!("Title: {:?}", e.name().as_ref());
                                }
                            }
                            Ok(Event::End(e)) => {
                                if e.name().as_ref() == b"page" {
                                    break;
                                }
                            }

                            Ok(Event::Eof) => break,
                            _ => (),
                        }
                        buf.clear();
                    }
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
        itr += 1;
    }
    for i in txt {
        println!("{}", i);
    }
    Ok(())
}
