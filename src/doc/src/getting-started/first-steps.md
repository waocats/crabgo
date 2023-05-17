## First Steps with Crabgo

This section provides a quick sense for the `crabgo` command line tool. We
demonstrate its ability to generate a new [***package***][def-package] for us,
its ability to compile the [***crate***][def-crate] within the package, and
its ability to run the resulting program.

To start a new package with Crabgo, use `crabgo new`:

```console
$ crabgo new hello_world
```

Crabgo defaults to `--bin` to make a binary program. To make a library, we
would pass `--lib`, instead.

Let’s check out what Crabgo has generated for us:

```console
$ cd hello_world
$ tree .
.
├── Crabgo.toml
└── src
    └── main.rs

1 directory, 2 files
```

This is all we need to get started. First, let’s check out `Crabgo.toml`:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

This is called a [***manifest***][def-manifest], and it contains all of the
metadata that Crabgo needs to compile your package.

Here’s what’s in `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Crabgo generated a “hello world” program for us, otherwise known as a
[***binary crate***][def-crate]. Let’s compile it:

```console
$ crabgo build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

And then run it:

```console
$ ./target/debug/hello_world
Hello, world!
```

We can also use `crabgo run` to compile and then run it, all in one step:

```console
$ crabgo run
     Fresh hello_world v0.1.0 (file:///path/to/package/hello_world)
   Running `target/hello_world`
Hello, world!
```

### Going further

For more details on using Crabgo, check out the [Crabgo Guide](../guide/index.md)

[def-crate]:     ../appendix/glossary.md#crate     '"crate" (glossary entry)'
[def-manifest]:  ../appendix/glossary.md#manifest  '"manifest" (glossary entry)'
[def-package]:   ../appendix/glossary.md#package   '"package" (glossary entry)'
