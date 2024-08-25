use std::fmt;

#[derive(PartialEq)]
pub enum Expr {
    LiteralInt(i64),
    LiteralStr(String),
    LiteralChar(char),
    LiteralFloat(f64),
    LiteralBool(bool),
    LiteralArray(Vec<Expr>),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>),
    Identifier(String),
    Call(String, Vec<Expr>),
    ArrayExpr(String, Box<Expr>),
    Assign(String, Box<Expr>),
    Error,
}

#[derive(PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogAnd,
    LogOr,
    BitAnd,
    BitOr,
    Eq,
    NEq,
    LT,
    GT,
    LEq,
    GEq,
}

#[derive(PartialEq, Eq)]
pub enum UnOp {
    Neg,
    LogNot,
    BitNot,
}

#[derive(PartialEq)]
pub enum Stmt {
    Let(String, Type, Option<Expr>),
    Expr(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(String, Expr, Expr, Box<Stmt>),
    Return(Expr),
    Block(Vec<Stmt>),
    FnDef(String, Vec<(String, Type)>, Option<Type>, Box<Stmt>),
    Empty,
}

#[derive(PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Char,
    Str,
    Bool,
    Array(Box<Type>),
    Error,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::LiteralInt(i) => write!(f, "{}", i),
            Expr::LiteralStr(s) => write!(f, "\"{}\"", s),
            Expr::LiteralChar(c) => write!(f, "{}", c),
            Expr::LiteralFloat(fl) => write!(f, "{}", fl),
            Expr::LiteralBool(b) => write!(f, "{}", b),
            Expr::LiteralArray(arr) => {
                write!(f, "[")?;
                for (i, e) in arr.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i != arr.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expr::BinaryOp(e1, op, e2) => write!(f, "({} {} {})", e1, op, e2),
            Expr::UnaryOp(op, e) => write!(f, "({} {})", op, e),
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::Call(id, args) => {
                write!(f, "{}(", id)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::ArrayExpr(id, idx) => write!(f, "{}[{}]", id, idx),
            Expr::Assign(id, e) => write!(f, "{} = {}", id, e),
            Expr::Error => write!(f, "err_expr"),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::LogAnd => write!(f, "and"),
            BinOp::LogOr => write!(f, "or"),
            BinOp::BitAnd => write!(f, "&"),
            BinOp::BitOr => write!(f, "|"),
            BinOp::Eq => write!(f, "=="),
            BinOp::NEq => write!(f, "!="),
            BinOp::LT => write!(f, "<"),
            BinOp::GT => write!(f, ">"),
            BinOp::LEq => write!(f, "<="),
            BinOp::GEq => write!(f, ">="),
        }
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::LogNot => write!(f, "not"),
            UnOp::BitNot => write!(f, "~"),
        }
    }
}

const INDENT_LEN: usize = 4;

impl Stmt {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, ind_lvl: usize, pretty: bool) -> fmt::Result {
        let indent_str = if pretty {
            " ".repeat(INDENT_LEN)
        } else {
            "".to_string()
        };

        match self {
            Stmt::Let(id, t, e) => {
                write!(f, "let {}: {}", id, t)?;
                if let Some(expr) = e {
                    write!(f, " = {}", expr)?;
                }
                Ok(())
            }
            Stmt::Expr(e) => write!(f, "{}", e),
            Stmt::If(cond, then, els) => {
                write!(f, "if {} ", cond)?;

                then.fmt_with_indent(f, ind_lvl, pretty)?;

                if let Some(els) = els {
                    write!(f, " else ")?;
                    els.fmt_with_indent(f, ind_lvl, pretty)?;
                }
                Ok(())
            }
            Stmt::While(cond, body) => {
                write!(f, "while {} ", cond)?;
                body.fmt_with_indent(f, ind_lvl, pretty)
            }
            Stmt::For(id, start, end, body) => {
                write!(f, "for {} = {} to {} ", id, start, end)?;
                body.fmt_with_indent(f, ind_lvl, pretty)
            }
            Stmt::Return(e) => write!(f, "return {}", e),
            Stmt::Block(stmts) => {
                write!(f, "{{")?;
                if pretty {
                    writeln!(f)?;
                }
                for stmt in stmts {
                    write!(f, "{}", indent_str.repeat(ind_lvl + 1))?;
                    stmt.fmt_with_indent(f, ind_lvl + 1, pretty)?;
                    if pretty {
                        writeln!(f)?;
                    } else {
                        write!(f, ";")?;
                    }
                }
                write!(f, "{}}}", indent_str.repeat(ind_lvl))
            }
            Stmt::FnDef(id, params, ret, body) => {
                write!(f, "fn {}(", id)?;
                for (i, (id, t)) in params.iter().enumerate() {
                    write!(f, "{}: {}", id, t)?;
                    if i != params.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
                if let Some(ret) = ret {
                    write!(f, ": {}", ret)?;
                }
                write!(f, " ")?;
                body.fmt_with_indent(f, ind_lvl, pretty)?;
                
                if pretty {
                    writeln!(f)?;
                }

                Ok(())
            }
            Stmt::Empty => write!(f, ""),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0, f.alternate())
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Char => write!(f, "char"),
            Type::Str => write!(f, "str"),
            Type::Bool => write!(f, "bool"),
            Type::Array(t) => write!(f, "{}[]", t),
            Type::Error => write!(f, "err_type"),
        }
    }
}
