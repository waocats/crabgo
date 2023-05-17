//! Tests for the `crabgo locate-project` command.

use crabgo_test_support::project;

#[crabgo_test]
fn simple() {
    let p = project().build();

    p.crabgo("locate-project")
        .with_json(r#"{"root": "[ROOT]/foo/Crabgo.toml"}"#)
        .run();
}

#[crabgo_test]
fn message_format() {
    let p = project().build();

    p.crabgo("locate-project --message-format plain")
        .with_stdout("[ROOT]/foo/Crabgo.toml")
        .run();

    p.crabgo("locate-project --message-format json")
        .with_json(r#"{"root": "[ROOT]/foo/Crabgo.toml"}"#)
        .run();

    p.crabgo("locate-project --message-format cryptic")
        .with_stderr("error: invalid message format specifier: `cryptic`")
        .with_status(101)
        .run();
}

#[crabgo_test]
fn workspace() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "outer"
                version = "0.0.0"

                [workspace]
                members = ["inner"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "inner/Crabgo.toml",
            r#"
                [package]
                name = "inner"
                version = "0.0.0"
            "#,
        )
        .file("inner/src/lib.rs", "")
        .build();

    let outer_manifest = r#"{"root": "[ROOT]/foo/Crabgo.toml"}"#;
    let inner_manifest = r#"{"root": "[ROOT]/foo/inner/Crabgo.toml"}"#;

    p.crabgo("locate-project").with_json(outer_manifest).run();

    p.crabgo("locate-project")
        .cwd("inner")
        .with_json(inner_manifest)
        .run();

    p.crabgo("locate-project --workspace")
        .with_json(outer_manifest)
        .run();

    p.crabgo("locate-project --workspace")
        .cwd("inner")
        .with_json(outer_manifest)
        .run();
}
