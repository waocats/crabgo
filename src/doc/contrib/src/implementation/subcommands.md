# New Subcommands

Crabgo is a single binary composed of a set of [`clap`] subcommands. All
subcommands live in [`src/bin/crabgo/commands`] directory.
[`src/bin/crabgo/main.rs`] is the entry point.

Each subcommand, such as [`src/bin/crabgo/commands/build.rs`], usually performs
the following:

1. Parse the CLI flags. See the [`command_prelude`] module for some helpers to make this easier.
2. Load the config files.
3. Discover and load the workspace.
4. Calls the actual implementation of the subcommand which resides in [`src/crabgo/ops`].

If the subcommand is not found in the built-in list, then Crabgo will
automatically search for a subcommand named `crabgo-{NAME}` in the users `PATH`
to execute the subcommand.


[`clap`]: https://clap.rs/
[`src/bin/crabgo/commands/build.rs`]: https://github.com/rust-lang/crabgo/tree/master/src/bin/crabgo/commands/build.rs
[`src/crabgo/ops`]: https://github.com/rust-lang/crabgo/tree/master/src/crabgo/ops
[`src/bin/crabgo/commands`]: https://github.com/rust-lang/crabgo/tree/master/src/bin/crabgo/commands
[`src/bin/crabgo/main.rs`]: https://github.com/rust-lang/crabgo/blob/master/src/bin/crabgo/main.rs
[`command_prelude`]: https://github.com/rust-lang/crabgo/blob/master/src/crabgo/util/command_prelude.rs
