pub enum Expr {
    LiteralInt(i64),
    LiteralStr(String),
    LiteralChar(char),
    LiteralFloat(f64),
    LiteralArray(Vec<Expr>),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>),
    Identifier(String),
    Call(String, Vec<Expr>),
    ArrayExpr(String, Box<Expr>),
    Assign(String, Box<Expr>),
    Error,
}

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

pub enum UnOp {
    Neg,
    LogNot,
    BitNot,
}

pub enum Stmt {
    Let(String, Expr),
    Expr(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(String, Expr, Expr, Box<Stmt>),
    Return(Expr),
    Block(Vec<Stmt>),
    FnDef(String, Vec<String>, Box<Stmt>),
}