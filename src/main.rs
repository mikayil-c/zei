use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod lexer;
use lexer::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: zei [script]");
        exit(64);
    } else if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Error reading file: {}", e);
            exit(74);
        };
    } else {
        if let Err(e) = run_prompt() {
            eprintln!("REPL Error: {}", e);
            exit(1);
        }
    }
}

/// For file instructions.
fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(&contents);
    Ok(())
}

/// For line instructions.
fn run_prompt() -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        print!("> ");
        // Do not wait for user interactive.
        io::stdout().flush()?;
        // Clear buffer data for memory efficiency.
        buffer.clear();

        let bytes_read = io::stdin().read_line(&mut buffer)?;

        if bytes_read == 0 {
            println!("Exiting...");
            break;
        }

        if buffer.trim().is_empty() {
            continue;
        }

        run(buffer.trim());
    }

    Ok(())
}

fn run(source: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    for token in tokens {
        println!("{:?}", token);
    }
}
