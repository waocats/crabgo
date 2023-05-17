//! Tests for displaying the crabgo version.

use crabgo_test_support::{crabgo_process, project};

#[crabgo_test]
fn simple() {
    let p = project().build();

    p.crabgo("version")
        .with_stdout(&format!("crabgo {}\n", crabgo::version()))
        .run();

    p.crabgo("--version")
        .with_stdout(&format!("crabgo {}\n", crabgo::version()))
        .run();
}

#[crabgo_test]
fn version_works_without_rustc() {
    let p = project().build();
    p.crabgo("version").env("PATH", "").run();
}

#[crabgo_test]
fn version_works_with_bad_config() {
    let p = project().file(".crabgo/config", "this is not toml").build();
    p.crabgo("version").run();
}

#[crabgo_test]
fn version_works_with_bad_target_dir() {
    let p = project()
        .file(
            ".crabgo/config",
            r#"
                [build]
                target-dir = 4
            "#,
        )
        .build();
    p.crabgo("version").run();
}

#[crabgo_test]
fn verbose() {
    // This is mainly to check that it doesn't explode.
    crabgo_process("-vV")
        .with_stdout_contains(&format!("crabgo {}", crabgo::version()))
        .with_stdout_contains("host: [..]")
        .with_stdout_contains("libgit2: [..]")
        .with_stdout_contains("libcurl: [..]")
        .with_stdout_contains("os: [..]")
        .run();
}
