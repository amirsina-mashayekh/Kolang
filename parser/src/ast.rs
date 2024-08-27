use std::fmt;

#[derive(PartialEq)]
pub enum Expr {
    LiteralInt {
        value: i64,
        line: usize,
        column: usize,
    },
    LiteralStr {
        value: String,
        line: usize,
        column: usize,
    },
    LiteralChar {
        value: char,
        line: usize,
        column: usize,
    },
    LiteralFloat {
        value: f64,
        line: usize,
        column: usize,
    },
    LiteralBool {
        value: bool,
        line: usize,
        column: usize,
    },
    LiteralArray {
        elements: Vec<Expr>,
        line: usize,
        column: usize,
    },
    BinaryOp {
        l: Box<Expr>,
        op: BinOp,
        r: Box<Expr>,
    },
    UnaryOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    Identifier {
        id: String,
        line: usize,
        column: usize,
    },
    Call {
        id: String,
        args: Vec<Expr>,
        line: usize,
        column: usize,
    },
    ArrayExpr {
        id: String,
        index: Box<Expr>,
        line: usize,
        column: usize,
    },
    Assign {
        id: String,
        expr: Box<Expr>,
        line: usize,
        column: usize,
    },
    Error {
        line: usize,
        column: usize,
    },
}

#[derive(PartialEq, Eq)]
pub enum BinOp {
    Add { line: usize, column: usize },
    Sub { line: usize, column: usize },
    Mul { line: usize, column: usize },
    Div { line: usize, column: usize },
    Mod { line: usize, column: usize },
    LogAnd { line: usize, column: usize },
    LogOr { line: usize, column: usize },
    BitAnd { line: usize, column: usize },
    BitOr { line: usize, column: usize },
    Eq { line: usize, column: usize },
    NEq { line: usize, column: usize },
    LT { line: usize, column: usize },
    GT { line: usize, column: usize },
    LEq { line: usize, column: usize },
    GEq { line: usize, column: usize },
}

#[derive(PartialEq, Eq)]
pub enum UnOp {
    Neg { line: usize, column: usize },
    LogNot { line: usize, column: usize },
    BitNot { line: usize, column: usize },
}

#[derive(PartialEq)]
pub enum Stmt {
    Let {
        id: String,
        var_type: Type,
        expr: Option<Expr>,
        line: usize,
        column: usize,
    },
    Expr {
        expr: Expr,
    },
    If {
        cond: Expr,
        then_stmt: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
        line: usize,
        column: usize,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
        line: usize,
        column: usize,
    },
    For {
        id: String,
        start: Expr,
        end: Expr,
        body: Box<Stmt>,
        line: usize,
        column: usize,
    },
    Return {
        expr: Expr,
        line: usize,
        column: usize,
    },
    Block {
        stmts: Vec<Stmt>,
        line: usize,
        column: usize,
    },
    FnDef {
        id: String,
        params: Vec<(String, Type)>,
        return_type: Option<Type>,
        body: Box<Stmt>,
        line: usize,
        column: usize,
    },
    Empty {
        line: usize,
        column: usize,
    },
}

#[derive(PartialEq, Eq)]
pub enum Type {
    Int {
        line: usize,
        column: usize,
    },
    Float {
        line: usize,
        column: usize,
    },
    Char {
        line: usize,
        column: usize,
    },
    Str {
        line: usize,
        column: usize,
    },
    Bool {
        line: usize,
        column: usize,
    },
    Array {
        element_type: Box<Type>,
        line: usize,
        column: usize,
    },
    Error {
        line: usize,
        column: usize,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::LiteralInt { value, .. } => write!(f, "{}", value),
            Expr::LiteralStr { value, .. } => write!(f, "\"{}\"", value),
            Expr::LiteralChar { value, .. } => write!(f, "{}", value),
            Expr::LiteralFloat { value, .. } => write!(f, "{}", value),
            Expr::LiteralBool { value, .. } => write!(f, "{}", value),
            Expr::LiteralArray { elements, .. } => {
                write!(f, "[")?;
                for (i, e) in elements.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i != elements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expr::BinaryOp { l, op, r } => write!(f, "({} {} {})", l, op, r),
            Expr::UnaryOp { op, expr } => write!(f, "({} {})", op, expr),
            Expr::Identifier { id, .. } => write!(f, "{}", id),
            Expr::Call {
                id, args, ..
            } => {
                write!(f, "{}(", id)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::ArrayExpr {
                id,
                index,
                ..
            } => write!(f, "{}[{}]", id, index),
            Expr::Assign {
                id, expr, ..
            } => write!(f, "{} = {}", id, expr),
            Expr::Error { .. } => write!(f, "err_expr"),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Add { .. } => write!(f, "+"),
            BinOp::Sub { .. } => write!(f, "-"),
            BinOp::Mul { .. } => write!(f, "*"),
            BinOp::Div { .. } => write!(f, "/"),
            BinOp::Mod { .. } => write!(f, "%"),
            BinOp::LogAnd { .. } => write!(f, "and"),
            BinOp::LogOr { .. } => write!(f, "or"),
            BinOp::BitAnd { .. } => write!(f, "&"),
            BinOp::BitOr { .. } => write!(f, "|"),
            BinOp::Eq { .. } => write!(f, "=="),
            BinOp::NEq { .. } => write!(f, "!="),
            BinOp::LT { .. } => write!(f, "<"),
            BinOp::GT { .. } => write!(f, ">"),
            BinOp::LEq { .. } => write!(f, "<="),
            BinOp::GEq { .. } => write!(f, ">="),
        }
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnOp::Neg { .. } => write!(f, "-"),
            UnOp::LogNot { .. } => write!(f, "not"),
            UnOp::BitNot { .. } => write!(f, "~"),
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
            Stmt::Let {
                id,
                var_type,
                expr: init_expr,
                ..
            } => {
                write!(f, "let {}: {}", id, var_type)?;
                if let Some(expr) = init_expr {
                    write!(f, " = {}", expr)?;
                }
                Ok(())
            }
            Stmt::Expr { expr } => write!(f, "{}", expr),
            Stmt::If {
                cond,
                then_stmt,
                else_stmt,
                ..
            } => {
                write!(f, "if {} ", cond)?;
                then_stmt.fmt_with_indent(f, ind_lvl, pretty)?;
                if let Some(els) = else_stmt {
                    write!(f, " else ")?;
                    els.fmt_with_indent(f, ind_lvl, pretty)?;
                }
                Ok(())
            }
            Stmt::While {
                cond,
                body,
                ..
            } => {
                write!(f, "while {} ", cond)?;
                body.fmt_with_indent(f, ind_lvl, pretty)
            }
            Stmt::For {
                id,
                start,
                end,
                body,
                ..
            } => {
                write!(f, "for {} = {} to {} ", id, start, end)?;
                body.fmt_with_indent(f, ind_lvl, pretty)
            }
            Stmt::Return { expr, .. } => write!(f, "return {}", expr),
            Stmt::Block { stmts, .. } => {
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
            Stmt::FnDef {
                id,
                params,
                return_type,
                body,
                ..
            } => {
                write!(f, "fn {}(", id)?;
                for (i, (id, t)) in params.iter().enumerate() {
                    write!(f, "{}: {}", id, t)?;
                    if i != params.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
                if let Some(rt) = return_type {
                    write!(f, ": {}", rt)?;
                }
                write!(f, " ")?;
                body.fmt_with_indent(f, ind_lvl, pretty)?;
                if pretty {
                    writeln!(f)
                } else {
                    Ok(())
                }
            }
            Stmt::Empty { .. } => Ok(()),
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
            Type::Int { .. } => write!(f, "int"),
            Type::Float { .. } => write!(f, "float"),
            Type::Char { .. } => write!(f, "char"),
            Type::Str { .. } => write!(f, "str"),
            Type::Bool { .. } => write!(f, "bool"),
            Type::Array { element_type, .. } => write!(f, "{}[]", element_type),
            Type::Error { .. } => write!(f, "err_type"),
        }
    }
}
