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

        let line = self.current.line;
        let column = self.current.column;

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

        let return_type = match &self.current.token_type {
            TokenType::Colon => {
                self.next()?;
                Some(self.types()?)
            }
            _ => None,
        };

        let body = Box::new(self.stmt()?);

        Ok(ast::Stmt::FnDef {
            id,
            params,
            return_type,
            body,
            line,
            column,
        })
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
                ast::Stmt::Empty {
                    line: self.current.line,
                    column: self.current.column,
                }
            }
            _ => self.expr_stmt()?,
        };

        Ok(s)
    }

    /// Parses the let statement.
    fn let_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwLet)?;

        let line = self.current.line;
        let column = self.current.column;

        let (id, var_type) = self.typed_ident()?;

        let expr = match &self.current.token_type {
            TokenType::Assign => {
                self.next()?;
                Some(self.expr()?)
            }
            _ => None,
        };

        self.expect(TokenType::Semicolon)?;

        Ok(ast::Stmt::Let {
            id,
            var_type,
            expr,
            line,
            column,
        })
    }

    /// Parses the expression statement.
    fn expr_stmt(&mut self) -> io::Result<ast::Stmt> {
        let expr = self.expr()?;

        self.expect(TokenType::Semicolon)?;

        Ok(ast::Stmt::Expr { expr })
    }

    /// Parses the if statement.
    fn if_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwIf)?;

        let line = self.current.line;
        let column = self.current.column;

        let cond = self.expr()?;

        let then_stmt = Box::new(self.stmt()?);

        let else_stmt = match self.current.token_type {
            TokenType::KwElse => {
                self.next()?;
                Some(Box::new(self.stmt()?))
            }
            _ => None,
        };

        Ok(ast::Stmt::If {
            cond,
            then_stmt,
            else_stmt,
            line,
            column,
        })
    }

    /// Parses the while statement.
    fn while_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwWhile)?;

        let line = self.current.line;
        let column = self.current.column;

        let cond = self.expr()?;

        let body = Box::new(self.stmt()?);

        Ok(ast::Stmt::While {
            cond,
            body,
            line,
            column,
        })
    }

    /// Parses the for statement.
    fn for_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwFor)?;

        let line = self.current.line;
        let column = self.current.column;

        let id = match &self.current.token_type {
            TokenType::Iden(id) => id.to_string(),
            _ => {
                self.syntax_error("Expected identifier".into());
                "".to_string()
            }
        };
        self.next()?;

        self.expect(TokenType::Assign)?;

        let start = self.expr()?;

        self.expect(TokenType::KwTo)?;

        let end = self.expr()?;

        let body = Box::new(self.stmt()?);

        Ok(ast::Stmt::For {
            id,
            start,
            end,
            body,
            line,
            column,
        })
    }

    /// Parses the return statement.
    fn return_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::KwReturn)?;

        let line = self.current.line;
        let column = self.current.column;

        let expr = self.expr()?;

        self.expect(TokenType::Semicolon)?;

        Ok(ast::Stmt::Return { expr, line, column })
    }

    /// Parses the block statement.
    fn block_stmt(&mut self) -> io::Result<ast::Stmt> {
        self.expect(TokenType::LBrace)?;

        let line = self.current.line;
        let column = self.current.column;

        let stmts = self.multi_stmt()?;

        self.expect(TokenType::RBrace)?;

        Ok(ast::Stmt::Block {
            stmts,
            line,
            column,
        })
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
        let line = self.current.line;
        let column = self.current.column;

        let mut t = match self.current.token_type {
            TokenType::KwInt => ast::Type::Int { line, column },
            TokenType::KwFloat => ast::Type::Float { line, column },
            TokenType::KwChar => ast::Type::Char { line, column },
            TokenType::KwStr => ast::Type::Str { line, column },
            TokenType::KwBool => ast::Type::Bool { line, column },
            _ => {
                self.syntax_error("Expected type".into());
                ast::Type::Error { line, column }
            }
        };
        self.next()?;

        if self.current.token_type == TokenType::LBracket {
            self.next()?;
            t = ast::Type::Array {
                element_type: Box::new(t),
                line,
                column,
            };
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
            let op = ast::BinOp::LogOr {
                line: self.current.line,
                column: self.current.column,
            };
            self.next()?;

            let r = self.log_and_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the logical and expression.
    fn log_and_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.eq_neq_expr()?;

        while self.current.token_type == TokenType::KwAnd {
            let op = ast::BinOp::LogAnd {
                line: self.current.line,
                column: self.current.column,
            };
            self.next()?;

            let r = self.eq_neq_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the equality and inequality expression.
    fn eq_neq_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.comp_expr()?;

        loop {
            let line = self.current.line;
            let column = self.current.column;

            let op = match self.current.token_type {
                TokenType::Eq => ast::BinOp::Eq { line, column },
                TokenType::NEq => ast::BinOp::NEq { line, column },
                _ => break,
            };
            self.next()?;

            let r = self.comp_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the comparison expression.
    fn comp_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.bit_or()?;

        loop {
            let line = self.current.line;
            let column = self.current.column;

            let op = match self.current.token_type {
                TokenType::LT => ast::BinOp::LT { line, column },
                TokenType::GT => ast::BinOp::GT { line, column },
                TokenType::LEq => ast::BinOp::LEq { line, column },
                TokenType::GEq => ast::BinOp::GEq { line, column },
                _ => break,
            };
            self.next()?;

            let r = self.bit_or()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the bitwise or expression.
    fn bit_or(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.bit_and_expr()?;

        while self.current.token_type == TokenType::Pipe {
            let op = ast::BinOp::BitOr {
                line: self.current.line,
                column: self.current.column,
            };
            self.next()?;

            let r = self.bit_and_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the bitwise and expression.
    fn bit_and_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.add_sub_expr()?;

        while self.current.token_type == TokenType::Amp {
            let op = ast::BinOp::BitAnd {
                line: self.current.line,
                column: self.current.column,
            };
            self.next()?;

            let r = self.add_sub_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the addition and subtraction expression.
    fn add_sub_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.mul_div_mod_expr()?;

        loop {
            let line = self.current.line;
            let column = self.current.column;

            let op = match self.current.token_type {
                TokenType::Plus => ast::BinOp::Add { line, column },
                TokenType::Minus => ast::BinOp::Sub { line, column },
                _ => break,
            };
            self.next()?;

            let r = self.mul_div_mod_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses the multiplication, division and modulo expression.
    fn mul_div_mod_expr(&mut self) -> io::Result<ast::Expr> {
        let mut l = self.unary_expr()?;

        loop {
            let line = self.current.line;
            let column = self.current.column;

            let op = match self.current.token_type {
                TokenType::Asterisk => ast::BinOp::Mul { line, column },
                TokenType::Slash => ast::BinOp::Div { line, column },
                TokenType::Percent => ast::BinOp::Mod { line, column },
                _ => break,
            };
            self.next()?;

            let r = self.unary_expr()?;
            l = ast::Expr::BinaryOp {
                l: Box::new(l),
                op,
                r: Box::new(r),
            };
        }

        Ok(l)
    }

    /// Parses unary expressions.
    fn unary_expr(&mut self) -> io::Result<ast::Expr> {
        let line = self.current.line;
        let column = self.current.column;

        let e = match self.current.token_type {
            TokenType::Plus => {
                self.next()?;
                self.primary_expr()?
            }
            TokenType::Minus => {
                let op = ast::UnOp::Neg { line, column };
                self.next()?;

                let expr = Box::new(self.primary_expr()?);
                ast::Expr::UnaryOp { op, expr }
            }
            TokenType::KwNot => {
                let op = ast::UnOp::LogNot { line, column };
                self.next()?;

                let expr = Box::new(self.primary_expr()?);
                ast::Expr::UnaryOp { op, expr }
            }
            TokenType::Tilde => {
                let op = ast::UnOp::BitNot { line, column };
                self.next()?;

                let expr = Box::new(self.primary_expr()?);
                ast::Expr::UnaryOp { op, expr }
            }
            _ => self.primary_expr()?,
        };

        Ok(e)
    }

    /// Parses the primary expressions.
    fn primary_expr(&mut self) -> io::Result<ast::Expr> {
        let line = self.current.line;
        let column = self.current.column;

        let e = match &self.current.token_type {
            TokenType::LiteralStr(s) => {
                let raw = s.trim_matches('"').to_string();
                let value = raw
                    .replace("\\n", "\n")
                    .replace("\\\"", "\"")
                    .replace("\\t", "\t")
                    .replace("\\\\", "\\")
                    .replace("\\r", "\r")
                    .replace("\\'", "'")
                    .replace("\\0", "\0");

                self.next()?;

                ast::Expr::LiteralStr {
                    value,
                    line,
                    column,
                }
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
                    Some(value) => ast::Expr::LiteralChar {
                        value,
                        line,
                        column,
                    },
                    None => {
                        self.syntax_error("Invalid character".into());
                        ast::Expr::Error { line, column }
                    }
                }
            }
            TokenType::LiteralFloat(f) => {
                let expr = match f.parse::<f64>() {
                    Ok(value) => ast::Expr::LiteralFloat { value, line, column },
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error { line, column }
                    }
                };

                self.next()?;

                expr
            }
            TokenType::LiteralIntDec(n) => {
                let expr = match i64::from_str_radix(&n, 10) {
                    Ok(value) => ast::Expr::LiteralInt { value, line, column },
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error { line, column }
                    }
                };

                self.next()?;
                expr
            }
            TokenType::LiteralIntHex(n) => {
                let trimmed = &n[2..];

                let expr = match i64::from_str_radix(trimmed, 16) {
                    Ok(value) => ast::Expr::LiteralInt { value, line, column },
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error { line, column }
                    }
                };

                self.next()?;

                expr
            }
            TokenType::LiteralIntBin(n) => {
                let trimmed = &n[2..];

                let expr = match i64::from_str_radix(trimmed, 2) {
                    Ok(value) => ast::Expr::LiteralInt { value, line, column },
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error { line, column }
                    }
                };

                self.next()?;

                expr
            }
            TokenType::LiteralIntOct(n) => {
                let trimmed = &n[2..];

                let expr = match i64::from_str_radix(trimmed, 8) {
                    Ok(value) => ast::Expr::LiteralInt { value, line, column },
                    Err(e) => {
                        self.syntax_error(format!("Invalid integer, {}", e.to_string()));
                        ast::Expr::Error { line, column }
                    }
                };

                self.next()?;

                expr
            }
            TokenType::KwTrue => {
                self.next()?;
                ast::Expr::LiteralBool { value: true, line, column }
            }
            TokenType::KwFalse => {
                self.next()?;
                ast::Expr::LiteralBool { value: false, line, column }
            }
            TokenType::LBracket => {
                // array_lit
                self.next()?;
                let elements = self.comma_list()?;
                self.expect(TokenType::RBracket)?;
                ast::Expr::LiteralArray { elements, line, column }
            }
            TokenType::LPar => {
                self.next()?;
                let expr = self.expr()?;
                self.expect(TokenType::RPar)?;
                expr
            }
            TokenType::Iden(id) => {
                let id = id.to_string();
                self.next()?;

                match self.current.token_type {
                    TokenType::Assign => {
                        // iden = expr
                        self.next()?;
                        let expr = Box::new(self.expr()?);
                        ast::Expr::Assign { id, expr, line, column }
                    }
                    TokenType::LPar => {
                        // iden ( comma_list )
                        self.next()?;
                        let args = self.comma_list()?;
                        self.expect(TokenType::RPar)?;
                        ast::Expr::Call { id, args, line, column }
                    }
                    TokenType::LBracket => {
                        // iden [ expr ]
                        self.next()?;
                        let index = Box::new(self.expr()?);
                        self.expect(TokenType::RBracket)?;
                        ast::Expr::ArrayExpr { id, index, line, column }
                    }
                    _ => {
                        // iden
                        ast::Expr::Identifier { id, line, column }
                    }
                }
            }
            _ => {
                self.syntax_error("Expected expression".into());
                ast::Expr::Error { line, column }
            }
        };

        Ok(e)
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
