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

    pub fn next(&mut self) -> io::Result<Token> {
        self.consume_whitespace();
        
        let tok = match self.current {
            '(' => TokenType::LPar,
            ')' => TokenType::RPar,
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            '|' => TokenType::Pipe,
            '&' => TokenType::Amp,
            '~' => TokenType::Tilde,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '\0' => TokenType::EOF,
            '<' => {
                self.next_char();
                if self.current == '=' {
                    TokenType::LEq
                } else {
                    TokenType::LT
                }
            },
            '>' => {
                self.next_char();
                if self.current == '=' {
                    TokenType::GEq
                } else {
                    TokenType::GT
                }
            },
            '!' => {
                self.next_char();
                if self.current == '=' {
                    TokenType::NEq
                } else {
                    TokenType::Invalid
                }
            },
            '=' => {
                self.next_char();
                if self.current == '=' {
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            },
            '/' => {
                self.next_char();
                match self.current {
                    '/' => {
                        self.next_char();
                        TokenType::LC(
                            "//".to_string()
                            + &self.match_line_comment()?
                        )
                    },
                    '*' => {
                        self.next_char();
                        TokenType::BC(
                            "/*".to_string()
                            + &self.match_block_comment()?
                            + "*/"
                        )
                    },
                    _ => TokenType::Invalid,
                }
            },
        };
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

    fn match_num(&mut self, base: u32) -> io::Result<String> {
        let mut num = String::new();

        while self.current.is_digit(base) {
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

    fn match_scientific(&mut self) -> io::Result<String> {
        let mut num = self.match_num(10)?;

        if self.current == 'e' || self.current == 'E' {
            num.push(self.current);
            self.next_char()?;
            num.push_str(&self.match_num(10)?);
        }

        Ok(num)
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

    fn match_line_comment(&mut self) -> io::Result<String> {
        let mut comment = String::new();

        while self.current != '\n' && self.current != '\0' {
            comment.push(self.current);
            self.next_char()?;
        }

        Ok(comment)
    }

    fn match_block_comment(&mut self) -> io::Result<String> {
        let mut comment = String::new();
        let mut asterisk = false;

        while self.current != '\0' && !(asterisk && self.current != '/') {
            comment.push(self.current);
            asterisk = self.current == '*';
            self.next_char()?;
        }
        // Pop final asterisk
        comment.pop();

        Ok(comment)
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
