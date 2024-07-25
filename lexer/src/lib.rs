use std::io::{self, BufReader, Read};

use token::Token;
use token_type::TokenType;

pub mod token;
pub mod token_type;

pub struct Lexer<R: Read> {
    line: usize,
    column: usize,
    stream: BufReader<R>,
    current: char,
}

impl<R: Read> Lexer<R> {
    pub fn new(stream: R) -> Self {
        Self {
            line: 1,
            column: 0,
            stream: BufReader::new(stream),
            current: ' ',
        }
    }

    fn consume_whitespace(&mut self) -> io::Result<()> {
        while self.current.is_whitespace() {
            self.next_char()?;
        }

        Ok(())
    }

    fn match_iden(&mut self) -> io::Result<String> {
        let mut id = String::new();

        while self.current.is_ascii_alphanumeric() || self.current == '_' {
            id.push(self.current);
            self.next_char()?;
        }

        Ok(id)
    }

    fn match_dec(&mut self) -> io::Result<String> {
        let mut num = String::new();

        while '0' <= self.current && self.current <= '9' {
            num.push(self.current);
            self.next_char()?;
        }

        Ok(num)
    }

    fn match_bin(&mut self) -> io::Result<String> {
        let mut num = String::new();

        while '0' == self.current || self.current <= '1' {
            num.push(self.current);
            self.next_char()?;
        }

        Ok(num)
    }

    fn match_oct(&mut self) -> io::Result<String> {
        let mut num = String::new();

        while '0' <= self.current && self.current <= '7' {
            num.push(self.current);
            self.next_char()?;
        }

        Ok(num)
    }

    fn match_hex(&mut self) -> io::Result<String> {
        let mut num = String::new();

        while ('0' <= self.current && self.current <= '9')
            || ('a' <= self.current && self.current <= 'f')
            || ('A' <= self.current && self.current <= 'F')
        {
            num.push(self.current);
            self.next_char()?;
        }

        Ok(num)
    }

    fn match_char(&mut self) -> io::Result<String> {
        let mut ch = String::from(self.current);
        self.next_char()?;

        if ch == "\\" && self.current != '\0' {
            ch.push(self.current);
            self.next_char()?;
        }

        Ok(ch)
    }

    fn match_str(&mut self) -> io::Result<String> {
        let mut s = String::new();
        let mut escape = false;

        while self.current != '\"' || self.current != '\0' || escape {
            s.push(self.current);
            escape = self.current == '\\';
            self.next_char()?;
        }

        Ok(s)
    }

    fn next_char(&mut self) -> io::Result<()> {
        let mut buf = [0u8];
        let c = self.stream.read(&mut buf)?;

        self.current = if c == 1 {
            if self.current == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            buf[0] as char
        } else {
            '\0'
        };

        Ok(())
    }
}
