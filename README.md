# roll-rs

[![Build Status](https://travis-ci.org/garbados/roll-rs.svg?branch=master)](https://travis-ci.org/garbados/roll-rs)

A simple dice-rolling utility, in Rust!

## Usage

roll-rs understands a `NdM[+-]XbB?wW?` syntax, which is probably unreadable
if software hasn't already rotted your brain. Here are some examples:

- `1d8`: Roll one eight-sided die.
- `1d4+1`: Roll one four-sided die and add one.
- `2d20b1`: Roll two twenty-sided dice and take the higher roll.
- `5d6w4`: Roll five six-sided dice and take the worst four rolls.
- `1d6+1d4-2`: Roll one six-sided die and one four-sided die, then sum them and
  subtract 2.

Input that doesn't match this format evaluates to 0, and expressions separated
by whitespace are rolled separately. For example:

```
$ roll-rs hello 1d8 world 1d20
1d20 => 8
hello => 0
1d8 => 3
world => 0
```

## Installation

You can install `roll-rs` using `cargo install`. This example installs the
binary to Cargo's default location in your home directory:

```bash
$ cargo install --git https://github.com/garbados/roll-rs.git --root $HOME/.cargo
```

Then restart your terminal or run `exec bash` and you should be able to run
`roll-rs`, if `$HOME/.cargo/bin` is in your `$PATH`.

## Testing

To run the test suite, get the source code and run `cargo test`.

## License

[GPL-3.0](https://opensource.org/licenses/GPL-3.0).

And remember, comrades:
Agitate! Educate! Organize!
[The union makes us strong.](https://www.youtube.com/watch?v=RKhwitXvVj8)
