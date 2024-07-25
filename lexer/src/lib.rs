use std::io::{self, BufReader, Read};

use token::Token;
use token_type::TokenType;

pub mod token_type;
pub mod token;

pub struct Lexer<R: Read> {
    line: usize,
    column: usize,
    stream: BufReader<R>,
    current: char,
}

impl<R: Read> Lexer <R> {
    pub fn new(stream: R) -> Self {
        Self {
            line: 1,
            column: 1,
            stream: BufReader::new(stream),
            current: ' ',
        }
    }

    fn consume_whitespace(&mut self) -> io::Result<()> {
        while self.current.is_whitespace() {
            self.advance()?;
        }

        Ok(())
    }

    fn match_iden(&mut self) -> io::Result<String> {
        let mut id = String::new();

        while self.current.is_ascii_alphanumeric() || self.current == '_' {
            id.push(self.current);
            self.advance()?;
        }

        Ok(id)
    }

    fn match_num(&mut self) -> io::Result<String> {
        let mut num = String::new();

        while '0' <= self.current && self.current <= '9' {
            num.push(self.current);
            self.advance()?;
        }

        Ok(num)
    }

    fn next(&mut self) -> io::Result<char> {
        let mut buf = [0 as u8];
        
        match self.stream.read(&mut buf) {
            Ok(0) => Ok('\0'),
            Ok(_) => Ok(buf[0] as char),
            Err(e) => Err(e),
        }
    }

    fn advance(&mut self) -> io::Result<()> {
        let c = self.next()?;
        self.current = c;

        Ok(())
    }
}