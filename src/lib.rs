use std::str::Chars;
use std::iter::Peekable;

pub struct IteratorWithLineNo<'a> {
    chars: Peekable<Chars<'a>>,
    line_no: u32,
}

impl<'a> IteratorWithLineNo<'a> {
    pub fn new(c: Chars<'a>) -> IteratorWithLineNo<'a> {
        IteratorWithLineNo {
            chars: c.peekable(),
            line_no: 1,
        }
    }

    pub fn line_no(&self) -> u32 {
        self.line_no
    }

    pub fn has_next(&mut self) -> bool {
        self.chars.peek().is_some()
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
    pub message: String,
    pub line_no: u32,
}

pub struct CsvParser {
    pub result: Vec<String>,
    buf: String,
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
            result: vec!(),
            buf: String::new()
        }
    }

    fn onChar(&mut self, s: &State, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match *s {
            State::Init => self.onInit(c, line_no),
            State::InQuote => self.onInQuote(c, line_no),
            State::InQuoteQuote => self.onInQuoteQuote(c, line_no),
            State::End => self.onEnd(c, line_no),
        }
    }

    fn onInit(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match c {
            '\0' => {
                self.result.push(self.buf.clone());
                Ok(State::End)
            },
            '\n' => {
                self.result.push(self.buf.clone());
                Ok(State::End)
            },
            '\r' => Ok(State::Init),
            '"' =>
                if self.buf.is_empty() { Ok(State::InQuote) }
                else {
                    Err(CsvParseError {
                        message: String::from("Parse error. Should be enclosed by double quote if data contains double quotes."),
                        line_no: line_no
                    })
                },
            ',' => {
                self.result.push(self.buf.clone());
                self.buf.clear();
                Ok(State::Init)
            },
            _ => {
                self.buf.push(c);
                Ok(State::Init)
            }
        }
    }

    fn onInQuote(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match c {
            '\0' => Err(
                CsvParseError {
                    message: String::from("Parse error. Quote is not closed."),
                    line_no: line_no
                }
            ),
            '"' => Ok(State::InQuoteQuote),
            _ => {
                self.buf.push(c);
                Ok(State::InQuote)
            }
        }
    }

    fn onInQuoteQuote(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match c {
            '\0' => {
                self.result.push(self.buf.clone());
                Ok(State::End)
            },
            ',' => {
                self.result.push(self.buf.clone());
                self.buf.clear();
                Ok(State::Init)
            },
            '\r' => Ok(State::InQuoteQuote),
            '"' => {
                self.buf.push(c);
                Ok(State::InQuote)
            }
            _ => Err(
                CsvParseError {
                    message: format!("Parse error. Invalid character '{}' after quote.", c),
                    line_no: line_no
                }
            )
        }
    }

    fn onEnd(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match c {
            _ => Err(
                CsvParseError {
                    message: String::from("End state does not accept input."),
                    line_no: 0,
                }
            )
        }
    }

    pub fn parse_line(&mut self, mut itr: &mut IteratorWithLineNo) -> Result<&Self, CsvParseError> {
        self.buf.clear();
        self.result.clear();

        let mut state = State::Init;
        loop {
            match itr.next() {
                Some(c) => {
                    match self.onChar(&state, c, itr.line_no) {
                        Err(e) => return Err(e),
                        Ok(next_state) => match next_state {
                            State::End => return Ok(self),
                            _ => {}
                        }
                    }
                },
                None => {
                    match self.onChar(&state, '\0', itr.line_no) {
                        Err(e) => return Err(e),
                        Ok(next_state) => return Ok(self)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
