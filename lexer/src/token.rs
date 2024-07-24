use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, Ln: {}, Col: {}", self.token_type, self.line, self.column)
    }
}