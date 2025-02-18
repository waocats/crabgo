//! Tests specifically related to target handling (lib, bins, examples, tests, benches).

use crabgo_test_support::project;

#[crabgo_test]
fn warn_unmatched_target_filters() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
        [package]
        name = "foo"
        version = "0.1.0"

        [lib]
        test = false
        bench = false
        "#,
        )
        .file("src/lib.rs", r#"fn main() {}"#)
        .build();

    p.crabgo("check --tests --bins --examples --benches")
        .with_stderr(
            "\
[WARNING] Target filters `bins`, `tests`, `examples`, `benches` specified, \
but no targets matched. This is a no-op
[FINISHED][..]
",
        )
        .run();
}

#[crabgo_test]
fn reserved_windows_target_name() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [[bin]]
            name = "con"
            path = "src/main.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    if cfg!(windows) {
        p.crabgo("check")
            .with_stderr(
                "\
[WARNING] binary target `con` is a reserved Windows filename, \
this target will not work on Windows platforms
[CHECKING] foo[..]
[FINISHED][..]
",
            )
            .run();
    } else {
        p.crabgo("check")
            .with_stderr("[CHECKING] foo[..]\n[FINISHED][..]")
            .run();
    }
}
