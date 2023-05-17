## Crabgo.toml vs Crabgo.lock

`Crabgo.toml` and `Crabgo.lock` serve two different purposes. Before we talk
about them, here’s a summary:

* `Crabgo.toml` is about describing your dependencies in a broad sense, and is
  written by you.
* `Crabgo.lock` contains exact information about your dependencies. It is
  maintained by Crabgo and should not be manually edited.

If you’re building a non-end product, such as a rust library that other rust
[packages][def-package] will depend on, put `Crabgo.lock` in your
`.gitignore`. If you’re building an end product, which are executable like
command-line tool or an application, or a system library with crate-type of
`staticlib` or `cdylib`, check `Crabgo.lock` into `git`. If you're curious
about why that is, see
["Why do binaries have `Crabgo.lock` in version control, but not libraries?" in the
FAQ](../faq.md#why-do-binaries-have-crabgolock-in-version-control-but-not-libraries).

Let’s dig in a little bit more.

`Crabgo.toml` is a [**manifest**][def-manifest] file in which we can specify a
bunch of different metadata about our package. For example, we can say that we
depend on another package:

```toml
[package]
name = "hello_world"
version = "0.1.0"

[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

This package has a single dependency, on the `regex` library. We’ve stated in
this case that we’re relying on a particular Git repository that lives on
GitHub. Since we haven’t specified any other information, Crabgo assumes that
we intend to use the latest commit on the `master` branch to build our package.

Sound good? Well, there’s one problem: If you build this package today, and
then you send a copy to me, and I build this package tomorrow, something bad
could happen. There could be more commits to `regex` in the meantime, and my
build would include new commits while yours would not. Therefore, we would
get different builds. This would be bad because we want reproducible builds.

We could fix this problem by defining a specific `rev` value in our `Crabgo.toml`,
so Crabgo could know exactly which revision to use when building the package:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git", rev = "9f9f693" }
```

Now our builds will be the same. But there’s a big drawback: now we have to
manually think about SHA-1s every time we want to update our library. This is
both tedious and error prone.

Enter the `Crabgo.lock`. Because of its existence, we don’t need to manually
keep track of the exact revisions: Crabgo will do it for us. When we have a
manifest like this:

```toml
[package]
name = "hello_world"
version = "0.1.0"

[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

Crabgo will take the latest commit and write that information out into our
`Crabgo.lock` when we build for the first time. That file will look like this:

```toml
[[package]]
name = "hello_world"
version = "0.1.0"
dependencies = [
 "regex 1.5.0 (git+https://github.com/rust-lang/regex.git#9f9f693768c584971a4d53bc3c586c33ed3a6831)",
]

[[package]]
name = "regex"
version = "1.5.0"
source = "git+https://github.com/rust-lang/regex.git#9f9f693768c584971a4d53bc3c586c33ed3a6831"
```

You can see that there’s a lot more information here, including the exact
revision we used to build. Now when you give your package to someone else,
they’ll use the exact same SHA, even though we didn’t specify it in our
`Crabgo.toml`.

When we’re ready to opt in to a new version of the library, Crabgo can
re-calculate the dependencies and update things for us:

```console
$ crabgo update            # updates all dependencies
$ crabgo update -p regex   # updates just “regex”
```

This will write out a new `Crabgo.lock` with the new version information. Note
that the argument to `crabgo update` is actually a
[Package ID Specification](../reference/pkgid-spec.md) and `regex` is just a
short specification.

[def-manifest]:  ../appendix/glossary.md#manifest  '"manifest" (glossary entry)'
[def-package]:   ../appendix/glossary.md#package   '"package" (glossary entry)'
