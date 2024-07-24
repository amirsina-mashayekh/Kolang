pub enum TokenType {
    Iden(String),
    LiteralIntDec(i32),
    LiteralIntBin(i32),
    LiteralIntOct(i32),
    LiteralIntHex(i32),
    LiteralChar(char),
    LiteralFloat(f32),
    LiteralStr(String),
    LPar,
    RPar,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LT,
    GT,
    LEq,
    GEq,
    Eq,
    NEq,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Pipe,
    Amp,
    Tilde,
    Semicolon,
    Colon,
    Comma,
    Period,
    LC(String),
    BC(String),
    KwFor,
    KwTo,
    KwWhile,
    KwIf,
    KwElse,
    KwTrue,
    KwFalse,
    KwOr,
    KwAnd,
    KwNot,
    KwLet,
    KwFn,
    KwInt,
    KwChar,
    KwBool,
    KwFloat,
    KwStr,
    Invalid,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Iden(id) => write!(f, "id:{id}"),
            TokenType::LiteralIntDec(n) => write!(f, "{n}"),
            TokenType::LiteralIntBin(n) => write!(f, "{:#b}", n),
            TokenType::LiteralIntOct(n) => write!(f, "{:#o}", n),
            TokenType::LiteralIntHex(n) => write!(f, "{:#x}", n),
            TokenType::LiteralChar(c) => write!(f, "'{c}'"),
            TokenType::LiteralFloat(num) => write!(f, "{:e}", num),
            TokenType::LiteralStr(s) => write!(f, "\"{s}\""),
            TokenType::LPar => f.write_str("("),
            TokenType::RPar => f.write_str(")"),
            TokenType::LBracket => f.write_str("["),
            TokenType::RBracket => f.write_str("]"),
            TokenType::LBrace => f.write_str("{"),
            TokenType::RBrace => f.write_str("}"),
            TokenType::LT => f.write_str("<"),
            TokenType::GT => f.write_str(">"),
            TokenType::LEq => f.write_str("<="),
            TokenType::GEq => f.write_str(">="),
            TokenType::Eq => f.write_str("=="),
            TokenType::NEq => f.write_str("!="),
            TokenType::Assign => f.write_str("="),
            TokenType::Plus => f.write_str("+"),
            TokenType::Minus => f.write_str("-"),
            TokenType::Asterisk => f.write_str("*"),
            TokenType::Slash => f.write_str("/"),
            TokenType::Percent => f.write_str("%"),
            TokenType::Pipe => f.write_str("|"),
            TokenType::Amp => f.write_str("&"),
            TokenType::Tilde => f.write_str("~"),
            TokenType::Semicolon => f.write_str(";"),
            TokenType::Colon => f.write_str(":"),
            TokenType::Comma => f.write_str(","),
            TokenType::Period => f.write_str("."),
            TokenType::LC(_) => f.write_str("comment"),
            TokenType::BC(_) => f.write_str("comment"),
            TokenType::KwFor => f.write_str("kw:for"),
            TokenType::KwTo => f.write_str("kw:to"),
            TokenType::KwWhile => f.write_str("kw:while"),
            TokenType::KwIf => f.write_str("kw:if"),
            TokenType::KwElse => f.write_str("kw:else"),
            TokenType::KwTrue => f.write_str("kw:true"),
            TokenType::KwFalse => f.write_str("kw:false"),
            TokenType::KwOr => f.write_str("kw:or"),
            TokenType::KwAnd => f.write_str("kw:and"),
            TokenType::KwNot => f.write_str("kw:not"),
            TokenType::KwLet => f.write_str("kw:let"),
            TokenType::KwFn => f.write_str("kw:fn"),
            TokenType::KwInt => f.write_str("kw:int"),
            TokenType::KwChar => f.write_str("kw:char"),
            TokenType::KwBool => f.write_str("kw:bool"),
            TokenType::KwFloat => f.write_str("kw:float"),
            TokenType::KwStr => f.write_str("kw:str"),
            TokenType::Invalid => f.write_str("Invalid"),
            TokenType::EOF => f.write_str("EOF"),
        }
    }
}