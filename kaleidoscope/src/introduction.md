# Kaleidoscope Tutorial

This tutorial teaches pliron by building a small
[Kaleidoscope-like](https://llvm.org/docs/tutorial/) language
and a compiler pipeline for it.

The tutorial takes you through the following chapters:

1. Parse source text (in the Kaleidoscope language) into an AST.
2. Define a new Kaleidoscope dialect for the language.
3. Lower AST nodes into the Kaleidoscope dialect IR.
4. Lower the Kaleidoscope dialect into LLVM dialect IR.
5. Use LLVM-JIT to compile and run the resulting LLVM IR.

## Prerequisites

- Comfortable with Rust (did you read the Rust book?).
- Familiarity with basic compiler terms (AST, IR, lowering).

## How to use this tutorial

1. Read a chapter.
2. Try out the example tests.
3. Modify it (as desired) or add new tests and re-run.
4. Move to the next chapter.

Each chapter mentions runnable example tests that you can execute.
Typically, you can run them with:

```sh
cargo test --example kaleidoscope -- <test_name> --show-output
```

For a chapter-to-test map, see the [Examples and Tests Index](./examples-index.md).

## Build and view the tutorial offline (optional):

```sh
mdbook build kaleidoscope
mdbook serve kaleidoscope
```
