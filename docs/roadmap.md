# ZEI Roadmap

## Current Limitations

The language already has a parser and interpreter for core statements and expressions, but several parts are still incomplete or intentionally minimal.

- Loop syntax is present in the parser, but execution support is limited.
- Function and class support exists in the language surface, but runtime behavior is still evolving.
- The environment currently uses a simple scope model and does not yet support full enclosing-scope traversal for every use case.
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
