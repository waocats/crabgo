//! Tests for setting custom rustdoc flags.

use crabgo_test_support::project;

#[crabgo_test]
fn parses_env() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("doc -v")
        .env("RUSTDOCFLAGS", "--cfg=foo")
        .with_stderr_contains("[RUNNING] `rustdoc [..] --cfg=foo[..]`")
        .run();
}

#[crabgo_test]
fn parses_config() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                [build]
                rustdocflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.crabgo("doc -v")
        .with_stderr_contains("[RUNNING] `rustdoc [..] --cfg foo[..]`")
        .run();
}

#[crabgo_test]
fn bad_flags() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("doc")
        .env("RUSTDOCFLAGS", "--bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn rerun() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("doc").env("RUSTDOCFLAGS", "--cfg=foo").run();
    p.crabgo("doc")
        .env("RUSTDOCFLAGS", "--cfg=foo")
        .with_stderr("[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]")
        .run();
    p.crabgo("doc")
        .env("RUSTDOCFLAGS", "--cfg=bar")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustdocflags_passed_to_rustdoc_through_crabgo_test() {
    let p = project()
        .file(
            "src/lib.rs",
            r#"
                //! ```
                //! assert!(cfg!(do_not_choke));
                //! ```
            "#,
        )
        .build();

    p.crabgo("test --doc")
        .env("RUSTDOCFLAGS", "--cfg do_not_choke")
        .run();
}

#[crabgo_test]
fn rustdocflags_passed_to_rustdoc_through_crabgo_test_only_once() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("test --doc")
        .env("RUSTDOCFLAGS", "--markdown-no-toc")
        .run();
}

#[crabgo_test]
fn rustdocflags_misspelled() {
    let p = project().file("src/main.rs", "fn main() { }").build();

    p.crabgo("doc")
        .env("RUSTDOC_FLAGS", "foo")
        .with_stderr_contains("[WARNING] Crabgo does not read `RUSTDOC_FLAGS` environment variable. Did you mean `RUSTDOCFLAGS`?")
        .run();
}

#[crabgo_test]
fn whitespace() {
    // Checks behavior of different whitespace characters.
    let p = project().file("src/lib.rs", "").build();

    // "too many operands"
    p.crabgo("doc")
        .env("RUSTDOCFLAGS", "--crate-version this has spaces")
        .with_stderr_contains("[ERROR] could not document `foo`")
        .with_status(101)
        .run();

    const SPACED_VERSION: &str = "a\nb\tc\u{00a0}d";
    p.crabgo("doc")
        .env_remove("__CRABGO_TEST_FORCE_ARGFILE") // Not applicable for argfile.
        .env(
            "RUSTDOCFLAGS",
            format!("--crate-version {}", SPACED_VERSION),
        )
        .run();

    let contents = p.read_file("target/doc/foo/index.html");
    assert!(contents.contains(SPACED_VERSION));
}

#[crabgo_test]
fn not_affected_by_target_rustflags() {
    let cfg = if cfg!(windows) { "windows" } else { "unix" };
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            &format!(
                r#"
                    [target.'cfg({cfg})']
                    rustflags = ["-D", "missing-docs"]

                    [build]
                    rustdocflags = ["--cfg", "foo"]
                "#,
            ),
        )
        .build();

    // `crabgo build` should fail due to missing docs.
    p.crabgo("build -v")
        .with_status(101)
        .with_stderr_contains("[RUNNING] `rustc [..] -D missing-docs[..]`")
        .run();

    // `crabgo doc` shouldn't fail.
    p.crabgo("doc -v")
        .with_stderr_contains("[RUNNING] `rustdoc [..] --cfg foo[..]`")
        .run();
}
