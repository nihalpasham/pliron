# Kaleidoscope Tutorial (pliron)

This directory contains an LLVM-tutorial-style onboarding track implemented with pliron.

The tutorial is organized as progressive chapters in `kaleidoscope/src/` and a single
runnable compiler example in `examples/kaleidoscope/`.

## Local build

1. Install mdBook:
   cargo install mdbook
2. Build the book:
   mdbook build kaleidoscope
3. Serve with live reload:
   mdbook serve kaleidoscope

The tutorial prioritizes a beginner path with runnable, test-backed chapters.

## Run chapter tests

List available tests:

```sh
cargo test --example kaleidoscope -- --list
```

Run all chapter tests:

```sh
cargo test --example kaleidoscope -- --show-output
```

Run the example CLI end-to-end:

```sh
cargo run --example kaleidoscope -- --input examples/kaleidoscope/fibonacci.kal --fn main --arg 5
```
