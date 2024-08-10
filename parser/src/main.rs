use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use lexer::Lexer;
use parser::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("⛏️  Kolang parser v{}\n", VERSION);
        println!("Code file path (relative or absolute):");
        print!(">>> ");
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().lock().read_line(&mut buf)?;
        buf.trim_end().to_string()
    };

    let f = File::open(path)?;
    let l = Lexer::new(f);
    let mut p = Parser::new(l);

    p.parse()?;

    Ok(())
}
