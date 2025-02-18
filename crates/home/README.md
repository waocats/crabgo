[![Documentation](https://docs.rs/home/badge.svg)](https://docs.rs/home)
[![crates.io](https://img.shields.io/crates/v/home.svg)](https://crates.io/crates/home)

Canonical definitions of `home_dir`, `cargo_home`, and `rustup_home`.

This provides the definition of `home_dir` used by Crabgo and rustup,
as well functions to find the correct value of `CRABGO_HOME` and
`RUSTUP_HOME`.

The definition of `home_dir` provided by the standard library is
incorrect because it considers the `HOME` environment variable on
Windows. This causes surprising situations where a Rust program will
behave differently depending on whether it is run under a Unix
emulation environment like Cygwin or MinGW. Neither Crabgo nor rustup
use the standard library's definition - they use the definition here.

This crate further provides two functions, `cargo_home` and
`rustup_home`, which are the canonical way to determine the location
that Crabgo and rustup store their data.

See [rust-lang/rust#43321].

[rust-lang/rust#43321]: https://github.com/rust-lang/rust/issues/43321

## License

MIT OR Apache-2.0
