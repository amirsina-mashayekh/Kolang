#[derive(Debug, PartialEq, Eq)]
/// The `Token` struct stores and represents a token of Kolang code.
pub struct Token {
    /// Line of code where this token starts.
    pub line: usize,
    /// Column of code where this token starts.
    pub column: usize,
    /// Type of this token.
    pub token_type: TokenType,
}

impl Token {
    /// Creates a new `Token` with provided type in specified position.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::token::{Token, TokenType};
    ///
    /// let tok = Token::new(1, 1, TokenType::KwFn);
    /// ```
    pub fn new(line: usize, column: usize, token_type: TokenType) -> Self {
        Self {
            line,
            column,
            token_type,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, Ln: {}, Col: {}",
            self.token_type, self.line, self.column
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
/// A set of Kolang token types. Some types also store the value of token as string.
pub enum TokenType {
    /// Identifier: variable name, function name
    Iden(String),
    /// Decimal integer literal: `123`, `0`
    LiteralIntDec(String),
    /// Binary integer literal: `0b1101`, `0B1`
    LiteralIntBin(String),
    /// Octal integer literal: `0o7231`, `0O44`
    LiteralIntOct(String),
    /// Hexadecimal integer literal: `0xff`, `0XA1`
    LiteralIntHex(String),
    /// Character literal: `'a'`, `'\0'`
    LiteralChar(String),
    /// Floating-point literal: `9.1`, `2e3`, `.05`
    LiteralFloat(String),
    /// String literal: `"Hello\tworld!"`
    LiteralStr(String),
    /// Left parenthesis
    LPar,
    /// Right parenthesis
    RPar,
    /// Left bracket                           
    LBracket,
    /// Right bracket                          
    RBracket,
    /// Left curly bracket                     
    LBrace,
    /// Right curly bracket                    
    RBrace,
    /// Less than                              
    LT,
    /// Greater than                           
    GT,
    /// Less than or equal                     
    LEq,
    /// Greater than or equal                  
    GEq,
    /// Equals                                 
    Eq,
    /// Not equal                              
    NEq,
    /// Assignment                             
    Assign,
    /// Plus sign                              
    Plus,
    /// Minus sign                             
    Minus,
    /// Asterisk                               
    Asterisk,
    /// Slash                                  
    Slash,
    /// Percent                                
    Percent,
    /// Pipe (bitwise or)                      
    Pipe,
    /// Ampersand (bitwise and)                
    Amp,
    /// Tilde (bitwise not)                    
    Tilde,
    /// Statement terminator                   
    Semicolon,
    /// Colon                                  
    Colon,
    /// Comma                                  
    Comma,
    /// Period                                 
    Period,
    /// `// Line comment`                          
    LC(String),
    /// `/*Block comment*/` (not nested)             
    BC(String),
    /// `for` keyword (loop)                   
    KwFor,
    /// `to` keyword (loop range)              
    KwTo,
    /// `while` keyword (loop)                 
    KwWhile,
    /// `if` keyword (conditional)             
    KwIf,
    /// `else` keyword (conditional)           
    KwElse,
    /// `true` keyword (boolean)               
    KwTrue,
    /// `false` keyword (boolean)              
    KwFalse,
    /// `or` keyword (logical)                 
    KwOr,
    /// `and` keyword (logical)                
    KwAnd,
    /// `not` keyword (logical)                
    KwNot,
    /// `let` keyword (variable def.)          
    KwLet,
    /// `fn` keyword (function def.)           
    KwFn,
    /// `return` keyword (function result)           
    KwReturn,
    /// `int` keyword (integer type)           
    KwInt,
    /// `char` keyword (character type)        
    KwChar,
    /// `bool` keyword (boolean type)          
    KwBool,
    /// `float` keyword (floating-point type)  
    KwFloat,
    /// `str` keyword (string type)            
    KwStr,
    /// Invalid (unmatched) token
    Invalid,
    /// End of file
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Iden(id) => write!(f, "{id}"),
            TokenType::LiteralIntDec(n)
            | TokenType::LiteralIntBin(n)
            | TokenType::LiteralIntOct(n)
            | TokenType::LiteralIntHex(n) => write!(f, "{n}"),
            TokenType::LiteralChar(c) => write!(f, "{c}"),
            TokenType::LiteralFloat(num) => write!(f, "{num}"),
            TokenType::LiteralStr(s) => write!(f, "{s}"),
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
            TokenType::LC(_) | TokenType::BC(_) => f.write_str("comment"),
            TokenType::KwFor => f.write_str("for"),
            TokenType::KwTo => f.write_str("to"),
            TokenType::KwWhile => f.write_str("while"),
            TokenType::KwIf => f.write_str("if"),
            TokenType::KwElse => f.write_str("else"),
            TokenType::KwTrue => f.write_str("true"),
            TokenType::KwFalse => f.write_str("false"),
            TokenType::KwOr => f.write_str("or"),
            TokenType::KwAnd => f.write_str("and"),
            TokenType::KwNot => f.write_str("not"),
            TokenType::KwLet => f.write_str("let"),
            TokenType::KwFn => f.write_str("fn"),
            TokenType::KwReturn => f.write_str("return"),
            TokenType::KwInt => f.write_str("int"),
            TokenType::KwChar => f.write_str("char"),
            TokenType::KwBool => f.write_str("bool"),
            TokenType::KwFloat => f.write_str("float"),
            TokenType::KwStr => f.write_str("str"),
            TokenType::Invalid => f.write_str("invalid"),
            TokenType::EOF => f.write_str("EOF"),
        }
    }
}
