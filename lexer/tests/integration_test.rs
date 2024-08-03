const SOUCE_CODE: &str = "fn main() {
    let a: int = -25;
    let b = 3.1e1;
    let c: float;
    c = a + b * b/ a%b;

    let d = [1,2,3];

    let cond: bool;
    cond = 0x1fA | 0XAA + ~0B1001 & 0b1011 + a < b;
    if cond == true or 0o5 >= 0O5 {
\tprint('t');
    } else{print(\"hello!\\nworld!\")}
    
    // Comment
    while false != true or 3 >4 and not 5 <= 5.0 {
        let s:str = \"multiline
        string\";
        let ch='\\0';
    }

    /*
     * stylish
     * multiline 
     * comment
     */
    for i=0to 50
        i.something();
}";

use lexer::token::Token as TK;
use lexer::token::TokenType::*;
use lexer::Lexer;

#[test]
fn lexer_test() -> std::io::Result<()> {
    let stream = SOUCE_CODE.as_bytes();
    let mut l = Lexer::new(stream);

    assert_eq!(l.next()?, TK::new(1, 1, KwFn));
    assert_eq!(l.next()?, TK::new(1, 4, Iden("main".into())));
    assert_eq!(l.next()?, TK::new(1, 8, LPar));
    assert_eq!(l.next()?, TK::new(1, 9, RPar));
    assert_eq!(l.next()?, TK::new(1, 11, LBrace));

    assert_eq!(l.next()?, TK::new(2, 5, KwLet));
    assert_eq!(l.next()?, TK::new(2, 9, Iden("a".into())));
    assert_eq!(l.next()?, TK::new(2, 10, Colon));
    assert_eq!(l.next()?, TK::new(2, 12, KwInt));
    assert_eq!(l.next()?, TK::new(2, 16, Assign));
    assert_eq!(l.next()?, TK::new(2, 18, Minus));
    assert_eq!(l.next()?, TK::new(2, 19, LiteralIntDec("25".into())));
    assert_eq!(l.next()?, TK::new(2, 21, Semicolon));

    l.next()?; // let
    l.next()?; // b
    l.next()?; // =
    assert_eq!(l.next()?, TK::new(3, 13, LiteralFloat("3.1e1".into())));
    l.next()?; // ;

    l.next()?; // let
    l.next()?; // c
    l.next()?; // :
    assert_eq!(l.next()?, TK::new(4, 12, KwFloat));
    l.next()?; // ;

    assert_eq!(l.next()?, TK::new(5, 5, Iden("c".into())));
    assert_eq!(l.next()?, TK::new(5, 7, Assign));
    assert_eq!(l.next()?, TK::new(5, 9, Iden("a".into())));
    assert_eq!(l.next()?, TK::new(5, 11, Plus));
    assert_eq!(l.next()?, TK::new(5, 13, Iden("b".into())));
    assert_eq!(l.next()?, TK::new(5, 15, Asterisk));
    assert_eq!(l.next()?, TK::new(5, 17, Iden("b".into())));
    assert_eq!(l.next()?, TK::new(5, 18, Slash));
    assert_eq!(l.next()?, TK::new(5, 20, Iden("a".into())));
    assert_eq!(l.next()?, TK::new(5, 21, Percent));
    assert_eq!(l.next()?, TK::new(5, 22, Iden("b".into())));
    assert_eq!(l.next()?, TK::new(5, 23, Semicolon));

    l.next()?; // let
    l.next()?; // d
    l.next()?; // =
    assert_eq!(l.next()?, TK::new(7, 13, LBracket));
    l.next()?; // 1
    assert_eq!(l.next()?, TK::new(7, 15, Comma));
    l.next()?; // 2
    l.next()?; // ,
    l.next()?; // 3
    assert_eq!(l.next()?, TK::new(7, 19, RBracket));
    l.next()?; // ;
    
    l.next()?; // let
    l.next()?; // cond
    l.next()?; // :
    assert_eq!(l.next()?, TK::new(9, 15, KwBool));
    l.next()?; // ;
    
    l.next()?; // cond
    l.next()?; // =
    assert_eq!(l.next()?, TK::new(10, 12, LiteralIntHex("0x1fA".into())));
    assert_eq!(l.next()?, TK::new(10, 18, Pipe));
    assert_eq!(l.next()?, TK::new(10, 20, LiteralIntHex("0XAA".into())));
    l.next()?; // +
    assert_eq!(l.next()?, TK::new(10, 27, Tilde));
    assert_eq!(l.next()?, TK::new(10, 28, LiteralIntBin("0B1001".into())));
    assert_eq!(l.next()?, TK::new(10, 35, Amp));
    assert_eq!(l.next()?, TK::new(10, 37, LiteralIntBin("0b1011".into())));
    l.next()?; // +
    l.next()?; // a
    assert_eq!(l.next()?, TK::new(10, 48, LT));
    l.next()?; // b
    l.next()?; // ;
    
    assert_eq!(l.next()?, TK::new(11, 5, KwIf));
    l.next()?; // cond
    assert_eq!(l.next()?, TK::new(11, 13, Eq));
    assert_eq!(l.next()?, TK::new(11, 16, KwTrue));
    assert_eq!(l.next()?, TK::new(11, 21, KwOr));
    assert_eq!(l.next()?, TK::new(11, 24, LiteralIntOct("0o5".into())));
    assert_eq!(l.next()?, TK::new(11, 28, GEq));
    assert_eq!(l.next()?, TK::new(11, 31, LiteralIntOct("0O5".into())));
    l.next()?; // {
    
    assert_eq!(l.next()?, TK::new(12, 2, Iden("print".into())));
    l.next()?; // (
    assert_eq!(l.next()?, TK::new(12, 8, LiteralChar("'t'".into())));
    l.next()?; // )
    l.next()?; // ;
    
    assert_eq!(l.next()?, TK::new(13, 5, RBrace));
    assert_eq!(l.next()?, TK::new(13, 7, KwElse));
    l.next()?; // {
    l.next()?; // print
    l.next()?; // (
    assert_eq!(l.next()?, TK::new(13, 18, LiteralStr("\"hello!\\nworld!\"".into())));
    l.next()?; // )
    l.next()?; // }
    
    assert_eq!(l.next()?, TK::new(15, 5, LC("// Comment".into())));

    assert_eq!(l.next()?, TK::new(16, 5, KwWhile));
    assert_eq!(l.next()?, TK::new(16, 11, KwFalse));
    assert_eq!(l.next()?, TK::new(16, 17, NEq));
    l.next()?; // true
    l.next()?; // or
    l.next()?; // 3
    assert_eq!(l.next()?, TK::new(16, 30, GT));
    l.next()?; // 4
    assert_eq!(l.next()?, TK::new(16, 33, KwAnd));
    assert_eq!(l.next()?, TK::new(16, 37, KwNot));
    l.next()?; // 5
    assert_eq!(l.next()?, TK::new(16, 43, LEq));
    l.next()?; // 5.0
    l.next()?; // {

    l.next()?; // let
    l.next()?; // s
    l.next()?; // :
    assert_eq!(l.next()?, TK::new(17, 15, KwStr));
    l.next()?; // =
    assert_eq!(l.next()?, TK::new(17, 21, LiteralStr("\"multiline\n        string\"".into())));
    l.next()?; // ;

    l.next()?; // let
    l.next()?; // ch
    l.next()?; // =
    assert_eq!(l.next()?, TK::new(19, 16, LiteralChar("'\\0'".into())));
    l.next()?; // ;
    
    l.next()?; // }
    
    assert_eq!(l.next()?, TK::new(22, 5, BC("/*\n     * stylish\n     * multiline \n     * comment\n     */".into())));
    
    assert_eq!(l.next()?, TK::new(27, 5, KwFor));
    assert_eq!(l.next()?, TK::new(27, 9, Iden("i".into())));
    assert_eq!(l.next()?, TK::new(27, 10, Assign));
    assert_eq!(l.next()?, TK::new(27, 11, LiteralIntDec("0".into())));
    assert_eq!(l.next()?, TK::new(27, 12, KwTo));
    assert_eq!(l.next()?, TK::new(27, 15, LiteralIntDec("50".into())));
    
    l.next()?; // i
    assert_eq!(l.next()?, TK::new(28, 10, Period));
    l.next()?; // something
    l.next()?; // (
    l.next()?; // )
    l.next()?; // ;

    l.next()?; // }

    assert_eq!(l.next()?, TK::new(29, 1, EOF));

    Ok(())
}
