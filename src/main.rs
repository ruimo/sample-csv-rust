extern crate csv;

use csv::IteratorWithLineNo;
use csv::CsvParser;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    for i in 1..1000000 {
        parse();
    }
    println!("Elapsed :{:?}", start.elapsed().unwrap());
}

fn parse() {
    let mut it = IteratorWithLineNo::new("1,\"2\",3\n5,6,7".chars());
    while it.has_next() {
        let mut parser = CsvParser::new();
        match parser.parse_line(&mut it) {
            Err(e) => panic!("{}: {}", e.line_no, e.message),
            Ok(p) => {
            }
        }
    }
}
