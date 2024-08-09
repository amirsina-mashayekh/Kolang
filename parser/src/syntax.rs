use std::io::{self, Read};

use lexer::token::TokenType;

use super::Parser;

impl<R: Read> Parser<R> {
    /// Parses the program.
    pub(super) fn prog(&mut self) -> io::Result<()> {
        match self.current.token_type {
            TokenType::KwFn => {
                self.func()?;
                self.prog()?;
            },
            TokenType::EOF => {
            },
            _ => {
                self.syntax_error("Expected `fn`".into());
            },
        };

        Ok(())
    }

    /// Parses the function.
    pub(super) fn func(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::KwFn {
            self.syntax_error("Expected `fn`".into());
        }
        self.next()?;
        
        if let TokenType::Iden(id) = &self.current.token_type {
            // Handle id
            _ = id;
        } else {
            self.syntax_error("Expected identifier".into());
        }
        self.next()?;
        
        if self.current.token_type != TokenType::LPar {
            self.syntax_error("Expected `(`".into());
        }
        self.next()?;
        
        self.param_list()?;
        
        if self.current.token_type != TokenType::RPar {
            self.syntax_error("Expected `)`".into());
        }
        self.next()?;

        self.stmt()?;

        Ok(())
    }

    /// Parses the function parameters list.
    pub(super) fn param_list(&mut self) -> io::Result<()> {
        todo!();
    }

    /// Parses the function parameters list.
    pub(super) fn stmt(&mut self) -> io::Result<()> {
        todo!();
    }

    /// Parses literals.
    pub(super) fn lit(&mut self) -> io::Result<()> {
        match &self.current.token_type {
            TokenType::LiteralIntDec(n) => {
                // handle decimal int
                self.next()?;
            }
            TokenType::LiteralStr(s) => {
                // handle string
                self.next()?;
            }
            TokenType::LiteralChar(c) => {
                // handle char
                self.next()?;
            }
            TokenType::LiteralFloat(f) => {
                // handle float
                self.next()?;
            }
            TokenType::LiteralIntHex(n) => {
                // handle hex int
                self.next()?;
            }
            TokenType::LiteralIntBin(n) => {
                // handle binary int
                self.next()?;
            }
            TokenType::LiteralIntOct(n) => {
                // handle octal int
                self.next()?;
            }
            TokenType::LBracket => {
                self.array_lit()?;
            }
            _ => {
                self.syntax_error(
                    format!("Unexpedted token `{}` Expected literal", self.current),
                );
            }
        }

        Ok(())
    }

    /// Parses array literals.
    pub(super) fn array_lit(&mut self) -> io::Result<()> {
        todo!();
    }
}
