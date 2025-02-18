//! Tests for `[features]` table.

use crabgo_test_support::paths::CrabgoPathExt;
use crabgo_test_support::registry::{Dependency, Package};
use crabgo_test_support::{basic_manifest, project};

#[crabgo_test]
fn invalid1() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                bar = ["baz"]
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  feature `bar` includes `baz` which is neither a dependency nor another feature
",
        )
        .run();
}

#[crabgo_test]
fn same_name() {
    // Feature with the same name as a dependency.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                bar = ["baz"]
                baz = []

                [dependencies.bar]
                path = "bar"
            "#,
        )
        .file("src/main.rs", "")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "1.0.0"))
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("tree -f")
        .arg("{p} [{f}]")
        .with_stderr("")
        .with_stdout(
            "\
foo v0.0.1 ([..]) []
└── bar v1.0.0 ([..]) []
",
        )
        .run();

    p.crabgo("tree --features bar -f")
        .arg("{p} [{f}]")
        .with_stderr("")
        .with_stdout(
            "\
foo v0.0.1 ([..]) [bar,baz]
└── bar v1.0.0 ([..]) []
",
        )
        .run();
}

#[crabgo_test]
fn invalid3() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                bar = ["baz"]

                [dependencies.baz]
                path = "foo"
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  feature `bar` includes `baz`, but `baz` is not an optional dependency
  A non-optional dependency of the same name is defined; consider adding `optional = true` to its definition.
",
        )
        .run();
}

#[crabgo_test]
fn invalid4() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                features = ["bar"]
            "#,
        )
        .file("src/main.rs", "")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to select a version for `bar`.
    ... required by package `foo v0.0.1 ([..])`
versions that meet the requirements `*` are: 0.0.1

the package `foo` depends on `bar`, with features: `bar` but `bar` does not have these features.


failed to select a version for `bar` which could resolve this conflict",
        )
        .run();

    p.change_file("Crabgo.toml", &basic_manifest("foo", "0.0.1"));

    p.crabgo("check --features test")
        .with_status(101)
        .with_stderr("error: Package `foo v0.0.1 ([..])` does not have the feature `test`")
        .run();
}

#[crabgo_test]
fn invalid5() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dev-dependencies.bar]
                path = "bar"
                optional = true
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  dev-dependencies are not allowed to be optional: `bar`
",
        )
        .run();
}

#[crabgo_test]
fn invalid6() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                foo = ["bar/baz"]
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check --features foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  feature `foo` includes `bar/baz`, but `bar` is not a dependency
",
        )
        .run();
}

#[crabgo_test]
fn invalid7() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                foo = ["bar/baz"]
                bar = []
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check --features foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  feature `foo` includes `bar/baz`, but `bar` is not a dependency
",
        )
        .run();
}

#[crabgo_test]
fn invalid8() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                features = ["foo/bar"]
            "#,
        )
        .file("src/main.rs", "")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("check --features foo")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[CWD]/Crabgo.toml`

Caused by:
  feature `foo/bar` in dependency `bar` is not allowed to contain slashes
  If you want to enable features [..]
",
        )
        .run();
}

#[crabgo_test]
fn invalid9() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("check --features bar")
        .with_stderr(
            "\
error: Package `foo v0.0.1 ([..])` does not have feature `bar`. It has a required dependency with that name, but only optional dependencies can be used as features.
",
        ).with_status(101).run();
}

#[crabgo_test]
fn invalid10() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                features = ["baz"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies.baz]
                path = "baz"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .file("bar/baz/Crabgo.toml", &basic_manifest("baz", "0.0.1"))
        .file("bar/baz/src/lib.rs", "")
        .build();

    p.crabgo("check").with_stderr("\
error: failed to select a version for `bar`.
    ... required by package `foo v0.0.1 ([..])`
versions that meet the requirements `*` are: 0.0.1

the package `foo` depends on `bar`, with features: `baz` but `bar` does not have these features.
 It has a required dependency with that name, but only optional dependencies can be used as features.


failed to select a version for `bar` which could resolve this conflict
").with_status(101)
        .run();
}

#[crabgo_test]
fn no_transitive_dep_feature_requirement() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.derived]
                path = "derived"

                [features]
                default = ["derived/bar/qux"]
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                extern crate derived;
                fn main() { derived::test(); }
            "#,
        )
        .file(
            "derived/Crabgo.toml",
            r#"
                [package]
                name = "derived"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("derived/src/lib.rs", "extern crate bar; pub use bar::test;")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [features]
                qux = []
            "#,
        )
        .file(
            "bar/src/lib.rs",
            r#"
                #[cfg(feature = "qux")]
                pub fn test() { print!("test"); }
            "#,
        )
        .build();
    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[CWD]/Crabgo.toml`

Caused by:
  multiple slashes in feature `derived/bar/qux` (included by feature `default`) are not allowed
",
        )
        .run();
}

#[crabgo_test]
fn no_feature_doesnt_build() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[cfg(feature = "bar")]
                extern crate bar;
                #[cfg(feature = "bar")]
                fn main() { bar::bar(); println!("bar") }
                #[cfg(not(feature = "bar"))]
                fn main() {}
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    p.crabgo("build")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
    p.process(&p.bin("foo")).with_stdout("").run();

    p.crabgo("build --features bar -v")
        .with_stderr(
            "\
[COMPILING] bar v0.0.1 ([CWD]/bar)
[RUNNING] `rustc --crate-name bar [..]
[DIRTY-MSVC] foo v0.0.1 ([CWD]): the list of features changed
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name foo [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
    p.process(&p.bin("foo")).with_stdout("bar\n").run();
}

#[crabgo_test]
fn default_feature_pulled_in() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                default = ["bar"]

                [dependencies.bar]
                path = "bar"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[cfg(feature = "bar")]
                extern crate bar;
                #[cfg(feature = "bar")]
                fn main() { bar::bar(); println!("bar") }
                #[cfg(not(feature = "bar"))]
                fn main() {}
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    p.crabgo("build")
        .with_stderr(
            "\
[COMPILING] bar v0.0.1 ([CWD]/bar)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
    p.process(&p.bin("foo")).with_stdout("bar\n").run();

    p.crabgo("build --no-default-features -v")
        .with_stderr(
            "\
[DIRTY-MSVC] foo v0.0.1 ([CWD]): the list of features changed
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name foo [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
    p.process(&p.bin("foo")).with_stdout("").run();
}

#[crabgo_test]
fn cyclic_feature() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                default = ["default"]
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr("[ERROR] cyclic feature dependency: feature `default` depends on itself")
        .run();
}

#[crabgo_test]
fn cyclic_feature2() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                foo = ["bar"]
                bar = ["foo"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check").with_stdout("").run();
}

#[crabgo_test]
fn groups_on_groups_on_groups() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                default = ["f1"]
                f1 = ["f2", "bar"]
                f2 = ["f3", "f4"]
                f3 = ["f5", "f6", "baz"]
                f4 = ["f5", "f7"]
                f5 = ["f6"]
                f6 = ["f7"]
                f7 = ["bar"]

                [dependencies.bar]
                path = "bar"
                optional = true

                [dependencies.baz]
                path = "baz"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate bar;
                #[allow(unused_extern_crates)]
                extern crate baz;
                fn main() {}
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Crabgo.toml", &basic_manifest("baz", "0.0.1"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn many_cli_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                optional = true

                [dependencies.baz]
                path = "baz"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate bar;
                #[allow(unused_extern_crates)]
                extern crate baz;
                fn main() {}
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Crabgo.toml", &basic_manifest("baz", "0.0.1"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.crabgo("check --features")
        .arg("bar baz")
        .with_stderr(
            "\
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn union_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.d1]
                path = "d1"
                features = ["f1"]
                [dependencies.d2]
                path = "d2"
                features = ["f2"]
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate d1;
                extern crate d2;
                fn main() {
                    d2::f1();
                    d2::f2();
                }
            "#,
        )
        .file(
            "d1/Crabgo.toml",
            r#"
                [package]
                name = "d1"
                version = "0.0.1"
                authors = []

                [features]
                f1 = ["d2"]

                [dependencies.d2]
                path = "../d2"
                features = ["f1"]
                optional = true
            "#,
        )
        .file("d1/src/lib.rs", "")
        .file(
            "d2/Crabgo.toml",
            r#"
                [package]
                name = "d2"
                version = "0.0.1"
                authors = []

                [features]
                f1 = []
                f2 = []
            "#,
        )
        .file(
            "d2/src/lib.rs",
            r#"
                #[cfg(feature = "f1")] pub fn f1() {}
                #[cfg(feature = "f2")] pub fn f2() {}
            "#,
        )
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[CHECKING] d2 v0.0.1 ([CWD]/d2)
[CHECKING] d1 v0.0.1 ([CWD]/d1)
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn many_features_no_rebuilds() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name    = "b"
                version = "0.1.0"
                authors = []

                [dependencies.a]
                path = "a"
                features = ["fall"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name    = "a"
                version = "0.1.0"
                authors = []

                [features]
                ftest  = []
                ftest2 = []
                fall   = ["ftest", "ftest2"]
            "#,
        )
        .file("a/src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[CHECKING] a v0.1.0 ([CWD]/a)
[CHECKING] b v0.1.0 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
    p.root().move_into_the_past();

    p.crabgo("check -v")
        .with_stderr(
            "\
[FRESH] a v0.1.0 ([..]/a)
[FRESH] b v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

// Tests that all cmd lines work with `--features ""`
#[crabgo_test]
fn empty_features() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.crabgo("check --features").arg("").run();
}

// Tests that all cmd lines work with `--features ""`
#[crabgo_test]
fn transitive_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                foo = ["bar/baz"]

                [dependencies.bar]
                path = "bar"
            "#,
        )
        .file("src/main.rs", "extern crate bar; fn main() { bar::baz(); }")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [features]
                baz = []
            "#,
        )
        .file(
            "bar/src/lib.rs",
            r#"#[cfg(feature = "baz")] pub fn baz() {}"#,
        )
        .build();

    p.crabgo("check --features foo").run();
}

#[crabgo_test]
fn everything_in_the_lockfile() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                f1 = ["d1/f1"]
                f2 = ["d2"]

                [dependencies.d1]
                path = "d1"
                [dependencies.d2]
                path = "d2"
                optional = true
                [dependencies.d3]
                path = "d3"
                optional = true
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "d1/Crabgo.toml",
            r#"
                [package]
                name = "d1"
                version = "0.0.1"
                authors = []

                [features]
                f1 = []
            "#,
        )
        .file("d1/src/lib.rs", "")
        .file("d2/Crabgo.toml", &basic_manifest("d2", "0.0.2"))
        .file("d2/src/lib.rs", "")
        .file(
            "d3/Crabgo.toml",
            r#"
                [package]
                name = "d3"
                version = "0.0.3"
                authors = []

                [features]
                f3 = []
            "#,
        )
        .file("d3/src/lib.rs", "")
        .build();

    p.crabgo("fetch").run();
    let lockfile = p.read_lockfile();
    assert!(
        lockfile.contains(r#"name = "d1""#),
        "d1 not found\n{}",
        lockfile
    );
    assert!(
        lockfile.contains(r#"name = "d2""#),
        "d2 not found\n{}",
        lockfile
    );
    assert!(
        lockfile.contains(r#"name = "d3""#),
        "d3 not found\n{}",
        lockfile
    );
}

#[crabgo_test]
fn no_rebuild_when_frobbing_default_feature() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                a = { path = "a" }
                b = { path = "b" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "b/Crabgo.toml",
            r#"
                [package]
                name = "b"
                version = "0.1.0"
                authors = []

                [dependencies]
                a = { path = "../a", features = ["f1"], default-features = false }
            "#,
        )
        .file("b/src/lib.rs", "")
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name = "a"
                version = "0.1.0"
                authors = []

                [features]
                default = ["f1"]
                f1 = []
            "#,
        )
        .file("a/src/lib.rs", "")
        .build();

    p.crabgo("check").run();
    p.crabgo("check").with_stdout("").run();
    p.crabgo("check").with_stdout("").run();
}

#[crabgo_test]
fn unions_work_with_no_default_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                a = { path = "a" }
                b = { path = "b" }
            "#,
        )
        .file("src/lib.rs", "extern crate a; pub fn foo() { a::a(); }")
        .file(
            "b/Crabgo.toml",
            r#"
                [package]
                name = "b"
                version = "0.1.0"
                authors = []

                [dependencies]
                a = { path = "../a", features = [], default-features = false }
            "#,
        )
        .file("b/src/lib.rs", "")
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name = "a"
                version = "0.1.0"
                authors = []

                [features]
                default = ["f1"]
                f1 = []
            "#,
        )
        .file("a/src/lib.rs", r#"#[cfg(feature = "f1")] pub fn a() {}"#)
        .build();

    p.crabgo("check").run();
    p.crabgo("check").with_stdout("").run();
    p.crabgo("check").with_stdout("").run();
}

#[crabgo_test]
fn optional_and_dev_dep() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name    = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = { path = "foo", optional = true }
                [dev-dependencies]
                foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("foo/Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("foo/src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[CHECKING] test v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn activating_feature_activates_dep() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name    = "test"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = { path = "foo", optional = true }

                [features]
                a = ["foo/a"]
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate foo; pub fn bar() { foo::bar(); }",
        )
        .file(
            "foo/Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [features]
                a = []
            "#,
        )
        .file("foo/src/lib.rs", r#"#[cfg(feature = "a")] pub fn bar() {}"#)
        .build();

    p.crabgo("check --features a -v").run();
}

#[crabgo_test]
fn dep_feature_in_cmd_line() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.derived]
                path = "derived"
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                extern crate derived;
                fn main() { derived::test(); }
            "#,
        )
        .file(
            "derived/Crabgo.toml",
            r#"
                [package]
                name = "derived"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"

                [features]
                default = []
                derived-feat = ["bar/some-feat"]
            "#,
        )
        .file("derived/src/lib.rs", "extern crate bar; pub use bar::test;")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [features]
                some-feat = []
            "#,
        )
        .file(
            "bar/src/lib.rs",
            r#"
                #[cfg(feature = "some-feat")]
                pub fn test() { print!("test"); }
            "#,
        )
        .build();

    // The foo project requires that feature "some-feat" in "bar" is enabled.
    // Building without any features enabled should fail:
    p.crabgo("check")
        .with_status(101)
        .with_stderr_contains("[..]unresolved import `bar::test`")
        .run();

    // We should be able to enable the feature "derived-feat", which enables "some-feat",
    // on the command line. The feature is enabled, thus building should be successful:
    p.crabgo("check --features derived/derived-feat").run();

    // Trying to enable features of transitive dependencies is an error
    p.crabgo("check --features bar/some-feat")
        .with_status(101)
        .with_stderr("error: package `foo v0.0.1 ([..])` does not have a dependency named `bar`")
        .run();

    // Hierarchical feature specification should still be disallowed
    p.crabgo("check --features derived/bar/some-feat")
        .with_status(101)
        .with_stderr("[ERROR] multiple slashes in feature `derived/bar/some-feat` is not allowed")
        .run();
}

#[crabgo_test]
fn all_features_flag_enables_all_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                foo = []
                bar = []

                [dependencies.baz]
                path = "baz"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[cfg(feature = "foo")]
                pub fn foo() {}

                #[cfg(feature = "bar")]
                pub fn bar() {
                    extern crate baz;
                    baz::baz();
                }

                fn main() {
                    foo();
                    bar();
                }
            "#,
        )
        .file("baz/Crabgo.toml", &basic_manifest("baz", "0.0.1"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.crabgo("check --all-features").run();
}

#[crabgo_test]
fn many_cli_features_comma_delimited() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                optional = true

                [dependencies.baz]
                path = "baz"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate bar;
                #[allow(unused_extern_crates)]
                extern crate baz;
                fn main() {}
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Crabgo.toml", &basic_manifest("baz", "0.0.1"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.crabgo("check --features bar,baz")
        .with_stderr(
            "\
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn many_cli_features_comma_and_space_delimited() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                optional = true

                [dependencies.baz]
                path = "baz"
                optional = true

                [dependencies.bam]
                path = "bam"
                optional = true

                [dependencies.bap]
                path = "bap"
                optional = true
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[allow(unused_extern_crates)]
                extern crate bar;
                #[allow(unused_extern_crates)]
                extern crate baz;
                #[allow(unused_extern_crates)]
                extern crate bam;
                #[allow(unused_extern_crates)]
                extern crate bap;
                fn main() {}
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Crabgo.toml", &basic_manifest("baz", "0.0.1"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .file("bam/Crabgo.toml", &basic_manifest("bam", "0.0.1"))
        .file("bam/src/lib.rs", "pub fn bam() {}")
        .file("bap/Crabgo.toml", &basic_manifest("bap", "0.0.1"))
        .file("bap/src/lib.rs", "pub fn bap() {}")
        .build();

    p.crabgo("check --features")
        .arg("bar,baz bam bap")
        .with_stderr(
            "\
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] ba[..] v0.0.1 ([CWD]/ba[..])
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn only_dep_is_optional() {
    Package::new("bar", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                foo = ['bar']

                [dependencies]
                bar = { version = "0.1", optional = true }

                [dev-dependencies]
                bar = "0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check").run();
}

#[crabgo_test]
fn all_features_all_crates() {
    Package::new("bar", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [workspace]
                members = ['bar']
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [features]
                foo = []
            "#,
        )
        .file("bar/src/main.rs", "#[cfg(feature = \"foo\")] fn main() {}")
        .build();

    p.crabgo("check --all-features --workspace").run();
}

#[crabgo_test]
fn feature_off_dylib() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                members = ["bar"]

                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                crate-type = ["dylib"]

                [features]
                f1 = []
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                pub fn hello() -> &'static str {
                    if cfg!(feature = "f1") {
                        "f1"
                    } else {
                        "no f1"
                    }
                }
            "#,
        )
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"

                [dependencies]
                foo = { path = ".." }
            "#,
        )
        .file(
            "bar/src/main.rs",
            r#"
                extern crate foo;

                fn main() {
                    assert_eq!(foo::hello(), "no f1");
                }
            "#,
        )
        .build();

    // Build the dylib with `f1` feature.
    p.crabgo("check --features f1").run();
    // Check that building without `f1` uses a dylib without `f1`.
    p.crabgo("run -p bar").run();
}

#[crabgo_test]
fn warn_if_default_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "foo"
               version = "0.0.1"
               authors = []

               [dependencies.bar]
               path = "bar"
               optional = true

               [features]
               default-features = ["bar"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    p.crabgo("check")
        .with_stderr(
            r#"
[WARNING] `default-features = [".."]` was found in [features]. Did you mean to use `default = [".."]`?
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
            "#.trim(),
        ).run();
}

#[crabgo_test]
fn no_feature_for_non_optional_dep() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = { path = "bar" }
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[cfg(not(feature = "bar"))]
                fn main() {
                }
            "#,
        )
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [features]
                a = []
            "#,
        )
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    p.crabgo("check --features bar/a").run();
}

#[crabgo_test]
fn features_option_given_twice() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                a = []
                b = []
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[cfg(all(feature = "a", feature = "b"))]
                fn main() {}
            "#,
        )
        .build();

    p.crabgo("check --features a --features b").run();
}

#[crabgo_test]
fn multi_multi_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                a = []
                b = []
                c = []
            "#,
        )
        .file(
            "src/main.rs",
            r#"
               #[cfg(all(feature = "a", feature = "b", feature = "c"))]
               fn main() {}
            "#,
        )
        .build();

    p.crabgo("check --features a --features").arg("b c").run();
}

#[crabgo_test]
fn cli_parse_ok() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                a = []
            "#,
        )
        .file(
            "src/main.rs",
            r#"
               #[cfg(feature = "a")]
               fn main() {
                    assert_eq!(std::env::args().nth(1).unwrap(), "b");
               }
            "#,
        )
        .build();

    p.crabgo("run --features a b").run();
}

#[crabgo_test]
fn all_features_virtual_ws() {
    // What happens with `--all-features` in the root of a virtual workspace.
    // Some of this behavior is a little strange (member dependencies also
    // have all features enabled, one might expect `f4` to be disabled).
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                members = ["a", "b"]
            "#,
        )
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2018"

                [dependencies]
                b = {path="../b", optional=true}

                [features]
                default = ["f1"]
                f1 = []
                f2 = []
            "#,
        )
        .file(
            "a/src/main.rs",
            r#"
                fn main() {
                    if cfg!(feature="f1") {
                        println!("f1");
                    }
                    if cfg!(feature="f2") {
                        println!("f2");
                    }
                    #[cfg(feature="b")]
                    b::f();
                }
            "#,
        )
        .file(
            "b/Crabgo.toml",
            r#"
                [package]
                name = "b"
                version = "0.1.0"

                [features]
                default = ["f3"]
                f3 = []
                f4 = []
            "#,
        )
        .file(
            "b/src/lib.rs",
            r#"
                pub fn f() {
                    if cfg!(feature="f3") {
                        println!("f3");
                    }
                    if cfg!(feature="f4") {
                        println!("f4");
                    }
                }
            "#,
        )
        .build();

    p.crabgo("run").with_stdout("f1\n").run();
    p.crabgo("run --all-features")
        .with_stdout("f1\nf2\nf3\nf4\n")
        .run();
    // In `a`, it behaves differently. :(
    p.crabgo("run --all-features")
        .cwd("a")
        .with_stdout("f1\nf2\nf3\n")
        .run();
}

#[crabgo_test]
fn slash_optional_enables() {
    // --features dep/feat will enable `dep` and set its feature.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            dep = {path="dep", optional=true}
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #[cfg(not(feature="dep"))]
            compile_error!("dep not set");
            "#,
        )
        .file(
            "dep/Crabgo.toml",
            r#"
            [package]
            name = "dep"
            version = "0.1.0"

            [features]
            feat = []
            "#,
        )
        .file(
            "dep/src/lib.rs",
            r#"
            #[cfg(not(feature="feat"))]
            compile_error!("feat not set");
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr_contains("[..]dep not set[..]")
        .run();

    p.crabgo("check --features dep/feat").run();
}

#[crabgo_test]
fn registry_summary_order_doesnt_matter() {
    // Checks for an issue where the resolver depended on the order of entries
    // in the registry summary. If there was a non-optional dev-dependency
    // that appeared before an optional normal dependency, then the resolver
    // would not activate the optional dependency with a pkg/featname feature
    // syntax.
    Package::new("dep", "0.1.0")
        .feature("feat1", &[])
        .file(
            "src/lib.rs",
            r#"
                #[cfg(feature="feat1")]
                pub fn work() {
                    println!("it works");
                }
            "#,
        )
        .publish();
    Package::new("bar", "0.1.0")
        .feature("bar_feat", &["dep/feat1"])
        .add_dep(Dependency::new("dep", "0.1.0").dev())
        .add_dep(Dependency::new("dep", "0.1.0").optional(true))
        .file(
            "src/lib.rs",
            r#"
                // This will fail to compile without `dep` optional dep activated.
                extern crate dep;

                pub fn doit() {
                    dep::work();
                }
            "#,
        )
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                edition = "2018"

                [dependencies]
                bar = { version="0.1", features = ["bar_feat"] }
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                fn main() {
                    bar::doit();
                }
            "#,
        )
        .build();

    p.crabgo("run")
        .with_stderr(
            "\
[UPDATING] [..]
[DOWNLOADING] crates ...
[DOWNLOADED] [..]
[DOWNLOADED] [..]
[COMPILING] dep v0.1.0
[COMPILING] bar v0.1.0
[COMPILING] foo v0.1.0 [..]
[FINISHED] [..]
[RUNNING] `target/debug/foo[EXE]`
",
        )
        .with_stdout("it works")
        .run();
}

#[crabgo_test]
fn nonexistent_required_features() {
    Package::new("required_dependency", "0.1.0")
        .feature("simple", &[])
        .publish();
    Package::new("optional_dependency", "0.2.0")
        .feature("optional", &[])
        .publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"
            [features]
            existing = []
            fancy = ["optional_dependency"]
            [dependencies]
            required_dependency = { version = "0.1", optional = false}
            optional_dependency = { version = "0.2", optional = true}
            [[example]]
            name = "ololo"
            required-features = ["not_present",
                                 "existing",
                                 "fancy",
                                 "required_dependency/not_existing",
                                 "required_dependency/simple",
                                 "optional_dependency/optional",
                                 "not_specified_dependency/some_feature"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("examples/ololo.rs", "fn main() {}")
        .build();

    p.crabgo("check --examples")
        .with_stderr_contains(
            "\
[WARNING] invalid feature `not_present` in required-features of target `ololo`: \
    `not_present` is not present in [features] section
[WARNING] invalid feature `required_dependency/not_existing` in required-features \
    of target `ololo`: feature `not_existing` does not exist in package \
    `required_dependency v0.1.0`
[WARNING] invalid feature `not_specified_dependency/some_feature` in required-features \
    of target `ololo`: dependency `not_specified_dependency` does not exist
",
        )
        .run();
}

#[crabgo_test]
fn invalid_feature_names_warning() {
    // Warnings for more restricted feature syntax.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                # Some valid, but unusual names, shouldn't warn.
                "c++17" = []
                "128bit" = []
                "_foo" = []
                "feat-name" = []
                "feat_name" = []
                "foo.bar" = []

                # Invalid names.
                "+foo" = []
                "-foo" = []
                ".foo" = []
                "foo:bar" = []
                "foo?" = []
                "?foo" = []
                "ⒶⒷⒸ" = []
                "a¼" = []
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    // Unfortunately the warnings are duplicated due to the Summary being
    // loaded twice (once in the Workspace, and once in PackageRegistry) and
    // Crabgo does not have a de-duplication system. This should probably be
    // OK, since I'm not expecting this to affect anyone.
    p.crabgo("check")
        .with_stderr("\
[WARNING] invalid character `+` in feature `+foo` in package foo v0.1.0 ([ROOT]/foo), the first character must be a Unicode XID start character or digit (most letters or `_` or `0` to `9`)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `-` in feature `-foo` in package foo v0.1.0 ([ROOT]/foo), the first character must be a Unicode XID start character or digit (most letters or `_` or `0` to `9`)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `.` in feature `.foo` in package foo v0.1.0 ([ROOT]/foo), the first character must be a Unicode XID start character or digit (most letters or `_` or `0` to `9`)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `?` in feature `?foo` in package foo v0.1.0 ([ROOT]/foo), the first character must be a Unicode XID start character or digit (most letters or `_` or `0` to `9`)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `¼` in feature `a¼` in package foo v0.1.0 ([ROOT]/foo), characters must be Unicode XID characters, `+`, or `.` (numbers, `+`, `-`, `_`, `.`, or most letters)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `:` in feature `foo:bar` in package foo v0.1.0 ([ROOT]/foo), characters must be Unicode XID characters, `+`, or `.` (numbers, `+`, `-`, `_`, `.`, or most letters)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `?` in feature `foo?` in package foo v0.1.0 ([ROOT]/foo), characters must be Unicode XID characters, `+`, or `.` (numbers, `+`, `-`, `_`, `.`, or most letters)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `Ⓐ` in feature `ⒶⒷⒸ` in package foo v0.1.0 ([ROOT]/foo), the first character must be a Unicode XID start character or digit (most letters or `_` or `0` to `9`)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `Ⓑ` in feature `ⒶⒷⒸ` in package foo v0.1.0 ([ROOT]/foo), characters must be Unicode XID characters, `+`, or `.` (numbers, `+`, `-`, `_`, `.`, or most letters)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[WARNING] invalid character `Ⓒ` in feature `ⒶⒷⒸ` in package foo v0.1.0 ([ROOT]/foo), characters must be Unicode XID characters, `+`, or `.` (numbers, `+`, `-`, `_`, `.`, or most letters)
This was previously accepted but is being phased out; it will become a hard error in a future release.
For more information, see issue #8813 <https://github.com/rust-lang/crabgo/issues/8813>, and please leave a comment if this will be a problem for your project.
[CHECKING] foo v0.1.0 [..]
[FINISHED] [..]
")
        .run();
}

#[crabgo_test]
fn invalid_feature_names_error() {
    // Errors for more restricted feature syntax.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                "foo/bar" = []
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[CWD]/Crabgo.toml`

Caused by:
  feature named `foo/bar` is not allowed to contain slashes
",
        )
        .run();
}

#[crabgo_test]
fn default_features_conflicting_warning() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                a = { path = "a", features = ["f1"], default-features = false, default_features = false }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name = "a"
                version = "0.1.0"
                authors = []

                [features]
                default = ["f1"]
                f1 = []
            "#,
        )
        .file("a/src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_stderr_contains(
"[WARNING] conflicting between `default-features` and `default_features` in the `a` dependency.\n
        `default_features` is ignored and not recommended for use in the future"
        )
        .run();
}
