use std::{fmt::Error, fs::File, io::{self, BufReader, Read}};

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

    fn next(&mut self) -> io::Result<char> {
        let mut buf = [0 as u8];
        
        match self.stream.read(&mut buf) {
            Ok(0) => Ok('\0'),
            Ok(_) => Ok(buf[0] as char),
            Err(e) => Err(e),
        }
    }
}