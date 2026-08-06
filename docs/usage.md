# ZEI Usage

## Running The Interpreter

The binary supports two modes:

- `zei [script]` to run a source file.
- `zei` with no arguments to open the interactive REPL.

If more than one argument is passed, the program prints a short usage message and exits.

## REPL Behavior

In the REPL, each non-empty line is sent through the lexer, parser, and interpreter using the same runtime instance. That means variables and declarations can persist across lines during the session.

## Example

```zei
var message = "hello";
print message;
print clock();
```

## Built-in Runtime Features

The runtime currently exposes:

- `clock()` as a native function that returns the current Unix timestamp in seconds.

## Notes

- Syntax and language details are documented in [language.md](language.md).
- Planned improvements and limitations are listed in [roadmap.md](roadmap.md).
