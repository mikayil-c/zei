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
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    if !lexer.diagnostics.is_empty() || !parser.diagnostics.is_empty() {
        // İki array'i birleştirirken değerlerini reference (&) alıp klonlayarak (ya da to_vec) ilerleyin
        let mut all_diags = lexer.diagnostics.clone();
        all_diags.extend(parser.diagnostics.clone());
        report_diagnostics(&source, &all_diags);
        return Err(65);
    }

    // if !lexer.diagnostics.is_empty() || !parser.diagnostics.is_empty() {
    //     report_diagnostics(&source, &[lexer.diagnostics, parser.diagnostics].concat());
    //     return Err(65);
    // }

    interpreter.interpret(&statements, &source);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_source(source: &str) -> Result<(), i32> {
        let mut interpreter = Interpreter::new();
        run(source, &mut interpreter)
    }

    #[test]
    fn functions_can_be_defined_and_called() {
        let source = r#"
            fun add(a, b) {
                return a + b;
            }

            print add(2, 3);
        "#;

        assert!(run_source(source).is_ok());
    }

    #[test]
    fn classes_can_be_instantiated_and_methods_called() {
        let source = r#"
            class Doughnut {
                cook() {
                    print "cooked";
                }
            }

            var doughnut = Doughnut();
            doughnut.cook();
        "#;

        assert!(run_source(source).is_ok());
    }

    #[test]
    fn instance_fields_can_be_assigned_and_read() {
        let source = r#"
            class Counter {}
            var counter = Counter();
            counter.value = 3;
            print counter.value;
        "#;

        assert!(run_source(source).is_ok());
    }
}
