# Tiny Calc

A tiny calculator for decimal, hex, and binary arithmetic.

## Installation

This project currently isn't published and requires being built from source. As such, you'll also need [the Rust toolchain](https://www.rust-lang.org/tools/install) installed locally.

The easiest way to install `tiny-calc` is via `cargo install` with the Github repository as the source:
```
$ cargo install --git https://github.com/CDThomas/tiny-calc.git
```

You'll also want to make sure that `~/.cargo/bin` is available in your `PATH` in order for your shell to be able to find the `tiny-calc` executable.

## Usage

Start the [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) by running:
```
tiny-calc
```

From there, you can perform arithmetic operations:
```
2 ^ 4 / 4 * 5 + 6 - 1
  = 25
```

Hex and binary literals are also supported:
```
0xFF - 0b11
 = 252
```

You can optionally specify an output format by appending `#x` (for hex) or `#b` (for binary) to the end of an expression:
```
0xFF - 0b11 #x
 = 0xFC
0xFF - 0b11 #b
 = 0b11111100
```

## Limitations

Current limitations:
* All operations must be on integers
* Division only returns the integer part and remainders are discarded
* Exponents must be non-negative

These are all fixable, but may or may not be changed.


