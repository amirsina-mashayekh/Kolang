use std::io::{self, BufReader, Read};

use token::{Token, TokenType};

pub mod token;

#[derive(Debug)]
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
        self.consume_whitespace()?;

        let line = self.line;
        let column = self.column;
        let mut consumed = false;

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
            '%' => TokenType::Percent,
            '|' => TokenType::Pipe,
            '&' => TokenType::Amp,
            '~' => TokenType::Tilde,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '\0' => TokenType::EOF,
            '<' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::LEq
                } else {
                    TokenType::LT
                }
            }
            '>' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::GEq
                } else {
                    TokenType::GT
                }
            }
            '!' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::NEq
                } else {
                    TokenType::Invalid
                }
            }
            '=' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            }
            '/' => {
                self.next_char()?;
                match self.current {
                    '/' => {
                        self.next_char()?;
                        consumed = true;
                        TokenType::LC("//".to_string() + &self.match_line_comment()?)
                    }
                    '*' => {
                        self.next_char()?;
                        let mut comment = "/*".to_string();
                        comment.push_str(&self.match_block_comment()?);
                        if self.current != '\0' {
                            comment.push_str("*/");
                        }

                        TokenType::BC(comment)
                    }
                    _ => TokenType::Slash,
                }
            }
            '\'' => {
                let mut c = String::from(self.current);
                self.next_char()?;
                c.push_str(&self.match_char()?);
                match self.current {
                    '\'' => TokenType::LiteralChar(c + "\'"),
                    _ => TokenType::Invalid,
                }
            }
            '"' => {
                let mut s = String::from(self.current);
                self.next_char()?;
                s.push_str(&self.match_str()?);
                match self.current {
                    '"' => TokenType::LiteralChar(s + "\""),
                    _ => TokenType::Invalid,
                }
            }
            '.' => {
                self.next_char()?;
                if self.current.is_digit(10) {
                    // float literal
                    let mut f = '.'.to_string();
                    f.push_str(&self.match_scientific()?);
                    consumed = true;
                    TokenType::LiteralFloat(f)
                } else {
                    TokenType::Period
                }
            }
            c => {
                consumed = true;
                let mut tmp = String::new();
                if c.is_ascii_alphabetic() || c == '_' {
                    // identifier or keyword
                    tmp.push_str(&self.match_iden()?);
                    match tmp.as_str() {
                        "for" => TokenType::KwFor,
                        "to" => TokenType::KwTo,
                        "while" => TokenType::KwWhile,
                        "if" => TokenType::KwIf,
                        "else" => TokenType::KwElse,
                        "true" => TokenType::KwTrue,
                        "false" => TokenType::KwFalse,
                        "or" => TokenType::KwOr,
                        "and" => TokenType::KwAnd,
                        "not" => TokenType::KwNot,
                        "let" => TokenType::KwLet,
                        "fn" => TokenType::KwFn,
                        "int" => TokenType::KwInt,
                        "char" => TokenType::KwChar,
                        "bool" => TokenType::KwBool,
                        "float" => TokenType::KwFloat,
                        "str" => TokenType::KwStr,
                        _ => TokenType::Iden(tmp),
                    }
                } else if c.is_digit(10) {
                    // numeric (int or float)
                    tmp.push_str(&self.match_num(10)?);

                    if tmp.as_str() == "0" {
                        // prefixed int?
                        match self.current {
                            'b' | 'B' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_num(2)?);
                                TokenType::LiteralIntBin(tmp)
                            }
                            'o' | 'O' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_num(8)?);
                                TokenType::LiteralIntOct(tmp)
                            }
                            'x' | 'X' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_num(16)?);
                                TokenType::LiteralIntHex(tmp)
                            }
                            '.' => {
                                tmp.push(self.current);
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            'e' => {
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            _ => TokenType::LiteralIntDec(tmp),
                        }
                    } else {
                        match self.current {
                            '.' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            'e' => {
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            _ => TokenType::LiteralIntDec(tmp),
                        }
                    }
                } else {
                    TokenType::Invalid
                }
            }
        };

        if !consumed && tok != TokenType::Invalid {
            self.next_char()?;
        }

        Ok(Token {
            token_type: tok,
            line,
            column,
        })
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

        while (self.current != '\"' && self.current != '\0') || escape {
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

        while self.current != '\0' && !(asterisk && self.current == '/') {
            comment.push(self.current);
            asterisk = self.current == '*';
            self.next_char()?;
        }
        // Pop final asterisk
        if self.current != '\0' {
            comment.pop();
        }

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
