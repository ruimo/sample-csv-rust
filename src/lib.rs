use std::str::Chars;

pub struct IteratorWithLineNo<'a> {
    chars: Chars<'a>,
    line_no: u32,
}

impl<'a> IteratorWithLineNo<'a> {
    pub fn new(c: Chars<'a>) -> IteratorWithLineNo<'a> {
        IteratorWithLineNo {
            chars: c,
            line_no: 1,
        }
    }

    pub fn line_no(&self) -> u32 {
        self.line_no
    }
}

impl<'a> Iterator for IteratorWithLineNo<'a> { 
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let oc = self.chars.next();
        if let Some(ch) = oc {
            if ch == '\n' {
                self.line_no += 1
            }
        };
        oc
    }
}

pub struct CsvParseError {
}

pub struct CsvParser {
    result: Vec<String>
}

enum State {
    Init,
    InQuote,
    InQuoteQuote,
    End,
}

impl CsvParser {
    pub fn new() -> CsvParser {
        CsvParser {
            result: vec!()
        }
    }

    fn onChar(&mut self, s: State, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match s {
            State::Init => {
                
                Ok(State::Init)
            },
            State::InQuote => Ok(State::Init),
            State::InQuoteQuote => Ok(State::Init),
            State::End => Ok(State::Init),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
