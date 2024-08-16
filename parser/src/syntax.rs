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

    /// Expects a token. Consumes the token if matches,
    /// otherwise raises syntax error.
    fn expect(&mut self, expected: TokenType) -> io::Result<()> {
        if self.current.token_type == expected {
            self.next()?;
        } else {
            self.syntax_error(format!("Expected `{}`", expected));
        }

        Ok(())
    }

    /// Parses the function.
    fn func(&mut self) -> io::Result<()> {
        self.expect(TokenType::KwFn)?;

        if let TokenType::Iden(id) = &self.current.token_type {
            // Handle id
        } else {
            self.syntax_error("Expected identifier".into());
        }
        self.next()?;

        self.expect(TokenType::LPar)?;

        self.param_list()?;

        self.expect(TokenType::RPar)?;

        if self.current.token_type == TokenType::Colon {
            // fn iden ( expr ) : type
            self.next()?;
            self.types()?;
        }

        self.stmt()?;

        Ok(())
    }

    /// Parses the function parameters list.
    fn param_list(&mut self) -> io::Result<()> {
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
    fn stmt(&mut self) -> io::Result<()> {
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
    fn let_stmt(&mut self) -> io::Result<()> {
        self.expect(TokenType::KwLet)?;

        self.typed_ident()?;

        if self.current.token_type == TokenType::Assign {
            self.next()?;
            self.expr()?;
        }

        self.expect(TokenType::Semicolon)?;

        Ok(())
    }

    /// Parses the expression statement.
    fn expr_stmt(&mut self) -> io::Result<()> {
        self.expr()?;

        self.expect(TokenType::Semicolon)?;

        Ok(())
    }

    /// Parses the if statement.
    fn if_stmt(&mut self) -> io::Result<()> {
        self.expect(TokenType::KwIf)?;

        self.expr()?;

        self.stmt()?;

        if self.current.token_type == TokenType::KwElse {
            self.next()?;
            self.stmt()?;
        }

        Ok(())
    }

    /// Parses the while statement.
    fn while_stmt(&mut self) -> io::Result<()> {
        self.expect(TokenType::KwWhile)?;

        self.expr()?;

        self.stmt()?;

        Ok(())
    }

    /// Parses the for statement.
    fn for_stmt(&mut self) -> io::Result<()> {
        self.expect(TokenType::KwFor)?;

        if let TokenType::Iden(id) = &self.current.token_type {
            // Handle id
        } else {
            self.syntax_error("Expected identifier".into());
        }
        self.next()?;

        self.expect(TokenType::Assign)?;

        self.expr()?;

        self.expect(TokenType::KwTo)?;

        self.expr()?;

        self.stmt()?;

        Ok(())
    }

    /// Parses the return statement.
    fn return_stmt(&mut self) -> io::Result<()> {
        self.expect(TokenType::KwReturn)?;

        self.expr()?;

        self.expect(TokenType::Semicolon)?;

        Ok(())
    }

    /// Parses the block statement.
    fn block_stmt(&mut self) -> io::Result<()> {
        self.expect(TokenType::LBrace)?;

        self.multi_stmt()?;

        self.expect(TokenType::RBrace)?;

        Ok(())
    }

    /// Parses consecutive statements.
    fn multi_stmt(&mut self) -> io::Result<()> {
        if self.current.token_type == TokenType::RBrace {
            return Ok(());
        }

        self.stmt()?;

        self.multi_stmt()?;

        Ok(())
    }

    /// Parses the typed identifier.
    fn typed_ident(&mut self) -> io::Result<()> {
        if let TokenType::Iden(id) = &self.current.token_type {
            // Handle id
        } else {
            self.syntax_error("Expected identifier".into());
        }
        self.next()?;

        self.expect(TokenType::Colon)?;

        self.types()?;

        Ok(())
    }

    /// Parses the types.
    fn types(&mut self) -> io::Result<()> {
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
                self.syntax_error("Expected type".into());
            }
        }
        self.next()?;

        Ok(())
    }

    /// Parses the expression.
    fn expr(&mut self) -> io::Result<()> {
        self.log_or_expr()
    }

    /// Parses the logical or expression.
    fn log_or_expr(&mut self) -> io::Result<()> {
        self.log_and_expr()?;

        while self.current.token_type == TokenType::KwOr {
            self.next()?;
            self.log_and_expr()?;
        }

        Ok(())
    }

    /// Parses the logical and expression.
    fn log_and_expr(&mut self) -> io::Result<()> {
        self.eq_neq_expr()?;

        while self.current.token_type == TokenType::KwAnd {
            self.next()?;
            self.eq_neq_expr()?;
        }

        Ok(())
    }

    /// Parses the equality and inequality expression.
    fn eq_neq_expr(&mut self) -> io::Result<()> {
        self.comp_expr()?;

        while self.current.token_type == TokenType::Eq || self.current.token_type == TokenType::NEq
        {
            self.next()?;
            self.comp_expr()?;
        }

        Ok(())
    }

    /// Parses the comparison expression.
    fn comp_expr(&mut self) -> io::Result<()> {
        self.bit_or_expr()?;

        while self.current.token_type == TokenType::LT
            || self.current.token_type == TokenType::GT
            || self.current.token_type == TokenType::LEq
            || self.current.token_type == TokenType::GEq
        {
            self.next()?;
            self.bit_or_expr()?;
        }

        Ok(())
    }

    /// Parses the bitwise or expression.
    fn bit_or_expr(&mut self) -> io::Result<()> {
        self.bit_and_expr()?;

        while self.current.token_type == TokenType::Pipe {
            self.next()?;
            self.bit_and_expr()?;
        }

        Ok(())
    }

    /// Parses the bitwise and expression.
    fn bit_and_expr(&mut self) -> io::Result<()> {
        self.add_sub_expr()?;

        while self.current.token_type == TokenType::Amp {
            self.next()?;
            self.add_sub_expr()?;
        }

        Ok(())
    }

    /// Parses the addition and subtraction expression.
    fn add_sub_expr(&mut self) -> io::Result<()> {
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
    fn mul_div_mod_expr(&mut self) -> io::Result<()> {
        self.unary_expr()?;

        while self.current.token_type == TokenType::Asterisk
            || self.current.token_type == TokenType::Slash
            || self.current.token_type == TokenType::Percent
        {
            self.next()?;
            self.unary_expr()?;
        }

        Ok(())
    }

    /// Parses unary expressions.
    fn unary_expr(&mut self) -> io::Result<()> {
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
    fn primary_expr(&mut self) -> io::Result<()> {
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
                // array_lit
                self.next()?;
                self.comma_list()?;
                self.expect(TokenType::RBracket)?;
            }
            TokenType::LPar => {
                self.next()?;
                self.expr()?;
                self.expect(TokenType::RPar)?;
            }
            TokenType::Iden(id) => {
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
                        self.expect(TokenType::RPar)?;
                    }
                    TokenType::LBracket => {
                        // iden [ expr ]
                        self.next()?;
                        self.expr()?;
                        self.expect(TokenType::RBracket)?;
                    }
                    _ => {
                        // iden
                    }
                }
            }
            _ => {
                self.syntax_error("Expected expression".into());
            }
        }

        Ok(())
    }

    /// Parses the comma separated list.
    fn comma_list(&mut self) -> io::Result<()> {
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
