use std::io::{self, Read};

use lexer::token::TokenType;

use super::Parser;
use crate::ast;

impl<R: Read> Parser<R> {
    /// Parses the program.
    pub(super) fn prog(&mut self) -> io::Result<Vec<ast::Stmt>> {
        let mut p: Vec<ast::Stmt> = Vec::new();

        match self.current.token_type {
            TokenType::KwFn => {
                let f = self.func()?;
                p.push(f);
                let mut crd = self.prog()?;
                p.append(&mut crd);
            }
            TokenType::EOF => {}
            _ => {
                self.syntax_error("Expected `fn`".into());
            }
        };

        Ok(p)
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
    fn func(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwFn)?;

        let id = match &self.current.token_type {
            TokenType::Iden(id) => id.to_string(),
            _ => {
                self.syntax_error("Expected identifier".into());
                "".to_string()
            }
        };
        self.next()?;

        self.expect(TokenType::LPar)?;

        let params = self.param_list()?;

        self.expect(TokenType::RPar)?;

        let t = match &self.current.token_type {
            TokenType::Colon => {
                self.next()?;
                Some(self.types()?)
            }
            _ => None,
        };

        let s = self.stmt()?;

        Ok(ast::Stmt::FnDef(id, params, t, Box::new(s)))
    }

    /// Parses the function parameters list.
    fn param_list(&mut self) -> io::Result<Vec<(String, ast::Type)>> {
        let mut params: Vec<(String, ast::Type)> = Vec::new();

        if self.current.token_type == TokenType::RPar {
            // empty param list
            return Ok(params);
        }

        let idt = self.typed_ident()?;
        params.push(idt);

        if self.current.token_type == TokenType::Comma {
            self.next()?;
            let mut cdr = self.param_list()?;
            params.append(&mut cdr);
        }

        Ok(params)
    }

    /// Parses the statement.
    fn stmt(&mut self) -> io::Result<ast::Stmt> {
        let s = match self.current.token_type {
            TokenType::KwLet => self.let_stmt()?,
            TokenType::KwIf => self.if_stmt()?,
            TokenType::KwWhile => self.while_stmt()?,
            TokenType::KwFor => self.for_stmt()?,
            TokenType::KwReturn => self.return_stmt()?,
            TokenType::LBrace => self.block_stmt()?,
            TokenType::Semicolon => {
                self.next()?;
                ast::Stmt::Empty
            }
            _ => self.expr_stmt()?
        };

        Ok(s)
    }

    /// Parses the let statement.
    fn let_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwLet)?;

        let (id, t) = self.typed_ident()?;

        let e = match &self.current.token_type {
            TokenType::Assign => {
                self.next()?;
                Some(self.expr()?)
            }
            _ => None
        };

        self.expect(TokenType::Semicolon)?;

        Ok(ast::Stmt::Let(id, t, e))
    }

    /// Parses the expression statement.
    fn expr_stmt(&mut self) -> io::Result<ast::Stmt> {
        let e = self.expr()?;

        self.expect(TokenType::Semicolon)?;

        Ok(ast::Stmt::Expr(e))
    }

    /// Parses the if statement.
    fn if_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwIf)?;

        let cond = self.expr()?;

        let s = self.stmt()?;

        let else_stmt = match self.current.token_type {
            TokenType::KwElse => {
                self.next()?;
                Some(Box::new(self.stmt()?))
            }
            _ => None,
        };

        Ok(ast::Stmt::If(cond, Box::new(s), else_stmt))
    }

    /// Parses the while statement.
    fn while_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwWhile)?;

        let cond = self.expr()?;

        let s = self.stmt()?;

        Ok(ast::Stmt::While(cond, Box::new(s)))
    }

    /// Parses the for statement.
    fn for_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwFor)?;

        let id = match &self.current.token_type {
            TokenType::Iden(id) => id.to_string(),
            _ => {
                self.syntax_error("Expected identifier".into());
                "".to_string()
            }
        };
        self.next()?;

        self.expect(TokenType::Assign)?;

        let from = self.expr()?;

        self.expect(TokenType::KwTo)?;

        let to = self.expr()?;

        let s = self.stmt()?;

        Ok(ast::Stmt::For(id, from, to, Box::new(s)))
    }

    /// Parses the return statement.
    fn return_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwReturn)?;

        let e = self.expr()?;

        self.expect(TokenType::Semicolon)?;

        Ok(ast::Stmt::Return(e))
    }

    /// Parses the block statement.
    fn block_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::LBrace)?;

        let s = self.multi_stmt()?;

        self.expect(TokenType::RBrace)?;

        Ok(ast::Stmt::Block(s))
    }

    /// Parses consecutive statements.
    fn multi_stmt(&mut self) -> io::Result<Vec<ast::Stmt>> {
        let mut stmts: Vec<ast::Stmt> = Vec::new();

        if self.current.token_type == TokenType::RBrace {
            return Ok(stmts);
        }

        let s = self.stmt()?;
        stmts.push(s);

        let mut cdr = self.multi_stmt()?;
        stmts.append(&mut cdr);

        Ok(stmts)
    }

    /// Parses the typed identifier.
    fn typed_ident(&mut self) -> io::Result<(String, ast::Type)> {
        let id = match &self.current.token_type {
            TokenType::Iden(id) => id.to_string(),
            _ => {
                self.syntax_error("Expected identifier".into());
                "".to_string()
            }
        };
        self.next()?;

        self.expect(TokenType::Colon)?;

        let t = self.types()?;

        Ok((id, t))
    }

    /// Parses the types.
    fn types(&mut self) -> io::Result<ast::Type> {
        let mut t = match self.current.token_type {
            TokenType::KwInt => ast::Type::Int,
            TokenType::KwFloat => ast::Type::Float,
            TokenType::KwChar => ast::Type::Char,
            TokenType::KwStr => ast::Type::Str,
            TokenType::KwBool => ast::Type::Bool,
            _ => {
                self.syntax_error("Expected type".into());
                ast::Type::Error
            }
        };
        self.next()?;

        if self.current.token_type == TokenType::LBracket {
            self.next()?;
            t = ast::Type::Array(Box::new(t));
            self.expect(TokenType::RBracket)?;
        }

        Ok(t)
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
                self.primary_expr()?
            }
            TokenType::Minus => {
                self.next()?;
                let e = self.primary_expr()?;
                ast::Expr::UnaryOp(ast::UnOp::Neg, Box::new(e))
            }
            TokenType::KwNot => {
                self.next()?;
                let e = self.primary_expr()?;
                ast::Expr::UnaryOp(ast::UnOp::LogNot, Box::new(e))
            }
            TokenType::Tilde => {
                self.next()?;
                let e = self.primary_expr()?;
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
        let expr = match &self.current.token_type {
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

                self.next()?;

                ast::Expr::LiteralStr(unescaped)
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

                self.next()?;

                match parsed_char {
                    Some(ch) => ast::Expr::LiteralChar(ch),
                    None => {
                        self.syntax_error("Invalid character".into());
                        ast::Expr::Error
                    }
                }
            }
            TokenType::LiteralFloat(f) => {
                let e = match f.parse::<f64>() {
                    Ok(f) => {
                        ast::Expr::LiteralFloat(f)
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error
                    }
                };

                self.next()?;
                
                e
            }
            TokenType::LiteralIntDec(n) => {
                let e = match i64::from_str_radix(&n, 10) {
                    Ok(n) => {
                        ast::Expr::LiteralInt(n)
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error
                    }
                };

                self.next()?;
                e
            }
            TokenType::LiteralIntHex(n) => {
                let trimmed = &n[2..];

                let e = match i64::from_str_radix(trimmed, 16) {
                    Ok(n) => {
                        ast::Expr::LiteralInt(n)
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error
                    }
                };

                self.next()?;

                e
            }
            TokenType::LiteralIntBin(n) => {
                let trimmed = &n[2..];

                let e = match i64::from_str_radix(trimmed, 2) {
                    Ok(n) => {
                        ast::Expr::LiteralInt(n)
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error
                    }
                };

                self.next()?;

                e
            }
            TokenType::LiteralIntOct(n) => {
                let trimmed = &n[2..];

                let e = match i64::from_str_radix(trimmed, 8) {
                    Ok(n) => {
                        ast::Expr::LiteralInt(n)
                    }
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error
                    }
                };

                self.next()?;

                e
            }
            TokenType::LBracket => {
                // array_lit
                self.next()?;
                let clist = self.comma_list()?;
                self.expect(TokenType::RBracket)?;
                ast::Expr::LiteralArray(clist)
            }
            TokenType::LPar => {
                self.next()?;
                let e = self.expr()?;
                self.expect(TokenType::RPar)?;
                e
            }
            TokenType::Iden(id) => {
                let id = id.to_string();
                self.next()?;

                match self.current.token_type {
                    TokenType::Assign => {
                        // iden = expr
                        self.next()?;
                        let e = self.expr()?;
                        ast::Expr::Assign(id, Box::new(e))
                    }
                    TokenType::LPar => {
                        // iden ( comma_list )
                        self.next()?;
                        let clist = self.comma_list()?;
                        self.expect(TokenType::RPar)?;
                        ast::Expr::Call(id, clist)
                    }
                    TokenType::LBracket => {
                        // iden [ expr ]
                        self.next()?;
                        let e = self.expr()?;
                        self.expect(TokenType::RBracket)?;
                        ast::Expr::ArrayExpr(id, Box::new(e))
                    }
                    _ => {
                        // iden
                        ast::Expr::Identifier(id)
                    }
                }
            }
            _ => {
                self.syntax_error("Expected expression".into());
                ast::Expr::Error
            }
        };

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
