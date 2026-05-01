use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

mod environment;
mod error;
mod interpreter;
mod lexer;
mod parser;
use crate::error::report::report_diagnostics;
use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

// pub fn print_ast(expr: &Expr) -> String {
//     match expr {
//         Expr::Binary {
//             left,
//             operator,
//             right,
//         } => parenthesize(&operator.lexeme, &[left, right]),
//         Expr::Grouping { expression } => parenthesize("group", &[expression]),
//         Expr::Literal { value } => {
//             format!("{:?}", value)
//         }
//         Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &[right]),
//     }
// }

// fn parenthesize(name: &str, exprs: &[&Box<Expr>]) -> String {
//     let mut builder = String::new();

//     builder.push('(');
//     builder.push_str(name);
//     for expr in exprs {
//         builder.push(' ');
//         // Print child nodes recursively so nested expressions stay readable.
//         builder.push_str(&print_ast(expr));
//     }
//     builder.push(')');

//     builder
// }

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

// pub fn print_expr(expr: &Expr) -> String {
//     match expr {
//         Expr::Binary {
//             left,
//             operator,
//             right,
//         } => {
//             format!(
//                 "({} {} {})",
//                 operator.lexeme,
//                 print_expr(left),
//                 print_expr(right)
//             )
//         }
//         Expr::Grouping { expression } => {
//             format!("(group {})", print_expr(expression))
//         }
//         Expr::Literal { value } => {
//             format!("{:?}", value)
//         }
//         Expr::Unary { operator, right } => {
//             format!("({} {})", operator.lexeme, print_expr(right))
//         }
//     }
// }

/// Run a source file through the lexer, parser, and interpreter.
fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;

    let mut interpreter = Interpreter::new();

    // Propagate the process exit code returned by the language pipeline.
    if let Err(exit_code) = run(&contents, &mut interpreter) {
        exit(exit_code);
    }

    Ok(())
}

/// Run the interactive REPL loop.
fn run_prompt() -> io::Result<()> {
    let mut buffer = String::new();

    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush()?;
        buffer.clear();

        let bytes_read = io::stdin().read_line(&mut buffer)?;

        if bytes_read == 0 {
            println!("Exiting...");
            break;
        }

        if buffer.trim().is_empty() {
            continue;
        }

        // Keep the REPL alive after an error so the next line can still be entered.
        let _ = run(buffer.trim(), &mut interpreter);
    }

    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), i32> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    let mut parser = Parser::new(&tokens);
    let statements = parser.parse();

    if !lexer.diagnostics.is_empty() || !parser.diagnostics.is_empty() {
        report_diagnostics(&source, &[lexer.diagnostics, parser.diagnostics].concat());
        return Err(65);
    }

    interpreter.interpret(&statements, &source);

    // if let statements = &expression {

    // match interpreter.evaluate() {
    //     Ok(value) => {
    //         println!("{}", value);
    //     }
    //     Err(diagnostic) => {
    //         report_diagnostics(source, &[diagnostic]);
    //         return Err(70);
    //     }
    // }
    // }
    Ok(())
}
