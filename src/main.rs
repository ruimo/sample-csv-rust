extern crate csv;

use csv::IteratorWithLineNo;

fn main() {
    let mut it = IteratorWithLineNo::new("Hello\nWorld".chars());
    loop {
        let line_no = it.line_no();
        let c = it.next();
        if c.is_none() {
            break
        }
        println!("{}: {}", line_no, c.unwrap());
    }
}
