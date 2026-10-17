# ZEI Roadmap

## Current Limitations

The language already has a parser and interpreter for core statements and expressions, but several parts are still incomplete or intentionally minimal.

- `while` and `for` loops support `break` and `continue` with loop-only validation.
- Function and class declarations, calls, closures, instance methods, and instance fields are supported; inheritance and `super` are not implemented.
- The environment supports enclosing-scope traversal, while resolver output is currently used for validation rather than optimized lexical-distance lookup.
- There is no standard library beyond the small native surface.
- Arrays, objects, and richer composite types are not available yet.
- String escape sequences and more advanced numeric literal forms are not implemented.

## Good Next Documentation Additions

- A syntax reference with small examples for each statement and expression type.
- A native functions page as the runtime grows beyond `clock()`.
- A troubleshooting section for common lexer, parser, and runtime errors.
- A contributor guide describing project structure, tests, and formatting conventions.

## Suggested Product Steps

- Stabilize core runtime behavior for variables, functions, and classes.
- Expand native functions and document them together.
- Add a language reference with grammar snippets and example programs.
- Add sample programs under `test/` and document how to run them.
