# Num As Words

Rust based project, using monadic functions and pure functional concepts

( Except for one spot but don't look )

# Build

Install rust [then](https://www.rust-lang.org/) use `cargo run -- ` with a number when you are in this directory.

The program may panic if you introduce non numeric characters. The minus character may be used but only at the beginning, so `-231` is valid but `22-22` is not valid.

# Example builds

`cargo run -- 123`
`cargo run -- -869`
`cargo run -- 0`
