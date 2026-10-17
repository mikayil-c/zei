# ZEI Language Notes

## Overview

The execution pipeline is:

1. Source text is read from a file or the REPL.
2. The lexer converts characters into tokens.
3. The parser turns tokens into an AST.
4. The interpreter evaluates the AST at runtime.

## Core Files

- [src/main.rs](../src/main.rs): program entry point, REPL, file execution, and pipeline wiring.
- [src/lexer/lexer.rs](../src/lexer/lexer.rs): turns characters into tokens and tracks spans.
- [src/lexer/models.rs](../src/lexer/models.rs): token types, literals, and token spans.
- [src/parser/parser.rs](../src/parser/parser.rs): recursive-descent parser with error recovery.
- [src/parser/models.rs](../src/parser/models.rs): AST node definitions for expressions and statements.
- [src/interpreter/interpreter.rs](../src/interpreter/interpreter.rs): evaluates AST nodes at runtime.
- [src/interpreter/value.rs](../src/interpreter/value.rs): runtime value representation and truthiness rules.
- [src/interpreter/native.rs](../src/interpreter/native.rs): native callables exposed to the runtime.
- [src/environment/environment.rs](../src/environment/environment.rs): scoped variable storage and lookup.
- [src/error/report.rs](../src/error/report.rs): diagnostics and source-span reporting.

## Current Language Features

- Literals: numbers, strings, booleans, and nil.
- Expressions: unary, binary, grouping, variable references, variable and instance-field assignment, logical operators, calls, and property access.
- Statements: expression statements, print statements, variable declarations, blocks, if/else, while, for, break, continue, function declarations, return statements, and class declarations.
- Runtime operations: arithmetic, comparisons, equality, string concatenation, truthiness, native `clock` calls, and instance method dispatch.

## Class and Function Structure

- Functions can now be declared with `fun name(params) { ... }` and called with normal argument lists.
- Functions capture their defining environment, so closures can read and update variables from an enclosing scope.
- Classes can be declared with `class Name { ... }`, and methods are defined inside the class body with `fun methodName(...) { ... }`.
- Instance methods receive a bound `this` value when invoked through an instance, allowing methods to access the instance state. Fields can be assigned with `instance.field = value`.

## Parser Notes

- Grammar precedence is built from equality down to primary expressions.
- Parentheses create grouping expressions.
- Error recovery uses synchronization to skip to the next likely statement boundary.
- The parser records diagnostics and keeps going so multiple syntax errors can be reported in one run.

## Interpreter Notes

- Truthy values follow Lox-style rules: nil is false, false is false, everything else is true.
- Numeric operators require numbers.
- Plus supports both number addition and string concatenation.
- Runtime errors are reported with spans so the error reporter can point back to the original source.

## Practical Reading Order

If you are changing behavior, read the code in this order:

1. `main.rs` for the end-to-end flow.
2. `lexer.rs` and `models.rs` for tokenization and source spans.
3. `parser.rs` and `models.rs` for syntax and AST shape.
4. `interpreter.rs` and `value.rs` for runtime behavior.
5. `native.rs` for built-in callables.
6. `report.rs` for diagnostic formatting.
