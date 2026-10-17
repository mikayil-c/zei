Hello, meet Zei. A high-level, dynamically typed, interpreted programming language developed using Rust.

Current version: `0.1.0-beta.1`

<p align="center">
  <img src="images/zei_logo_half.png"/>
</p>

## Documentation

For a more detailed overview of the language and its current implementation, see [docs/language.md](docs/language.md).

Additional notes:

- [docs/usage.md](docs/usage.md) for running the interpreter and REPL.
- [docs/roadmap.md](docs/roadmap.md) for the current limitations and likely next documentation topics.

## Quick Start

Run a Zei script:

```powershell
cargo run -- test/file.zei
```

Start the interactive REPL:

```powershell
cargo run
```

The goal of this project is to create a simple, high-level programming language by using Rust’s powerful features. As a computer engineering student, I am working on this project on my own. I am using the book \*Crafting Interpreters\* by Robert Nystorm as a reference.

## Current Features

- numbers, strings, booleans, and `nil`,
- variables, assignment, blocks, and enclosing scopes,
- `if`, `while`, and `for` statements with `break` and `continue`,
- functions, returns, and closures,
- classes, instances, methods, `this`, and instance fields,
- arithmetic, comparison, equality, and logical operators,
- native `clock()` function,
- lexer, parser, resolver, and runtime diagnostics with source spans.

## Current Limitations

- Arrays and object literals are not implemented.
- Inheritance and `super` are not implemented.
- String escape sequences are not implemented.
