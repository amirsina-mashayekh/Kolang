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
            }
            TokenType::EOF => {}
            _ => {
                self.syntax_error("Expected `fn`".into());
            }
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
        if self.current.token_type == TokenType::RPar {
            // empty param list
            return Ok(());
        }

        self.typed_ident()?;

        if self.current.token_type == TokenType::Comma {
            self.next()?;
            self.param_list()?;
        }

        Ok(())
    }

    /// Parses the statement.
    pub(super) fn stmt(&mut self) -> io::Result<()> {
        match self.current.token_type {
            TokenType::KwLet => {
                self.let_stmt()?;
            }
            TokenType::KwIf => {
                self.if_stmt()?;
            }
            TokenType::KwWhile => {
                self.while_stmt()?;
            }
            TokenType::KwFor => {
                self.for_stmt()?;
            }
            TokenType::KwReturn => {
                self.return_stmt()?;
            }
            TokenType::LBrace => {
                self.block_stmt()?;
            }
            _ => {
                self.expr_stmt()?;
            }
        }

        Ok(())
    }

    /// Parses the let statement.
    pub(super) fn let_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::KwLet {
            self.syntax_error("Expected `let`".into());
        }
        self.next()?;

        self.typed_ident()?;

        if self.current.token_type == TokenType::Assign {
            self.next()?;
            self.expr()?;
        }

        if self.current.token_type != TokenType::Semicolon {
            self.syntax_error("Expected `;`".into());
        }

        Ok(())
    }

    /// Parses the expression statement.
    pub(super) fn expr_stmt(&mut self) -> io::Result<()> {
        self.expr()?;

        if self.current.token_type != TokenType::Semicolon {
            self.syntax_error("Expected `;`".into());
        }

        Ok(())
    }

    /// Parses the if statement.
    pub(super) fn if_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::KwIf {
            self.syntax_error("Expected `if`".into());
        }
        self.next()?;

        self.expr()?;

        self.stmt()?;

        if self.current.token_type == TokenType::KwElse {
            self.next()?;
            self.stmt()?;
        }

        Ok(())
    }

    /// Parses the while statement.
    pub(super) fn while_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::KwWhile {
            self.syntax_error("Expected `while`".into());
        }
        self.next()?;

        self.expr()?;

        self.stmt()?;

        Ok(())
    }

    /// Parses the for statement.
    pub(super) fn for_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::KwFor {
            self.syntax_error("Expected `for`".into());
        }
        self.next()?;

        if let TokenType::Iden(id) = &self.current.token_type {
            // Handle id
            _ = id;
        } else {
            self.syntax_error("Expected identifier".into());
        }
        self.next()?;

        if self.current.token_type != TokenType::Assign {
            self.syntax_error("Expected `=`".into());
        }
        self.next()?;
        
        self.expr()?;
        
        if self.current.token_type != TokenType::KwTo {
            self.syntax_error("Expected `to`".into());
        }
        self.next()?;
        
        self.expr()?;

        self.stmt()?;

        Ok(())
    }

    /// Parses the return statement.
    pub(super) fn return_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::KwReturn {
            self.syntax_error("Expected `return`".into());
        }
        self.next()?;

        self.expr()?;

        if self.current.token_type != TokenType::Semicolon {
            self.syntax_error("Expected `;`".into());
        }

        Ok(())
    }

    /// Parses the block statement.
    pub(super) fn block_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type != TokenType::LBrace {
            self.syntax_error("Expected `{`".into());
        }
        self.next()?;

        self.stmt()?;

        if self.current.token_type != TokenType::RBrace {
            self.syntax_error("Expected `}`".into());
        }
        self.next()?;

        Ok(())
    }

    pub(super) fn typed_ident(&mut self) -> io::Result<()> {
        todo!();
    }

    /// Parses the expression.
    pub(super) fn expr(&mut self) -> io::Result<()> {
        todo!();
    }

    pub(super) fn assign_stmt(&mut self) -> io::Result<()> {
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
                self.syntax_error(format!(
                    "Unexpedted token `{}` Expected literal",
                    self.current
                ));
            }
        }

        Ok(())
    }

    /// Parses array literals.
    pub(super) fn array_lit(&mut self) -> io::Result<()> {
        todo!();
    }
}
