#![warn(missing_docs)]

//! # Kolang parser
//! Utilities for parsing Kolang code.

use std::io::{self, Read};

use lexer::{
    token::{Token, TokenType},
    Lexer,
};

mod syntax;

/// The `Parser<R>` struct allows you to parse Kolang code from any byte source
/// which implements [`Read`] trait (file, network, in-memory buffer, etc.).
pub struct Parser<R: Read> {
    /// The `Lexer<R>` instance which provides source code tokens.
    lexer: Lexer<R>,
    /// The current token being processed.
    current: Token,
}

impl<R: Read> Parser<R> {
    /// Creates a new `Parser<R>` with provided lexer as the token source.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::Lexer;
    /// use parser::Parser;
    ///
    /// let source = "fn main(): int {}".as_bytes();
    /// let l = Lexer::new(source);
    /// let p = Parser::new(l);
    /// ```
    pub fn new(lexer: Lexer<R>) -> Self {
        Self {
            lexer,
            current: Token::new(0, 0, TokenType::LC("".to_string())),
        }
    }

    /// Starts parsing the provided souce code.
    /// # Examples
    ///
    /// ```
    /// use lexer::Lexer;
    /// use parser::Parser;
    ///
    /// let source = "fn main(): int {}".as_bytes();
    /// let l = Lexer::new(source);
    /// let mut p = Parser::new(l);
    /// p.parse();
    /// ```
    pub fn parse(&mut self) -> io::Result<()> {
        self.next()?;
        self.prog()?;

        Ok(())
    }

    /// Advances to the next token.
    fn next(&mut self) -> io::Result<()> {
        loop {
            self.current = self.lexer.next()?;

            match self.current.token_type {
                TokenType::LC(_) | TokenType::BC(_) => continue,
                TokenType::Invalid(_) => {
                    self.syntax_error(format!("Invalid token `{}`", self.current));
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn syntax_error(&mut self, msg: String) {
        panic!(
            "{}:{}: Syntax error: {}",
            self.current.line, self.current.column, msg
        );
    }
}
