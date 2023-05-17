//! Tests for renaming dependencies.

use crabgo_test_support::git;
use crabgo_test_support::paths;
use crabgo_test_support::registry::{self, Package};
use crabgo_test_support::{basic_manifest, project};

#[crabgo_test]
fn rename_dependency() {
    Package::new("bar", "0.1.0").publish();
    Package::new("bar", "0.2.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = { version = "0.1.0" }
                baz = { version = "0.2.0", package = "bar" }
            "#,
        )
        .file("src/lib.rs", "extern crate bar; extern crate baz;")
        .build();

    p.crabgo("build").run();
}

#[crabgo_test]
fn rename_with_different_names() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                baz = { path = "bar", package = "bar" }
            "#,
        )
        .file("src/lib.rs", "extern crate baz;")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [lib]
                name = "random_name"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("build").run();
}

#[crabgo_test]
fn lots_of_names() {
    registry::alt_init();
    Package::new("foo", "0.1.0")
        .file("src/lib.rs", "pub fn foo1() {}")
        .publish();
    Package::new("foo", "0.2.0")
        .file("src/lib.rs", "pub fn foo() {}")
        .publish();
    Package::new("foo", "0.1.0")
        .file("src/lib.rs", "pub fn foo2() {}")
        .alternative(true)
        .publish();

    let g = git::repo(&paths::root().join("another"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "pub fn foo3() {}")
        .build();

    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                    [package]
                    name = "test"
                    version = "0.1.0"
                    authors = []

                    [dependencies]
                    foo = "0.2"
                    foo1 = {{ version = "0.1", package = "foo" }}
                    foo2 = {{ version = "0.1", registry = "alternative", package = "foo" }}
                    foo3 = {{ git = '{}', package = "foo" }}
                    foo4 = {{ path = "foo", package = "foo" }}
                "#,
                g.url()
            ),
        )
        .file(
            "src/lib.rs",
            "
                extern crate foo;
                extern crate foo1;
                extern crate foo2;
                extern crate foo3;
                extern crate foo4;

                pub fn foo() {
                    foo::foo();
                    foo1::foo1();
                    foo2::foo2();
                    foo3::foo3();
                    foo4::foo4();
                }
            ",
        )
        .file("foo/Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("foo/src/lib.rs", "pub fn foo4() {}")
        .build();

    p.crabgo("build -v").run();
}

#[crabgo_test]
fn rename_and_patch() {
    Package::new("foo", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { version = "0.1", package = "foo" }

                [patch.crates-io]
                foo = { path = "foo" }
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::foo(); }",
        )
        .file("foo/Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("foo/src/lib.rs", "pub fn foo() {}")
        .build();

    p.crabgo("build -v").run();
}

#[crabgo_test]
fn rename_twice() {
    Package::new("foo", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { version = "0.1", package = "foo" }
                [build-dependencies]
                foo = { version = "0.1" }
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("build -v")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.1.0 (registry [..])
error: the crate `test v0.1.0 ([CWD])` depends on crate `foo v0.1.0` multiple times with different names
",
        )
        .run();
}

#[crabgo_test]
fn rename_affects_fingerprint() {
    Package::new("foo", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = { version = "0.1", package = "foo" }
            "#,
        )
        .file("src/lib.rs", "extern crate foo;")
        .build();

    p.crabgo("build -v").run();

    p.change_file(
        "Crabgo.toml",
        r#"
                [package]
                name = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { version = "0.1", package = "foo" }
        "#,
    );

    p.crabgo("build -v")
        .with_status(101)
        .with_stderr_contains("[..]can't find crate for `foo`")
        .run();
}

#[crabgo_test]
fn can_run_doc_tests() {
    Package::new("bar", "0.1.0").publish();
    Package::new("bar", "0.2.0").publish();

    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                bar = { version = "0.1.0" }
                baz = { version = "0.2.0", package = "bar" }
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate bar;
            extern crate baz;
        ",
        )
        .build();

    foo.crabgo("test -v")
        .with_stderr_contains(
            "\
[DOCTEST] foo
[RUNNING] `rustdoc [..]--test [..]src/lib.rs \
        [..] \
        --extern bar=[CWD]/target/debug/deps/libbar-[..].rlib \
        --extern baz=[CWD]/target/debug/deps/libbar-[..].rlib \
        [..]`
",
        )
        .run();
}

#[crabgo_test]
fn features_still_work() {
    Package::new("foo", "0.1.0").publish();
    Package::new("bar", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                p1 = { path = 'a', features = ['b'] }
                p2 = { path = 'b' }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name = "p1"
                version = "0.1.0"
                authors = []

                [dependencies]
                b = { version = "0.1", package = "foo", optional = true }
            "#,
        )
        .file("a/src/lib.rs", "extern crate b;")
        .file(
            "b/Crabgo.toml",
            r#"
                [package]
                name = "p2"
                version = "0.1.0"
                authors = []

                [dependencies]
                b = { version = "0.1", package = "bar", optional = true }

                [features]
                default = ['b']
            "#,
        )
        .file("b/src/lib.rs", "extern crate b;")
        .build();

    p.crabgo("build -v").run();
}

#[crabgo_test]
fn features_not_working() {
    Package::new("foo", "0.1.0").publish();
    Package::new("bar", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                a = { path = 'a', package = 'p1', optional = true }

                [features]
                default = ['p1']
            "#,
        )
        .file("src/lib.rs", "")
        .file("a/Crabgo.toml", &basic_manifest("p1", "0.1.0"))
        .build();

    p.crabgo("build -v")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  feature `default` includes `p1` which is neither a dependency nor another feature
",
        )
        .run();
}

#[crabgo_test]
fn rename_with_dash() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "qwerty"
                version = "0.1.0"

                [dependencies]
                foo-bar = { path = 'a', package = 'a' }
            "#,
        )
        .file("src/lib.rs", "extern crate foo_bar;")
        .file("a/Crabgo.toml", &basic_manifest("a", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    p.crabgo("build").run();
}
