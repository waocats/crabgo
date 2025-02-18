//! Tests for network configuration.

use crabgo_test_support::project;

#[crabgo_test]
fn net_retry_loads_from_config() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                git = "http://127.0.0.1:11/foo/bar"
            "#,
        )
        .file("src/main.rs", "")
        .file(
            ".crabgo/config",
            r#"
           [net]
           retry=1
           [http]
           timeout=1
            "#,
        )
        .build();

    p.crabgo("check -v")
        .with_status(101)
        .with_stderr_contains(
            "[WARNING] spurious network error \
             (1 tries remaining): [..]",
        )
        .run();
}

#[crabgo_test]
fn net_retry_git_outputs_warning() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                git = "http://127.0.0.1:11/foo/bar"
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
           [http]
           timeout=1
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check -v -j 1")
        .with_status(101)
        .with_stderr_contains(
            "[WARNING] spurious network error \
             (2 tries remaining): [..]",
        )
        .with_stderr_contains("[WARNING] spurious network error (1 tries remaining): [..]")
        .run();
}
