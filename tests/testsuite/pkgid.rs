//! Tests for the `crabgo pkgid` command.

use crabgo_test_support::project;
use crabgo_test_support::registry::Package;

#[crabgo_test]
fn simple() {
    Package::new("bar", "0.1.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                edition = "2018"

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("generate-lockfile").run();

    p.crabgo("pkgid foo")
        .with_stdout(format!("file://[..]{}#0.1.0", p.root().to_str().unwrap()))
        .run();

    p.crabgo("pkgid bar")
        .with_stdout("https://github.com/rust-lang/crates.io-index#bar@0.1.0")
        .run();
}

#[crabgo_test]
fn suggestion_bad_pkgid() {
    Package::new("crates-io", "0.1.0").publish();
    Package::new("two-ver", "0.1.0").publish();
    Package::new("two-ver", "0.2.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                edition = "2018"

                [dependencies]
                crates-io = "0.1.0"
                two-ver = "0.1.0"
                two-ver2 = { package = "two-ver", version = "0.2.0" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("cratesio", "")
        .build();

    p.crabgo("generate-lockfile").run();

    // Bad URL.
    p.crabgo("pkgid https://example.com/crates-io")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `https://example.com/crates-io` did not match any packages
Did you mean one of these?

  crates-io@0.1.0
",
        )
        .run();

    // Bad name.
    p.crabgo("pkgid crates_io")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `crates_io` did not match any packages

<tab>Did you mean `crates-io`?
",
        )
        .run();

    // Bad version.
    p.crabgo("pkgid two-ver:0.3.0")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `two-ver@0.3.0` did not match any packages
Did you mean one of these?

  two-ver@0.1.0
  two-ver@0.2.0
",
        )
        .run();

    // Bad file URL.
    p.crabgo("pkgid ./Crabgo.toml")
        .with_status(101)
        .with_stderr(
            "\
error: invalid package ID specification: `./Crabgo.toml`

Caused by:
  package ID specification `./Crabgo.toml` looks like a file path, maybe try file://[..]/Crabgo.toml
",
        )
        .run();

    // Bad file URL with similar name.
    p.crabgo("pkgid './cratesio'")
        .with_status(101)
        .with_stderr(
            "\
error: invalid package ID specification: `./cratesio`

<tab>Did you mean `crates-io`?

Caused by:
  package ID specification `./cratesio` looks like a file path, maybe try file://[..]/cratesio
",
        )
        .run();
}
