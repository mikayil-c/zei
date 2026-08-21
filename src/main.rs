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
use crate::interpreter::resolver::Resolver;
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

    let mut resolver = Resolver::new();
    if let Err(diagnostic) = resolver.resolve(&statements) {
        report_diagnostics(source, &[diagnostic]);
        return Err(65);
    }

    // if !lexer.diagnostics.is_empty() || !parser.diagnostics.is_empty() {
    //     report_diagnostics(&source, &[lexer.diagnostics, parser.diagnostics].concat());
    //     return Err(65);
    // }

    if interpreter.interpret(&statements, source) {
        Ok(())
    } else {
        Err(70)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::models::{LiteralType, TokenType};
    use crate::parser::models::{Expr, Stmt};

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

    #[test]
    fn local_variable_cannot_be_read_in_its_initializer() {
        let source = r#"
            {
                var value = value;
            }
        "#;

        assert!(run_source(source).is_err());
    }

    #[test]
    fn runtime_errors_fail_the_run() {
        assert!(run_source("print missing;").is_err());
    }

    #[test]
    fn return_is_rejected_outside_a_function() {
        assert!(run_source("return 1;").is_err());
    }

    #[test]
    fn this_is_rejected_outside_a_class() {
        assert!(run_source("print this;").is_err());
    }

    #[test]
    fn closures_can_capture_and_update_variables() {
        let source = r#"
            fun makeCounter() {
                var count = 0;
                fun increment() {
                    count = count + 1;
                }
                return increment;
            }

            var counter = makeCounter();
            counter();
            counter();
        "#;

        assert!(run_source(source).is_ok());
    }

    #[test]
    fn class_initializers_can_set_instance_fields() {
        let source = r#"
            class Counter {
                init(value) {
                    this.value = value;
                }
            }

            var counter = Counter(7);
            print counter.value;
        "#;

        assert!(run_source(source).is_ok());
    }

    #[test]
    fn call_with_wrong_arity_fails_the_run() {
        let source = r#"
            fun identity(value) {
                return value;
            }

            identity();
        "#;

        assert!(run_source(source).is_err());
    }

    #[test]
    fn property_assignment_requires_an_instance() {
        assert!(run_source("var value = 1; value.field = 2;").is_err());
    }

    #[test]
    fn lexer_tokenizes_literals_keywords_and_comments() {
        let mut lexer = Lexer::new("var answer = 42.5; // ignored\nprint \"ok\";".to_string());
        let tokens = lexer.lex();

        assert!(lexer.diagnostics.is_empty());
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].literal, LiteralType::Number(42.5));
        assert_eq!(tokens[5].token_type, TokenType::Print);
        assert_eq!(tokens[6].literal, LiteralType::String("ok".to_string()));
        assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn lexer_reports_unterminated_strings() {
        let mut lexer = Lexer::new("print \"unfinished;".to_string());
        let tokens = lexer.lex();

        assert!(
            tokens
                .iter()
                .any(|token| token.token_type == TokenType::Eof)
        );
        assert_eq!(lexer.diagnostics.len(), 1);
    }

    #[test]
    fn lexer_tracks_columns_after_multiline_strings() {
        let mut lexer = Lexer::new("\"first\nsecond\"; @".to_string());
        lexer.lex();

        assert_eq!(lexer.diagnostics.len(), 1);
        assert_eq!(lexer.diagnostics[0].span.line, 2);
        assert_eq!(lexer.diagnostics[0].span.column, 9);
    }

    #[test]
    fn parser_builds_instance_field_assignment() {
        let mut lexer = Lexer::new("var object; object.field = 1;".to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(lexer.diagnostics.is_empty());
        assert!(parser.diagnostics.is_empty());
        assert!(matches!(
            &statements[1],
            Stmt::Expression {
                expression: Expr::Set { .. }
            }
        ));
    }

    #[test]
    fn parser_recovers_after_a_malformed_statement() {
        let mut lexer = Lexer::new("var = 1; print 2;".to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert_eq!(parser.diagnostics.len(), 1);
        assert_eq!(statements.len(), 1);
        assert!(matches!(statements[0], Stmt::Print { .. }));
    }

    #[test]
    fn while_loop_updates_state() {
        let source = r#"
            var value = 0;
            while (value < 3) {
                value = value + 1;
            }
            print value;
        "#;

        assert!(run_source(source).is_ok());
    }

    #[test]
    fn for_loop_executes_initializer_condition_and_increment() {
        let source = r#"
            var total = 0;
            for (var index = 0; index < 3; index = index + 1) {
                total = total + index;
            }
            print total;
        "#;

        assert!(run_source(source).is_ok());
    }
}
