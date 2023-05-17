//! Tests for --message-format flag.

use crabgo_test_support::{basic_lib_manifest, basic_manifest, project};

#[crabgo_test]
fn cannot_specify_two() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    let formats = ["human", "json", "short"];

    let two_kinds = "error: cannot specify two kinds of `message-format` arguments\n";
    for a in formats.iter() {
        for b in formats.iter() {
            p.crabgo(&format!("build --message-format {},{}", a, b))
                .with_status(101)
                .with_stderr(two_kinds)
                .run();
        }
    }
}

#[crabgo_test]
fn double_json_works() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check --message-format json,json-render-diagnostics")
        .run();
    p.crabgo("check --message-format json,json-diagnostic-short")
        .run();
    p.crabgo("check --message-format json,json-diagnostic-rendered-ansi")
        .run();
    p.crabgo("check --message-format json --message-format json-diagnostic-rendered-ansi")
        .run();
    p.crabgo("check --message-format json-diagnostic-rendered-ansi")
        .run();
    p.crabgo("check --message-format json-diagnostic-short,json-diagnostic-rendered-ansi")
        .run();
}

#[crabgo_test]
fn crabgo_renders() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = 'foo'
                version = '0.1.0'

                [dependencies]
                bar = { path = 'bar' }
            "#,
        )
        .file("src/main.rs", "")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("check --message-format json-render-diagnostics")
        .with_status(101)
        .with_stdout(
            "{\"reason\":\"compiler-artifact\",[..]\n\
             {\"reason\":\"build-finished\",\"success\":false}",
        )
        .with_stderr_contains(
            "\
[CHECKING] bar [..]
[CHECKING] foo [..]
error[..]`main`[..]
",
        )
        .run();
}

#[crabgo_test]
fn crabgo_renders_short() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "")
        .build();

    p.crabgo("check --message-format json-render-diagnostics,json-diagnostic-short")
        .with_status(101)
        .with_stderr_contains(
            "\
[CHECKING] foo [..]
error[..]`main`[..]
",
        )
        .with_stderr_does_not_contain("note:")
        .run();
}

#[crabgo_test]
fn crabgo_renders_ansi() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "")
        .build();

    p.crabgo("check --message-format json-diagnostic-rendered-ansi")
        .with_status(101)
        .with_stdout_contains("[..]\\u001b[38;5;9merror[..]")
        .run();
}

#[crabgo_test]
fn crabgo_renders_doctests() {
    let p = project()
        .file("Crabgo.toml", &basic_lib_manifest("foo"))
        .file(
            "src/lib.rs",
            "\
            /// ```rust
            /// bar()
            /// ```
            pub fn bar() {}
            ",
        )
        .build();

    p.crabgo("test --doc --message-format short")
        .with_status(101)
        .with_stdout_contains("src/lib.rs:2:1: error[E0425]:[..]")
        .with_stdout_contains("[..]src/lib.rs - bar (line 1)[..]")
        .run();
}
