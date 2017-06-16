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

    fn on_char(&mut self, s: &State, c: char, line_no: u32) -> Result<State, CsvParseError> {
        match *s {
            State::Init => self.on_init(c, line_no),
            State::InQuote => self.on_in_quote(c, line_no),
            State::InQuoteQuote => self.on_in_quote_quote(c, line_no),
            State::End => self.on_end(c, line_no),
        }
    }

    fn on_init(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
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
            '"' => {
                if self.buf.is_empty() { Ok(State::InQuote) }
                else {
                    Err(CsvParseError {
                        message: String::from("Parse error. Should be enclosed by double quote if data contains double quotes."),
                        line_no: line_no
                    })
                }
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

    fn on_in_quote(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
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

    fn on_in_quote_quote(&mut self, c: char, line_no: u32) -> Result<State, CsvParseError> {
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

    fn on_end(&mut self, c: char, _: u32) -> Result<State, CsvParseError> {
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
                    match self.on_char(&state, c, itr.line_no) {
                        Err(e) => return Err(e),
                        Ok(next_state) => match next_state {
                            State::End => return Ok(self),
                            _ => {state = next_state}
                        }
                    }
                },
                None => {
                    match self.on_char(&state, '\0', itr.line_no) {
                        Err(e) => return Err(e),
                        Ok(_) => return Ok(self)
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
