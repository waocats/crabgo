//! Tests for the `crabgo yank` command.

use std::fs;

use crabgo_test_support::paths::CrabgoPathExt;
use crabgo_test_support::project;
use crabgo_test_support::registry;

fn setup(name: &str, version: &str) {
    let dir = registry::api_path().join(format!("api/v1/crates/{}/{}", name, version));
    dir.mkdir_p();
    fs::write(dir.join("yank"), r#"{"ok": true}"#).unwrap();
}

#[crabgo_test]
fn explicit_version() {
    let registry = registry::init();
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("yank --version 0.0.1")
        .replace_crates_io(registry.index_url())
        .run();

    p.crabgo("yank --undo --version 0.0.1")
        .replace_crates_io(registry.index_url())
        .with_status(101)
        .with_stderr(
            "    Updating crates.io index
      Unyank foo@0.0.1
error: failed to undo a yank from the registry at file:///[..]

Caused by:
  EOF while parsing a value at line 1 column 0",
        )
        .run();
}

#[crabgo_test]
fn explicit_version_with_asymmetric() {
    let registry = registry::RegistryBuilder::new()
        .http_api()
        .token(crabgo_test_support::registry::Token::rfc_key())
        .build();
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    // The http_api server will check that the authorization is correct.
    // If the authorization was not sent then we would get an unauthorized error.
    p.crabgo("yank --version 0.0.1")
        .arg("-Zregistry-auth")
        .masquerade_as_nightly_crabgo(&["registry-auth"])
        .replace_crates_io(registry.index_url())
        .run();

    p.crabgo("yank --undo --version 0.0.1")
        .arg("-Zregistry-auth")
        .masquerade_as_nightly_crabgo(&["registry-auth"])
        .replace_crates_io(registry.index_url())
        .run();
}

#[crabgo_test]
fn inline_version() {
    let registry = registry::init();
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("yank foo@0.0.1")
        .replace_crates_io(registry.index_url())
        .run();

    p.crabgo("yank --undo foo@0.0.1")
        .replace_crates_io(registry.index_url())
        .with_status(101)
        .with_stderr(
            "    Updating crates.io index
      Unyank foo@0.0.1
error: failed to undo a yank from the registry at file:///[..]

Caused by:
  EOF while parsing a value at line 1 column 0",
        )
        .run();
}

#[crabgo_test]
fn version_required() {
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("yank foo")
        .with_status(101)
        .with_stderr("error: `--version` is required")
        .run();
}

#[crabgo_test]
fn inline_version_without_name() {
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("yank @0.0.1")
        .with_status(101)
        .with_stderr("error: missing crate name for `@0.0.1`")
        .run();
}

#[crabgo_test]
fn inline_and_explicit_version() {
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("yank foo@0.0.1 --version 0.0.1")
        .with_status(101)
        .with_stderr("error: cannot specify both `@0.0.1` and `--version`")
        .run();
}
