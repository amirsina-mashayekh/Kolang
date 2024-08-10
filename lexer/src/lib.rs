#![warn(missing_docs)]

//! # Kolang lexer
//! Utilities for tokenizing Kolang code.

use std::io::{self, BufReader, Read};

use token::{Token, TokenType};

/// This module includes some utilities to store and represent Kolang tokens.
pub mod token;

/// The `Lexer<R>` struct allows you to scan Kolang code from any byte source
/// which implements [`Read`] trait (file, network, in-memory buffer, etc.)
/// and get tokens.
///
/// # Examples
///
/// ```
/// use lexer::{Lexer, token::TokenType};
///
/// let source = "fn main(): int {}".as_bytes();
/// let mut l = Lexer::new(source);
///
/// assert_eq!(l.next().unwrap().token_type, TokenType::KwFn);
/// assert_eq!(l.next().unwrap().token_type, TokenType::Iden("main".to_string()));
/// assert_eq!(l.next().unwrap().token_type, TokenType::LPar);
/// assert_eq!(l.next().unwrap().token_type, TokenType::RPar);
/// assert_eq!(l.next().unwrap().token_type, TokenType::Colon);
/// assert_eq!(l.next().unwrap().token_type, TokenType::KwInt);
/// assert_eq!(l.next().unwrap().token_type, TokenType::LBrace);
/// assert_eq!(l.next().unwrap().token_type, TokenType::RBrace);
/// assert_eq!(l.next().unwrap().token_type, TokenType::EOF);
/// ```
#[derive(Debug)]
pub struct Lexer<R: Read> {
    /// Current line of source code.
    line: usize,
    /// Current column (character in line) of source code.
    column: usize,
    /// Byte stream which provides source code.
    stream: BufReader<R>,
    /// Current character of source code.
    current: char,
}

impl<R: Read> Lexer<R> {
    /// Creates a new `Lexer<R>` with provided byte stream as the token source.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::Lexer;
    ///
    /// let source = "fn main(): int {}".as_bytes();
    /// let mut l = Lexer::new(source);
    /// ```
    pub fn new(stream: R) -> Self {
        Self {
            line: 1,
            column: 0,
            stream: BufReader::new(stream),
            current: ' ',
        }
    }

    /// Reads next token from provided byte stream, constructs and returns it.
    /// If Lexer reaches end of stream, it will return [`TokenType::EOF`] tokens
    /// until there are new bytes on the stream.
    ///
    /// # Errors
    /// May return I/O error if something goes wrong while reading bytes
    /// from source.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::{Lexer, token::TokenType};
    ///
    /// let source = "fn main".as_bytes();
    /// let mut l = Lexer::new(source);
    ///
    /// assert_eq!(l.next().unwrap().token_type, TokenType::KwFn);
    /// assert_eq!(l.next().unwrap().token_type, TokenType::Iden("main".to_string()));
    /// assert_eq!(l.next().unwrap().token_type, TokenType::EOF);
    /// ```
    pub fn next(&mut self) -> io::Result<Token> {
        self.consume_whitespace()?;

        let line = self.line;
        let column = self.column;
        let mut consumed = false;

        let tok = match self.current {
            '(' => TokenType::LPar,
            ')' => TokenType::RPar,
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '%' => TokenType::Percent,
            '|' => TokenType::Pipe,
            '&' => TokenType::Amp,
            '~' => TokenType::Tilde,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '\0' => TokenType::EOF,
            '<' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::LEq
                } else {
                    consumed = true;
                    TokenType::LT
                }
            }
            '>' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::GEq
                } else {
                    consumed = true;
                    TokenType::GT
                }
            }
            '!' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::NEq
                } else {
                    consumed = true;
                    TokenType::Invalid("!".into())
                }
            }
            '=' => {
                self.next_char()?;
                if self.current == '=' {
                    TokenType::Eq
                } else {
                    consumed = true;
                    TokenType::Assign
                }
            }
            '/' => {
                self.next_char()?;
                consumed = true;
                match self.current {
                    '/' => {
                        self.next_char()?;
                        TokenType::LC("//".to_string() + &self.match_line_comment()?)
                    }
                    '*' => {
                        self.next_char()?;
                        let mut comment = "/*".to_string();
                        comment.push_str(&self.match_block_comment()?);

                        TokenType::BC(comment)
                    }
                    _ => TokenType::Slash,
                }
            }
            '\'' => {
                consumed = true;
                let c = self.match_char()?;
                match c.as_bytes().last() {
                    Some(b'\'') => TokenType::LiteralChar(c),
                    _ => TokenType::Invalid(c),
                }
            }
            '"' => {
                consumed = true;
                let s = self.match_str()?;
                match s.as_bytes().last() {
                    Some(b'"') => TokenType::LiteralStr(s),
                    _ => TokenType::Invalid(s),
                }
            }
            '.' => {
                self.next_char()?;
                consumed = true;
                if self.current.is_digit(10) {
                    // float literal
                    let mut f = '.'.to_string();
                    f.push_str(&self.match_scientific()?);
                    TokenType::LiteralFloat(f)
                } else {
                    TokenType::Period
                }
            }
            c => {
                consumed = true;
                let mut tmp = String::new();
                if c.is_ascii_alphabetic() || c == '_' {
                    // identifier or keyword
                    tmp.push_str(&self.match_iden()?);
                    match tmp.as_str() {
                        "for" => TokenType::KwFor,
                        "to" => TokenType::KwTo,
                        "while" => TokenType::KwWhile,
                        "if" => TokenType::KwIf,
                        "else" => TokenType::KwElse,
                        "true" => TokenType::KwTrue,
                        "false" => TokenType::KwFalse,
                        "or" => TokenType::KwOr,
                        "and" => TokenType::KwAnd,
                        "not" => TokenType::KwNot,
                        "let" => TokenType::KwLet,
                        "fn" => TokenType::KwFn,
                        "return" => TokenType::KwReturn,
                        "int" => TokenType::KwInt,
                        "char" => TokenType::KwChar,
                        "bool" => TokenType::KwBool,
                        "float" => TokenType::KwFloat,
                        "str" => TokenType::KwStr,
                        _ => TokenType::Iden(tmp),
                    }
                } else if c.is_digit(10) {
                    // numeric (int or float)
                    tmp.push_str(&self.match_num(10)?);

                    if tmp.as_str() == "0" {
                        // prefixed int?
                        match self.current {
                            'b' | 'B' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_num(2)?);
                                TokenType::LiteralIntBin(tmp)
                            }
                            'o' | 'O' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_num(8)?);
                                TokenType::LiteralIntOct(tmp)
                            }
                            'x' | 'X' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_num(16)?);
                                TokenType::LiteralIntHex(tmp)
                            }
                            '.' => {
                                tmp.push(self.current);
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            'e' => {
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            _ => TokenType::LiteralIntDec(tmp),
                        }
                    } else {
                        match self.current {
                            '.' => {
                                tmp.push(self.current);
                                self.next_char()?;
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            'e' => {
                                tmp.push_str(&self.match_scientific()?);
                                TokenType::LiteralFloat(tmp)
                            }
                            _ => TokenType::LiteralIntDec(tmp),
                        }
                    }
                } else {
                    TokenType::Invalid(tmp)
                }
            }
        };

        if !consumed && !matches!(tok, TokenType::Invalid(_)) {
            self.next_char()?;
        }

        Ok(Token {
            token_type: tok,
            line,
            column,
        })
    }

    /// Discards whitespace characters until it reaches a non-whitespace character
    /// or end of stream.
    fn consume_whitespace(&mut self) -> io::Result<()> {
        while self.current.is_whitespace() {
            self.next_char()?;
        }

        Ok(())
    }

    /// Reads next identifier (or keyword) token from stream and returns
    /// it as a string. Consumes all bytes of token. May return empty string.
    fn match_iden(&mut self) -> io::Result<String> {
        let mut id = String::new();

        while self.current.is_ascii_alphanumeric() || self.current == '_' {
            id.push(self.current);
            self.next_char()?;
        }

        Ok(id)
    }

    /// Reads next integer numeric token from stream and returns
    /// it as a string. Doesn't match prefixes (0b, 0x, etc.).
    /// `base` parameter defines radix or base of number
    /// (binary, octal, decimal, hexadecimal, etc. ).
    /// Consumes all bytes of token. May return empty string.
    fn match_num(&mut self, base: u32) -> io::Result<String> {
        let mut num = String::new();

        while self.current.is_digit(base) {
            num.push(self.current);
            self.next_char()?;
        }

        Ok(num)
    }

    /// Reads next scientific number token from stream and returns
    /// it as a string. Consumes all bytes of token.
    fn match_scientific(&mut self) -> io::Result<String> {
        let mut num = self.match_num(10)?;

        if self.current == 'e' || self.current == 'E' {
            num.push(self.current);
            self.next_char()?;
            if self.current == '+' || self.current == '-' {
                num.push(self.current);
                self.next_char()?;
            }
            num.push_str(&self.match_num(10)?);
        }

        Ok(num)
    }

    /// Reads next character literal token from stream and returns
    /// it as a string. Consumes two (normal) or three (escaped) bytes,
    /// including starting and ending `'`.
    fn match_char(&mut self) -> io::Result<String> {
        let mut ch = String::from(self.current);
        self.next_char()?;

        ch.push(self.current);
        self.next_char()?;

        if ch == "'\\" && self.current != '\0' {
            ch.push(self.current);
            self.next_char()?;
        }

        if self.current == '\'' {
            ch.push(self.current);
            self.next_char()?;
        }

        Ok(ch)
    }

    /// Reads next string literal token from stream and returns
    /// it as a string. Consumes all bytes of token, including
    /// starting and ending `"`;
    fn match_str(&mut self) -> io::Result<String> {
        let mut s = String::from(self.current);
        self.next_char()?;

        let mut escape = false;

        while (self.current != '\"' && self.current != '\0') || escape {
            s.push(self.current);
            escape = self.current == '\\';
            self.next_char()?;
        }

        if self.current == '"' {
            s.push(self.current);
            self.next_char()?;
        }

        Ok(s)
    }

    /// Reads next line comment token from stream and returns
    /// it as a string. Consumes all bytes of token
    /// excluding starting `//`.
    fn match_line_comment(&mut self) -> io::Result<String> {
        let mut comment = String::new();

        while self.current != '\n' && self.current != '\0' {
            comment.push(self.current);
            self.next_char()?;
        }

        Ok(comment)
    }

    /// Reads next block comment token from stream and returns
    /// it as a string. Consumes all bytes of token excluding
    /// starting `/*` but including final `*/`.
    fn match_block_comment(&mut self) -> io::Result<String> {
        let mut comment = String::new();
        let mut asterisk = false;

        while self.current != '\0' && !(asterisk && self.current == '/') {
            comment.push(self.current);
            asterisk = self.current == '*';
            self.next_char()?;
        }
        // Push final slash
        if self.current != '\0' {
            comment.push(self.current);
            self.next_char()?;
        }

        Ok(comment)
    }

    /// Reads next byte from stream and puts it in `self.current` as `char`.
    /// If reaches end of stream, it will put `'\0'` to indicate end of file.
    /// Also updates `self.line` and `self.column` based on next character.
    ///
    /// # Errors
    /// May return I/O error if something goes wrong while reading bytes
    /// from source.
    fn next_char(&mut self) -> io::Result<()> {
        let mut buf = [0u8];
        let c = self.stream.read(&mut buf)?;

        self.current = if c == 1 {
            if self.current == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            buf[0] as char
        } else {
            '\0'
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    fn create_lexer(code: &str) -> Lexer<&[u8]> {
        let source = code.as_bytes();

        let mut l = Lexer::new(source);
        l.next_char().unwrap();

        l
    }

    #[test]
    fn next_char() -> std::io::Result<()> {
        let source_str = concat!(
            "Never gonna give you up!\n",
            "nEvEr gonna l3t you down___ \n",
            " "
        );
        let source = source_str.as_bytes();

        let mut l = Lexer::new(source);
        l.next_char()?;

        let mut str = String::new();
        let mut ln = 1;
        let mut col = 1;

        for c in source_str.to_string().chars() {
            assert_eq!(l.line, ln);
            assert_eq!(l.column, col);
            assert_eq!(c, l.current);

            if c == '\n' {
                ln += 1;
                col = 1;
            } else {
                col += 1;
            }

            str.push(l.current);
            l.next_char()?;
        }

        assert_eq!(l.current, '\0');
        assert_eq!(source_str, str);

        Ok(())
    }

    #[test]
    fn whitespace() -> std::io::Result<()> {
        let source_str = concat!(
            "   Never gonnaRun        aroud\tand desert you   !    \n",
            "Never gonna \t make you cry\n",
            "Never gonna say goodbye \n"
        );
        let mut l = create_lexer(&source_str);

        for word in source_str.to_string().split_ascii_whitespace() {
            l.consume_whitespace()?;
            let mut chs = word.chars();
            while !l.current.is_ascii_whitespace() {
                assert_eq!(chs.next().unwrap(), l.current);
                l.next_char()?;
            }
        }

        Ok(())
    }

    #[test]
    fn iden() -> std::io::Result<()> {
        let source_str = concat!(
            "never_gonna_tell_a_lie_and_hurt_you\n",
            "sPoNg3cAsE\n",
            "camelCase\n",
            "PascalCase\n",
            "lowercase\n",
            "UPPERCASE\n",
            "_startsWithUnderline\n",
            "myvar123yourvar\n",
            "_\n",
            "789ourvar456\n",           // This is not a valid identifier, however it matches. This is handled by `next()`.
            "twoVars inOneLine\n",
        );
        let mut l = create_lexer(source_str);

        let mut idens = source_str.lines();

        for _ in 0..10 {
            assert_eq!(l.match_iden()?, idens.next().unwrap());
            l.consume_whitespace()?;
        }

        let two_vars = idens.next().unwrap().split(' ');
        for iden in two_vars {
            assert_eq!(l.match_iden()?, iden);
            l.consume_whitespace()?;
        }

        Ok(())
    }

    #[test]
    fn num() -> std::io::Result<()> {
        let source_str = concat!(
            "1234567890\n",
            "00000\n",
            "aAbBcCdDeEfF\n",
            "a1b2c3d4e5f6\n",
        );
        let mut l = create_lexer(source_str);

        let mut nums = source_str.lines();

        assert_eq!(l.match_num(10)?, nums.next().unwrap());
        l.consume_whitespace()?;
        assert_eq!(l.match_num(10)?, nums.next().unwrap());
        l.consume_whitespace()?;
        assert_eq!(l.match_num(16)?, nums.next().unwrap());
        l.consume_whitespace()?;
        assert_eq!(l.match_num(10)?, "");
        assert_eq!(l.match_num(16)?, nums.next().unwrap());

        Ok(())
    }

    #[test]
    fn sci() -> std::io::Result<()> {
        let source_str = concat!(
            "2e3\n",
            "e03\n",
            "5e-10\n",
            "7e+5\n",
        );
        let mut l = create_lexer(source_str);

        let mut nums = source_str.lines();

        for _ in 0..4 {
            assert_eq!(l.match_scientific()?, nums.next().unwrap());
            l.consume_whitespace()?;
        }

        Ok(())
    }

    #[test]
    fn char() -> std::io::Result<()> {
        let source_str = concat!(
            "'m'\n",
            "' '\n",
            "'\\0'\n",
            "'\\''\n",
            "'\\n'\n",
            "'a\n",
            "'ffffffff'\n",
            "'\\abcd'\n",
        );
        let mut l = create_lexer(source_str);

        let mut chars = source_str.lines();

        for _ in 0..5 {
            assert_eq!(l.match_char()?, chars.next().unwrap());
            l.consume_whitespace()?;
        }

        assert_eq!(l.match_char()?, chars.next().unwrap()[0..2]);
        l.consume_whitespace()?;

        assert_eq!(l.match_char()?, chars.next().unwrap()[0..2]);
        l.consume_whitespace()?;
        
        while l.current != '\'' {
            l.next_char()?;
        }
        l.next_char()?;
        l.consume_whitespace()?;

        assert_eq!(l.match_char()?, chars.next().unwrap()[0..3]);

        Ok(())
    }

    #[test]
    fn string() -> std::io::Result<()> {
        let source_str = concat!(
            "\"Hellllooooo there!\"\n",
            "\"a string including \\escaped characters\"\n",
            "\"a string including \\\"double\\\" quotes\"\n",
            "\"a multiline\nstring\"\n",
            "\"endless string?",
        );
        let mut l = create_lexer(source_str);

        let mut strs = source_str.lines();

        for _ in 0..3 {
            assert_eq!(l.match_str()?, strs.next().unwrap());
            l.consume_whitespace()?;
        }

        let mut multiline = strs.next().unwrap().to_string();
        multiline.push('\n');
        multiline.push_str(strs.next().unwrap());
        assert_eq!(l.match_str()?, multiline);
        l.consume_whitespace()?;

        assert_eq!(l.match_str()?, strs.next().unwrap()[0..16]);
        l.consume_whitespace()?;

        Ok(())
    }

    #[test]
    fn line_comment() -> std::io::Result<()> {
        let source_str = concat!(
            "//simple comment\n",
            "///strange comment\n",
            "////more strange comment\n",
            "// Neat comment.\n",
            "// comment // in //comment\n",
        );
        let mut l = create_lexer(source_str);

        let mut comments = source_str.lines();

        for _ in 0..5 {
            assert_eq!(l.match_line_comment()?, comments.next().unwrap());
            l.consume_whitespace()?;
        }

        Ok(())
    }

    #[test]
    fn block_comment() -> std::io::Result<()> {
        let source_str = concat!(
            "/*comment*/\n",
            "/** strange comment */\n",
            "/* Neat comment */\n",
            "/* comment including * asterisk */\n",
            "/* not nested /* comment */\n",
            "/* a\n * multiline\n * comment */\n",
            "/* a /*nested*/ comment */\n",
            "/*endless comment?",
        );
        let mut l = create_lexer(source_str);

        let mut comments = source_str.lines();

        for _ in 0..5 {
            assert_eq!(l.match_block_comment()?, comments.next().unwrap());
            l.consume_whitespace()?;
        }

        let mut multiline = comments.next().unwrap().to_string();
        multiline.push('\n');
        multiline.push_str(comments.next().unwrap());
        multiline.push('\n');
        multiline.push_str(comments.next().unwrap());
        assert_eq!(l.match_block_comment()?, multiline);
        l.consume_whitespace()?;

        let nested = comments.next().unwrap();
        assert_eq!(l.match_block_comment()?, nested[0..15]);
        while l.current != '\n' {
            l.next_char()?;
        }
        l.consume_whitespace()?;

        assert_eq!(l.match_block_comment()?, comments.next().unwrap()[0..18]);
        l.consume_whitespace()?;

        Ok(())
    }
}
