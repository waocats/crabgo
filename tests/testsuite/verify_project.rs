//! Tests for the `crabgo verify-project` command.

use crabgo_test_support::{basic_bin_manifest, main_file, project};

fn verify_project_success_output() -> String {
    r#"{"success":"true"}"#.into()
}

#[crabgo_test]
fn crabgo_verify_project_path_to_crabgo_toml_relative() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("verify-project --manifest-path foo/Crabgo.toml")
        .cwd(p.root().parent().unwrap())
        .with_stdout(verify_project_success_output())
        .run();
}

#[crabgo_test]
fn crabgo_verify_project_path_to_crabgo_toml_absolute() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("verify-project --manifest-path")
        .arg(p.root().join("Crabgo.toml"))
        .cwd(p.root().parent().unwrap())
        .with_stdout(verify_project_success_output())
        .run();
}

#[crabgo_test]
fn crabgo_verify_project_cwd() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("verify-project")
        .with_stdout(verify_project_success_output())
        .run();
}

#[crabgo_test]
fn crabgo_verify_project_honours_unstable_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["test-dummy-unstable"]

                [package]
                name = "foo"
                version = "0.0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("verify-project")
        .masquerade_as_nightly_crabgo(&["test-dummy-unstable"])
        .with_stdout(verify_project_success_output())
        .run();

    p.crabgo("verify-project")
        .with_status(1)
        .with_json(r#"{"invalid":"failed to parse manifest at `[CWD]/Crabgo.toml`"}"#)
        .run();
}
