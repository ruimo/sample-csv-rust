extern crate csv;

use csv::IteratorWithLineNo;
use csv::CsvParseError;
use csv::CsvParser;

fn main() {
    let mut it = IteratorWithLineNo::new("1,\"2\",3\n5,6,7".chars());
    println!("start!");
    while it.has_next() {
        println!("itr");
        let mut parser = CsvParser::new();
        match parser.parse_line(&mut it) {
            Err(e) => println!("{}: {}", e.line_no, e.message),
            Ok(p) => {
                println!("ok");
                for e in &p.result {
                    println!("{}", e)
                }
            }
        }
    }
}
