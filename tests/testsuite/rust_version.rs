//! Tests for targets with `rust-version`.

use crabgo_test_support::{project, registry::Package};

#[crabgo_test]
fn rust_version_satisfied() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.1.1"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check").run();
    p.crabgo("check --ignore-rust-version").run();
}

#[crabgo_test]
fn rust_version_bad_caret() {
    project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "^1.43"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .crabgo("check")
        .with_status(101)
        .with_stderr(
            "error: failed to parse manifest at `[..]`\n\n\
             Caused by:\n  `rust-version` must be a value like \"1.32\"",
        )
        .run();
}

#[crabgo_test]
fn rust_version_bad_pre_release() {
    project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.43-beta.1"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .crabgo("check")
        .with_status(101)
        .with_stderr(
            "error: failed to parse manifest at `[..]`\n\n\
             Caused by:\n  `rust-version` must be a value like \"1.32\"",
        )
        .run();
}

#[crabgo_test]
fn rust_version_bad_nonsense() {
    project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "foodaddle"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .crabgo("check")
        .with_status(101)
        .with_stderr(
            "error: failed to parse manifest at `[..]`\n\n\
             Caused by:\n  `rust-version` must be a value like \"1.32\"",
        )
        .run();
}

#[crabgo_test]
fn rust_version_too_high() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.9876.0"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "error: package `foo v0.0.1 ([..])` cannot be built because it requires \
             rustc 1.9876.0 or newer, while the currently active rustc version is [..]\n\n",
        )
        .run();
    p.crabgo("check --ignore-rust-version").run();
}

#[crabgo_test]
fn rust_version_dependency_fails() {
    Package::new("bar", "0.0.1")
        .rust_version("1.2345.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            [dependencies]
            bar = "0.0.1"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "    Updating `[..]` index\n \
             Downloading crates ...\n  \
             Downloaded bar v0.0.1 (registry `[..]`)\n\
             error: package `bar v0.0.1` cannot be built because it requires \
             rustc 1.2345.0 or newer, while the currently active rustc version is [..]\n\
             Either upgrade to rustc 1.2345.0 or newer, or use\n\
             crabgo update -p bar@0.0.1 --precise ver\n\
             where `ver` is the latest version of `bar` supporting rustc [..]",
        )
        .run();
    p.crabgo("check --ignore-rust-version").run();
}

#[crabgo_test]
fn rust_version_older_than_edition() {
    project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.1"
            edition = "2018"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .crabgo("check")
        .with_status(101)
        .with_stderr_contains("  rust-version 1.1 is older than first version (1.31.0) required by the specified edition (2018)",
        )
        .run();
}
