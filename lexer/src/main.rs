use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use lexer::{token::TokenType, Lexer};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("⛏️  Kolang lexer v{}\n", VERSION);
        println!("Code file path (relative or absolute):");
        print!(">>> ");
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().lock().read_line(&mut buf)?;
        buf.trim_end().to_string()
    };

    let f = File::open(path)?;
    let mut l = Lexer::new(f);

    while let Ok(tok) = l.next() {
        println!("{}", tok);
        if tok.token_type == TokenType::EOF {
            break;
        }
    }

    Ok(())
}
