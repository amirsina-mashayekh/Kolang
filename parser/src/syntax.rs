use std::io::{self, Read};

use lexer::token::TokenType;

use super::Parser;
use crate::ast;

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
    fn expr(&mut self) -> io::Result<ast::Expr> {
        self.log_or_expr()
    }

    /// Parses the logical or expression.
    fn log_or_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.log_and_expr()?;

        while self.current.token_type == TokenType::KwOr {
            self.next()?;
            let r = self.log_and_expr()?;
            l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::LogOr, Box::new(r));
        }

        Ok(l)
    }

    /// Parses the logical and expression.
    fn log_and_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.eq_neq_expr()?;

        while self.current.token_type == TokenType::KwAnd {
            self.next()?;
            let r = self.eq_neq_expr()?;
            l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::LogAnd, Box::new(r));
        }

        Ok(l)
    }

    /// Parses the equality and inequality expression.
    fn eq_neq_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.comp_expr()?;

        loop {
            match self.current.token_type {
                TokenType::Eq => {
                    self.next()?;
                    let r = self.comp_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::Eq, Box::new(r));
                }
                TokenType::NEq => {
                    self.next()?;
                    let r = self.comp_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::NEq, Box::new(r));
                }
                _ => break,
            }
        }

        Ok(l)
    }

    /// Parses the comparison expression.
    fn comp_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.bit_or()?;

        loop {
            match self.current.token_type {
                TokenType::LT => {
                    self.next()?;
                    let r = self.bit_or()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::LT, Box::new(r));
                }
                TokenType::GT => {
                    self.next()?;
                    let r = self.bit_or()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::GT, Box::new(r));
                }
                TokenType::LEq => {
                    self.next()?;
                    let r = self.bit_or()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::LEq, Box::new(r));
                }
                TokenType::GEq => {
                    self.next()?;
                    let r = self.bit_or()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::GEq, Box::new(r));
                }
                _ => break,
            }
        }

        Ok(l)
    }

    /// Parses the bitwise or expression.
    fn bit_or(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.bit_and_expr()?;

        while self.current.token_type == TokenType::Pipe {
            self.next()?;
            let r = self.bit_and_expr()?;
            l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::BitOr, Box::new(r));
        }

        Ok(l)
    }

    /// Parses the bitwise and expression.
    fn bit_and_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.add_sub_expr()?;

        while self.current.token_type == TokenType::Amp {
            self.next()?;
            let r = self.add_sub_expr()?;
            l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::BitAnd, Box::new(r));
        }

        Ok(l)
    }

    /// Parses the addition and subtraction expression.
    fn add_sub_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.mul_div_mod_expr()?;

        loop {
            match self.current.token_type {
                TokenType::Plus => {
                    self.next()?;
                    let r = self.mul_div_mod_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::Add, Box::new(r));
                }
                TokenType::Minus => {
                    self.next()?;
                    let r = self.mul_div_mod_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::Sub, Box::new(r));
                }
                _ => break,
            }
        }

        Ok(l)
    }

    /// Parses the multiplication, division and modulo expression.
    fn mul_div_mod_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.unary_expr()?;

        loop {
            match self.current.token_type {
                TokenType::Asterisk => {
                    self.next()?;
                    let r = self.unary_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::Mul, Box::new(r));
                }
                TokenType::Slash => {
                    self.next()?;
                    let r = self.unary_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::Div, Box::new(r));
                }
                TokenType::Percent => {
                    self.next()?;
                    let r = self.unary_expr()?;
                    l = ast::Expr::BinaryOp(Box::new(l), ast::BinOp::Mod, Box::new(r));
                }
                _ => break,
            }
        }

        Ok(l)
    }

    /// Parses unary expressions.
    fn unary_expr(&mut self) -> io::Result<ast::Expr> {
        let expr = match self.current.token_type {
            TokenType::Plus => {
                self.next()?;
                self.expr()?
            }
            TokenType::Minus => {
                self.next()?;
                let e = self.expr()?;
                ast::Expr::UnaryOp(ast::UnOp::Neg, Box::new(e))
            }
            TokenType::KwNot => {
                self.next()?;
                let e = self.expr()?;
                ast::Expr::UnaryOp(ast::UnOp::LogNot, Box::new(e))
            }
            TokenType::Tilde => {
                self.next()?;
                let e = self.expr()?;
                ast::Expr::UnaryOp(ast::UnOp::BitNot, Box::new(e))
            }
            _ => {
                self.primary_expr()?
            }
        };

        Ok(expr)
    }

    /// Parses the primary expressions.
    fn primary_expr(&mut self) -> io::Result<ast::Expr> {
        let mut expr: ast::Expr = ast::Expr::Error;

        match &self.current.token_type {
            TokenType::LiteralIntDec(n) => {
                match i64::from_str_radix(&n, 10) {
                    Ok(n) => {
                        expr = ast::Expr::LiteralInt(n);
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                    }
                }

                self.next()?;
            }
            TokenType::LiteralStr(s) => {
                let raw = s.trim_matches('"').to_string();
                let unescaped = raw
                    .replace("\\n", "\n")
                    .replace("\\\"", "\"")
                    .replace("\\t", "\t")
                    .replace("\\\\", "\\")
                    .replace("\\r", "\r")
                    .replace("\\'", "'")
                    .replace("\\0", "\0");
                expr = ast::Expr::LiteralStr(unescaped);

                self.next()?;
            }
            TokenType::LiteralChar(c) => {
                let trimmed = c.trim_matches('\'');

                let parsed_char = match trimmed.len() {
                    1 => trimmed.chars().next(),
                    _ => match trimmed {
                        "\\n" => Some('\n'),
                        "\\'" => Some('\''),
                        "\\\"" => Some('"'),
                        "\\t" => Some('\t'),
                        "\\\\" => Some('\\'),
                        "\\r" => Some('\r'),
                        "\\0" => Some('\0'),
                        _ => None,
                    },
                };

                match parsed_char {
                    Some(ch) => expr = ast::Expr::LiteralChar(ch),
                    None => self.syntax_error("Invalid character".into()),
                }

                self.next()?;
            }
            TokenType::LiteralFloat(f) => {
                match f.parse::<f64>() {
                    Ok(f) => {
                        expr = ast::Expr::LiteralFloat(f);
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                    }
                }

                self.next()?;
            }
            TokenType::LiteralIntHex(n) => {
                let trimmed = &n[2..];

                match i64::from_str_radix(trimmed, 16) {
                    Ok(n) => {
                        expr = ast::Expr::LiteralInt(n);
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                    }
                }

                self.next()?;
            }
            TokenType::LiteralIntBin(n) => {
                let trimmed = &n[2..];

                match i64::from_str_radix(trimmed, 2) {
                    Ok(n) => {
                        expr = ast::Expr::LiteralInt(n);
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                    }
                }
                self.next()?;
            }
            TokenType::LiteralIntOct(n) => {
                let trimmed = &n[2..];

                match i64::from_str_radix(trimmed, 8) {
                    Ok(n) => {
                        expr = ast::Expr::LiteralInt(n);
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                    }
                }
                self.next()?;
            }
            TokenType::LBracket => {
                // array_lit
                self.next()?;
                let clist = self.comma_list()?;
                expr = ast::Expr::LiteralArray(clist);
                self.expect(TokenType::RBracket)?;
            }
            TokenType::LPar => {
                self.next()?;
                expr = self.expr()?;
                self.expect(TokenType::RPar)?;
            }
            TokenType::Iden(id) => {
                let id = id.to_string();
                self.next()?;

                match self.current.token_type {
                    TokenType::Assign => {
                        // iden = expr
                        self.next()?;
                        let e = self.expr()?;
                        expr = ast::Expr::Assign(id, Box::new(e));
                    }
                    TokenType::LPar => {
                        // iden ( comma_list )
                        self.next()?;
                        let clist = self.comma_list()?;
                        expr = ast::Expr::Call(id, clist);
                        self.expect(TokenType::RPar)?;
                    }
                    TokenType::LBracket => {
                        // iden [ expr ]
                        self.next()?;
                        let e = self.expr()?;
                        self.expect(TokenType::RBracket)?;
                        expr = ast::Expr::ArrayExpr(id, Box::new(e));
                    }
                    _ => {
                        // iden
                        expr = ast::Expr::Identifier(id);
                    }
                }
            }
            _ => {
                self.syntax_error("Expected expression".into());
            }
        }

        Ok(expr)
    }

    /// Parses the comma separated list.
    fn comma_list(&mut self) -> io::Result<Vec<ast::Expr>> {
        let mut lst: Vec<ast::Expr> = Vec::new();

        if self.current.token_type == TokenType::RPar
            || self.current.token_type == TokenType::RBracket
            || self.current.token_type == TokenType::RBrace
        {
            return Ok(lst);
        }

        let expr = self.expr()?;
        lst.push(expr);

        if self.current.token_type == TokenType::Comma {
            self.next()?;
            let mut cdr = self.comma_list()?;
            lst.append(&mut cdr);
        }

        Ok(lst)
    }
}
