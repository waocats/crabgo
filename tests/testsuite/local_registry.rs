//! Tests for local-registry sources.

use crabgo_test_support::paths::{self, CrabgoPathExt};
use crabgo_test_support::registry::{registry_path, Package};
use crabgo_test_support::{basic_manifest, project, t};
use std::fs;

fn setup() {
    let root = paths::root();
    t!(fs::create_dir(&root.join(".crabgo")));
    t!(fs::write(
        root.join(".crabgo/config"),
        r#"
            [source.crates-io]
            registry = 'https://wut'
            replace-with = 'my-awesome-local-registry'

            [source.my-awesome-local-registry]
            local-registry = 'registry'
        "#
    ));
}

#[crabgo_test]
fn simple() {
    setup();
    Package::new("bar", "0.0.1")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
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
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    p.crabgo("build")
        .with_stderr(
            "\
[UNPACKING] bar v0.0.1 ([..])
[COMPILING] bar v0.0.1
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("build").with_stderr("[FINISHED] [..]").run();
    p.crabgo("test").run();
}

#[crabgo_test]
fn not_found() {
    setup();
    // Publish a package so that the directory hierarchy is created.
    // Note, however, that we declare a dependency on baZ.
    Package::new("bar", "0.0.1").local(true).publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                baz = "0.0.1"
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate baz; pub fn foo() { baz::bar(); }",
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] no matching package named `baz` found
location searched: registry `crates-io`
required by package `foo v0.0.1 ([..]/foo)`
",
        )
        .run();
}

#[crabgo_test]
fn depend_on_yanked() {
    setup();
    Package::new("bar", "0.0.1").local(true).publish();

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
        .file("src/lib.rs", "")
        .build();

    // Run crabgo to create lock file.
    p.crabgo("check").run();

    registry_path().join("index").join("3").rm_rf();
    Package::new("bar", "0.0.1")
        .local(true)
        .yanked(true)
        .publish();

    p.crabgo("check")
        .with_stderr(
            "\
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn multiple_versions() {
    setup();
    Package::new("bar", "0.0.1").local(true).publish();
    Package::new("bar", "0.1.0")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
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
                bar = "*"
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[UNPACKING] bar v0.1.0 ([..])
[CHECKING] bar v0.1.0
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] [..]
",
        )
        .run();

    Package::new("bar", "0.2.0")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
        .publish();

    p.crabgo("update -v")
        .with_stderr("[UPDATING] bar v0.1.0 -> v0.2.0")
        .run();
}

#[crabgo_test]
fn multiple_names() {
    setup();
    Package::new("bar", "0.0.1")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
        .publish();
    Package::new("baz", "0.1.0")
        .local(true)
        .file("src/lib.rs", "pub fn baz() {}")
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
                bar = "*"
                baz = "*"
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                extern crate bar;
                extern crate baz;
                pub fn foo() {
                    bar::bar();
                    baz::baz();
                }
            "#,
        )
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[UNPACKING] [..]
[UNPACKING] [..]
[CHECKING] [..]
[CHECKING] [..]
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn interdependent() {
    setup();
    Package::new("bar", "0.0.1")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
        .publish();
    Package::new("baz", "0.1.0")
        .local(true)
        .dep("bar", "*")
        .file("src/lib.rs", "extern crate bar; pub fn baz() {}")
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
                bar = "*"
                baz = "*"
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                extern crate bar;
                extern crate baz;
                pub fn foo() {
                    bar::bar();
                    baz::baz();
                }
            "#,
        )
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[UNPACKING] [..]
[UNPACKING] [..]
[CHECKING] bar v0.0.1
[CHECKING] baz v0.1.0
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn path_dep_rewritten() {
    setup();
    Package::new("bar", "0.0.1")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
        .publish();
    Package::new("baz", "0.1.0")
        .local(true)
        .dep("bar", "*")
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "baz"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "bar", version = "*" }
            "#,
        )
        .file("src/lib.rs", "extern crate bar; pub fn baz() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
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
                bar = "*"
                baz = "*"
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                extern crate bar;
                extern crate baz;
                pub fn foo() {
                    bar::bar();
                    baz::baz();
                }
            "#,
        )
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[UNPACKING] [..]
[UNPACKING] [..]
[CHECKING] bar v0.0.1
[CHECKING] baz v0.1.0
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn invalid_dir_bad() {
    setup();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "*"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                [source.crates-io]
                registry = 'https://wut'
                replace-with = 'my-awesome-local-directory'

                [source.my-awesome-local-directory]
                local-registry = '/path/to/nowhere'
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to get `bar` as a dependency of package `foo v0.0.1 [..]`

Caused by:
  failed to load source for dependency `bar`

Caused by:
  Unable to update registry `crates-io`

Caused by:
  failed to update replaced source registry `crates-io`

Caused by:
  local registry path is not a directory: [..]path[..]to[..]nowhere
",
        )
        .run();
}

#[crabgo_test]
fn different_directory_replacing_the_registry_is_bad() {
    setup();

    // Move our test's .crabgo/config to a temporary location and publish a
    // registry package we're going to use first.
    let config = paths::root().join(".crabgo");
    let config_tmp = paths::root().join(".crabgo-old");
    t!(fs::rename(&config, &config_tmp));

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "*"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    // Generate a lock file against the crates.io registry
    Package::new("bar", "0.0.1").publish();
    p.crabgo("check").run();

    // Switch back to our directory source, and now that we're replacing
    // crates.io make sure that this fails because we're replacing with a
    // different checksum
    config.rm_rf();
    t!(fs::rename(&config_tmp, &config));
    Package::new("bar", "0.0.1")
        .file("src/lib.rs", "invalid")
        .local(true)
        .publish();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] checksum for `bar v0.0.1` changed between lock files

this could be indicative of a few possible errors:

    * the lock file is corrupt
    * a replacement source in use (e.g., a mirror) returned a different checksum
    * the source itself may be corrupt in one way or another

unable to verify that `bar v0.0.1` is the same as when the lockfile was generated

",
        )
        .run();
}

#[crabgo_test]
fn crates_io_registry_url_is_optional() {
    let root = paths::root();
    t!(fs::create_dir(&root.join(".crabgo")));
    t!(fs::write(
        root.join(".crabgo/config"),
        r#"
            [source.crates-io]
            replace-with = 'my-awesome-local-registry'

            [source.my-awesome-local-registry]
            local-registry = 'registry'
        "#
    ));

    Package::new("bar", "0.0.1")
        .local(true)
        .file("src/lib.rs", "pub fn bar() {}")
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
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    p.crabgo("build")
        .with_stderr(
            "\
[UNPACKING] bar v0.0.1 ([..])
[COMPILING] bar v0.0.1
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("build").with_stderr("[FINISHED] [..]").run();
    p.crabgo("test").run();
}
