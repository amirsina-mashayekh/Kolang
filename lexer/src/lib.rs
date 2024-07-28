#![warn(missing_docs)]

//! # Kolang lexer
//! Utilities for tokenizing Kolang code.

use std::io::{self, BufReader, Read};

use token::{Token, TokenType};

/// This module includes some utilities to store and represent Kolang tokens.
pub mod token;

/// The `Lexer<R>` struct allows you to scan Kolang code from any byte source
/// which implements [`Read`] trait (file, network, in-memory buffer, etc.)
/// and get tokens.
///
/// # Examples
///
/// ```
/// use lexer::{Lexer, token::TokenType};
///
/// let source = "fn main(): int {}".as_bytes();
/// let mut l = Lexer::new(source);
///
/// assert_eq!(l.next().unwrap().token_type, TokenType::KwFn);
/// assert_eq!(l.next().unwrap().token_type, TokenType::Iden("main".to_string()));
/// assert_eq!(l.next().unwrap().token_type, TokenType::LPar);
/// assert_eq!(l.next().unwrap().token_type, TokenType::RPar);
/// assert_eq!(l.next().unwrap().token_type, TokenType::Colon);
/// assert_eq!(l.next().unwrap().token_type, TokenType::KwInt);
/// assert_eq!(l.next().unwrap().token_type, TokenType::LBrace);
/// assert_eq!(l.next().unwrap().token_type, TokenType::RBrace);
/// assert_eq!(l.next().unwrap().token_type, TokenType::EOF);
/// ```
#[derive(Debug)]
pub struct Lexer<R: Read> {
    /// Current line of source code.
    line: usize,
    /// Current column (character in line) of source code.
    column: usize,
    /// Byte stream which provides source code.
    stream: BufReader<R>,
    /// Current character of source code.
    current: char,
}

impl<R: Read> Lexer<R> {
    /// Creates a new `Lexer<R>` with provided byte stream as the token source.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::Lexer;
    ///
    /// let source = "fn main(): int {}".as_bytes();
    /// let mut l = Lexer::new(source);
    /// ```
    pub fn new(stream: R) -> Self {
        Self {
            line: 1,
            column: 0,
            stream: BufReader::new(stream),
            current: ' ',
        }
    }

    /// Reads next token from provided byte stream, constructs and returns it.
    /// If Lexer reaches end of stream, it will return [`TokenType::EOF`] tokens
    /// until there are new bytes on the stream.
    ///
    /// # Errors
    /// May return I/O error if something goes wrong while reading bytes
    /// from source.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::{Lexer, token::TokenType};
    ///
    /// let source = "fn main".as_bytes();
    /// let mut l = Lexer::new(source);
    ///
    /// assert_eq!(l.next().unwrap().token_type, TokenType::KwFn);
    /// assert_eq!(l.next().unwrap().token_type, TokenType::Iden("main".to_string()));
    /// assert_eq!(l.next().unwrap().token_type, TokenType::EOF);
    /// ```
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

                        TokenType::BC(comment)
                    }
                    _ => TokenType::Slash,
                }
            }
            '\'' => {
                let c = self.match_char()?;
                match self.current {
                    '\'' => TokenType::LiteralChar(c + "\'"),
                    _ => TokenType::Invalid,
                }
            }
            '"' => {
                let s = self.match_str()?;
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

    /// Discards whitespace characters until it reaches a non-whitespace character
    /// or end of stream.
    fn consume_whitespace(&mut self) -> io::Result<()> {
        while self.current.is_whitespace() {
            self.next_char()?;
        }

        Ok(())
    }

    /// Reads next identifier (or keyword) token from stream and returns
    /// it as a string. Consumes all bytes of token. May return empty string.
    fn match_iden(&mut self) -> io::Result<String> {
        let mut id = String::new();

        while self.current.is_ascii_alphanumeric() || self.current == '_' {
            id.push(self.current);
            self.next_char()?;
        }

        Ok(id)
    }

    /// Reads next integer numeric token from stream and returns
    /// it as a string. `base` parameter defines radix or base of number
    /// (binary, octal, decimal, hexadecimal, etc. ).  Consumes all bytes
    /// of token. May return empty string.
    fn match_num(&mut self, base: u32) -> io::Result<String> {
        let mut num = String::new();

        while self.current.is_digit(base) {
            num.push(self.current);
            self.next_char()?;
        }

        Ok(num)
    }

    /// Reads next scientific number token from stream and returns
    /// it as a string. Consumes all bytes of token.
    fn match_scientific(&mut self) -> io::Result<String> {
        let mut num = self.match_num(10)?;

        if self.current == 'e' || self.current == 'E' {
            num.push(self.current);
            self.next_char()?;
            num.push_str(&self.match_num(10)?);
        }

        Ok(num)
    }

    /// Reads next character literal token from stream and returns
    /// it as a string. Consumes two (normal) or three (escaped) bytes,
    /// including starting and ending `'`.
    fn match_char(&mut self) -> io::Result<String> {
        let mut ch = String::from(self.current);
        self.next_char()?;

        ch.push(self.current);
        self.next_char()?;

        if ch == "'\\" && self.current != '\0' {
            ch.push(self.current);
            self.next_char()?;
        }

        Ok(ch)
    }

    /// Reads next string literal token from stream and returns
    /// it as a string. Consumes all bytes of token, including
    /// starting and ending `"`;
    fn match_str(&mut self) -> io::Result<String> {
        let mut s = String::from(self.current);
        self.next_char()?;

        let mut escape = false;

        while (self.current != '\"' && self.current != '\0') || escape {
            s.push(self.current);
            escape = self.current == '\\';
            self.next_char()?;
        }

        Ok(s)
    }

    /// Reads next line comment token from stream and returns
    /// it as a string. Consumes all bytes of token
    /// excluding starting `//`.
    fn match_line_comment(&mut self) -> io::Result<String> {
        let mut comment = String::new();

        while self.current != '\n' && self.current != '\0' {
            comment.push(self.current);
            self.next_char()?;
        }

        Ok(comment)
    }

    /// Reads next block comment token from stream and returns
    /// it as a string. Consumes all bytes of token excluding
    /// starting `/*` but including final `*/`.
    fn match_block_comment(&mut self) -> io::Result<String> {
        let mut comment = String::new();
        let mut asterisk = false;

        while self.current != '\0' && !(asterisk && self.current == '/') {
            comment.push(self.current);
            asterisk = self.current == '*';
            self.next_char()?;
        }
        // Push final slash
        if self.current != '\0' {
            comment.push(self.current);
        }

        Ok(comment)
    }

    /// Reads next byte from stream and puts it in `self.current` as `char`.
    /// If reaches end of stream, it will put `'\0'` to indicate end of file.
    /// Also updates `self.line` and `self.column` based on next character.
    ///
    /// # Errors
    /// May return I/O error if something goes wrong while reading bytes
    /// from source.
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
