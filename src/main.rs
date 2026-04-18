use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod lexer;
mod parser;
use crate::lexer::lexer::Lexer;
use crate::lexer::report::report_diagnostics;
use crate::parser::expr::Expr;
use crate::parser::parser::Parser;

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => parenthesize(&operator.lexeme, &[left, right]),
        Expr::Grouping { expression } => parenthesize("group", &[expression]),
        Expr::Literal { value } => {
            format!("{:?}", value)
        }
        Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &[right]),
    }
}

fn parenthesize(name: &str, exprs: &[&Box<Expr>]) -> String {
    let mut builder = String::new();

    builder.push('(');
    builder.push_str(name);
    for expr in exprs {
        builder.push(' ');
        // Özyinelemeli (recursive) olarak alt dalları yazdır
        builder.push_str(&print_ast(expr));
    }
    builder.push(')');

    builder
}

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

pub fn print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                operator.lexeme,
                print_expr(left),
                print_expr(right)
            )
        }
        Expr::Grouping { expression } => {
            format!("(group {})", print_expr(expression))
        }
        Expr::Literal { value } => {
            format!("{:?}", value)
        }
        Expr::Unary { operator, right } => {
            format!("({} {})", operator.lexeme, print_expr(right))
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

    let mut parser = Parser::new(&tokens);
    let expression = parser.parse();

    if !lexer.diagnostics.is_empty() || !parser.diagnostics.is_empty() {
        report_diagnostics(&source, &[lexer.diagnostics, parser.diagnostics].concat());
    } else {
        if let Some(expr) = expression {
            println!("{}", print_ast(&expr));
        }
    }

    // if !lexer.diagnostics.is_empty() {
    //     report_diagnostics(&source, &lexer.diagnostics);
    // } else {
    //     for token in tokens {
    //         println!("{:?}", token);
    //     }
    // }
}
