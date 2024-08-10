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

    /// Parses the typed identifier.
    pub(super) fn typed_ident(&mut self) -> io::Result<()> {
        if let TokenType::Iden(id) = &self.current.token_type {
            // Handle id
        } else {
            self.syntax_error("Expected identifier".into());
        }
        self.next()?;

        if self.current.token_type != TokenType::Colon {
            self.syntax_error("Expected `:`".into());
        }
        self.next()?;

        match self.current.token_type {
            TokenType::KwInt => {
                // handle int
            }
            TokenType::KwStr => {
                // handle string
            }
            TokenType::KwChar => {
                // handle char
            }
            TokenType::KwFloat => {
                // handle float
            }
            _ => {
                self.syntax_error(format!(
                    "Unexpected token `{}`. Expected type",
                    self.current
                ));
            }
        }
        self.next()?;

        Ok(())
    }

    /// Parses the expression.
    pub(super) fn expr(&mut self) -> io::Result<()> {
        self.log_or_expr()
    }

    /// Parses the logical or expression.
    pub(super) fn log_or_expr(&mut self) -> io::Result<()> {
        self.log_and_expr()?;

        while self.current.token_type == TokenType::KwOr {
            self.next()?;
            self.log_and_expr()?;
        }

        Ok(())
    }

    /// Parses the logical and expression.
    pub(super) fn log_and_expr(&mut self) -> io::Result<()> {
        self.eq_neq_expr()?;

        while self.current.token_type == TokenType::KwAnd {
            self.next()?;
            self.eq_neq_expr()?;
        }

        Ok(())
    }

    /// Parses the equality and inequality expression.
    pub(super) fn eq_neq_expr(&mut self) -> io::Result<()> {
        self.comp_expr()?;

        while self.current.token_type == TokenType::Eq || self.current.token_type == TokenType::NEq
        {
            self.next()?;
            self.comp_expr()?;
        }

        Ok(())
    }

    /// Parses the comparison expression.
    pub(super) fn comp_expr(&mut self) -> io::Result<()> {
        self.bit_or_expr()?;

        while self.current.token_type == TokenType::Lt
            || self.current.token_type == TokenType::Gt
            || self.current.token_type == TokenType::LEq
            || self.current.token_type == TokenType::GEq
        {
            self.next()?;
            self.bit_or_expr()?;
        }

        Ok(())
    }

    /// Parses the bitwise or expression.
    pub(super) fn bit_or_expr(&mut self) -> io::Result<()> {
        self.bit_and_expr()?;

        while self.current.token_type == TokenType::Pipe {
            self.next()?;
            self.bit_and_expr()?;
        }

        Ok(())
    }

    /// Parses the bitwise and expression.
    pub(super) fn bit_and_expr(&mut self) -> io::Result<()> {
        self.add_sub_expr()?;

        while self.current.token_type == TokenType::Amp {
            self.next()?;
            self.add_sub_expr()?;
        }

        Ok(())
    }

    /// Parses the addition and subtraction expression.
    pub(super) fn add_sub_expr(&mut self) -> io::Result<()> {
        self.mul_div_mod_expr()?;

        while self.current.token_type == TokenType::Plus
            || self.current.token_type == TokenType::Minus
        {
            self.next()?;
            self.mul_div_mod_expr()?;
        }

        Ok(())
    }

    /// Parses the multiplication, division and modulo expression.
    pub(super) fn mul_div_mod_expr(&mut self) -> io::Result<()> {
        self.unary_expr()?;

        while self.current.token_type == TokenType::Mul
            || self.current.token_type == TokenType::Div
            || self.current.token_type == TokenType::Mod
        {
            self.next()?;
            self.unary_expr()?;
        }

        Ok(())
    }

    /// Parses unary expressions.
    pub(super) fn unary_expr(&mut self) -> io::Result<()> {
        match self.current.token_type {
            TokenType::Plus | TokenType::Minus | TokenType::KwNot | TokenType::Tilde => {
                self.next()?;
                self.expr()?;
            }
            _ => {
                self.primary_expr()?;
            }
        }

        Ok(())
    }

    /// Parses the primary expressions.
    pub(super) fn primary_expr(&mut self) -> io::Result<()> {
        match self.current.token_type {
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
                // array_lit
                self.next()?;
                self.comma_list()?;
                if self.current.token_type != TokenType::RBracket {
                    self.syntax_error("Expected `]`".into());
                }
                self.next()?;
            }
            TokenType::LPar => {
                self.next()?;
                self.expr()?;
                if self.current.token_type != TokenType::RPar {
                    self.syntax_error("Expected `)`".into());
                }
                self.next()?;
            }
            TokenType::Iden(_) => {
                self.next()?;
                match self.current.token_type {
                    TokenType::Assign => {
                        // iden = expr
                        self.next()?;
                        self.expr()?;
                    }
                    TokenType::LPar => {
                        // iden ( comma_list )
                        self.next()?;
                        self.comma_list()?;
                        if self.current.token_type != TokenType::RPar {
                            self.syntax_error("Expected `)`".into());
                        }
                        self.next()?;
                    }
                    TokenType::LBracket => {
                        // iden [ expr ]
                        self.next()?;
                        self.expr()?;
                        if self.current.token_type != TokenType::RBracket {
                            self.syntax_error("Expected `]`".into());
                        }
                        self.next()?;
                    }
                    _ => {
                        // iden
                    },
                }
            }
            _ => {
                self.syntax_error("Expected expression".into());
            }
        }

        Ok(())
    }

    /// Parses the comma separated list.
    pub(super) fn comma_list(&mut self) -> io::Result<()> {
        if self.current.token_type == TokenType::RPar
            || self.current.token_type == TokenType::RBracket
            || self.current.token_type == TokenType::RBrace
        {
            return Ok(());
        }

        self.expr()?;

        if self.current.token_type == TokenType::Comma {
            self.next()?;
            self.comma_list()?;
        }

        Ok(())
    }
}
