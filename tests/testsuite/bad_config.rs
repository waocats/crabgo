//! Tests for some invalid .crabgo/config files.

use crabgo_test_support::git::crabgo_uses_gitoxide;
use crabgo_test_support::registry::{self, Package};
use crabgo_test_support::{basic_manifest, project, rustc_host};

#[crabgo_test]
fn bad1() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                  [target]
                  nonexistent-target = "foo"
            "#,
        )
        .build();
    p.crabgo("check -v --target=nonexistent-target")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] expected table for configuration key `target.nonexistent-target`, \
but found string in [..]/config
",
        )
        .run();
}

#[crabgo_test]
fn bad2() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                  [http]
                    proxy = 3.0
            "#,
        )
        .build();
    p.crabgo("publish -v")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] could not load Crabgo configuration

Caused by:
  failed to load TOML configuration from `[..]config`

Caused by:
  failed to parse key `http`

Caused by:
  failed to parse key `proxy`

Caused by:
  found TOML configuration value of unknown type `float`
",
        )
        .run();
}

#[crabgo_test]
fn bad3() {
    let registry = registry::init();
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                [http]
                  proxy = true
            "#,
        )
        .build();
    Package::new("foo", "1.0.0").publish();

    p.crabgo("publish -v")
        .replace_crates_io(registry.index_url())
        .with_status(101)
        .with_stderr(
            "\
error: failed to update registry [..]

Caused by:
  error in [..]config: `http.proxy` expected a string, but found a boolean
",
        )
        .run();
}

#[crabgo_test]
fn bad4() {
    let p = project()
        .file(
            ".crabgo/config",
            r#"
                [crabgo-new]
                  vcs = false
            "#,
        )
        .build();
    p.crabgo("new -v foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] Failed to create package `foo` at `[..]`

Caused by:
  error in [..]config: `crabgo-new.vcs` expected a string, but found a boolean
",
        )
        .run();
}

#[crabgo_test]
fn bad6() {
    let registry = registry::init();
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                [http]
                  user-agent = true
            "#,
        )
        .build();
    Package::new("foo", "1.0.0").publish();

    p.crabgo("publish -v")
        .replace_crates_io(registry.index_url())
        .with_status(101)
        .with_stderr(
            "\
error: failed to update registry [..]

Caused by:
  error in [..]config: `http.user-agent` expected a string, but found a boolean
",
        )
        .run();
}

#[crabgo_test]
fn invalid_global_config() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies]
                foo = "0.1.0"
            "#,
        )
        .file(".crabgo/config", "4")
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check -v")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] could not load Crabgo configuration

Caused by:
  could not parse TOML configuration in `[..]`

Caused by:
  could not parse input as TOML

Caused by:
  TOML parse error at line 1, column 2
    |
  1 | 4
    |  ^
  expected `.`, `=`
",
        )
        .run();
}

#[crabgo_test]
fn bad_crabgo_lock() {
    let p = project()
        .file("Crabgo.lock", "[[package]]\nfoo = 92")
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check -v")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse lock file at: [..]Crabgo.lock

Caused by:
  missing field `name`
  in `package`
",
        )
        .run();
}

#[crabgo_test]
fn duplicate_packages_in_crabgo_lock() {
    Package::new("bar", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "Crabgo.lock",
            r#"
                [[package]]
                name = "foo"
                version = "0.0.1"
                dependencies = [
                 "bar 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)",
                ]

                [[package]]
                name = "bar"
                version = "0.1.0"
                source = "registry+https://github.com/rust-lang/crates.io-index"

                [[package]]
                name = "bar"
                version = "0.1.0"
                source = "registry+https://github.com/rust-lang/crates.io-index"
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse lock file at: [..]

Caused by:
  package `bar` is specified twice in the lockfile
",
        )
        .run();
}

#[crabgo_test]
fn bad_source_in_crabgo_lock() {
    Package::new("bar", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "Crabgo.lock",
            r#"
                [[package]]
                name = "foo"
                version = "0.0.1"
                dependencies = [
                 "bar 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)",
                ]

                [[package]]
                name = "bar"
                version = "0.1.0"
                source = "You shall not parse"
            "#,
        )
        .build();

    p.crabgo("check --verbose")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse lock file at: [..]

Caused by:
  invalid source `You shall not parse`
  in `package.source`
",
        )
        .run();
}

#[crabgo_test]
fn bad_dependency_in_lockfile() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "Crabgo.lock",
            r#"
                [[package]]
                name = "foo"
                version = "0.0.1"
                dependencies = [
                 "bar 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)",
                ]
            "#,
        )
        .build();

    p.crabgo("check").run();
}

#[crabgo_test]
fn bad_git_dependency() {
    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies]
                foo = {{ git = "{url}" }}
            "#,
                url = if crabgo_uses_gitoxide() {
                    "git://host.xz"
                } else {
                    "file:.."
                }
            ),
        )
        .file("src/lib.rs", "")
        .build();

    let expected_stderr = if crabgo_uses_gitoxide() {
        "\
[UPDATING] git repository `git://host.xz`
[ERROR] failed to get `foo` as a dependency of package `foo v0.0.0 [..]`

Caused by:
  failed to load source for dependency `foo`

Caused by:
  Unable to update git://host.xz

Caused by:
  failed to clone into: [..]

Caused by:
  URLs need to specify the path to the repository
"
    } else {
        "\
[UPDATING] git repository `file:///`
[ERROR] failed to get `foo` as a dependency of package `foo v0.0.0 [..]`

Caused by:
  failed to load source for dependency `foo`

Caused by:
  Unable to update file:///

Caused by:
  failed to clone into: [..]

Caused by:
  [..]'file:///' is not a valid local file URI[..]
"
    };
    p.crabgo("check -v")
        .with_status(101)
        .with_stderr(expected_stderr)
        .run();
}

#[crabgo_test]
fn bad_crate_type() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [lib]
                crate-type = ["bad_type", "rlib"]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("build -v")
        .with_status(101)
        .with_stderr_contains(
            "error: failed to run `rustc` to learn about crate-type bad_type information",
        )
        .run();
}

#[crabgo_test]
fn malformed_override() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [target.x86_64-apple-darwin.freetype]
                native = {
                  foo: "bar"
                }
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  could not parse input as TOML

Caused by:
  TOML parse error at line 8, column 27
    |
  8 |                 native = {
    |                           ^
  invalid inline table
  expected `}`
",
        )
        .run();
}

#[crabgo_test]
fn duplicate_binary_names() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "qqq"
               version = "0.1.0"
               authors = ["A <a@a.a>"]

               [[bin]]
               name = "e"
               path = "a.rs"

               [[bin]]
               name = "e"
               path = "b.rs"
            "#,
        )
        .file("a.rs", r#"fn main() -> () {}"#)
        .file("b.rs", r#"fn main() -> () {}"#)
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  found duplicate binary name e, but all binary targets must have a unique name
",
        )
        .run();
}

#[crabgo_test]
fn duplicate_example_names() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "qqq"
               version = "0.1.0"
               authors = ["A <a@a.a>"]

               [[example]]
               name = "ex"
               path = "examples/ex.rs"

               [[example]]
               name = "ex"
               path = "examples/ex2.rs"
            "#,
        )
        .file("examples/ex.rs", r#"fn main () -> () {}"#)
        .file("examples/ex2.rs", r#"fn main () -> () {}"#)
        .build();

    p.crabgo("check --example ex")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  found duplicate example name ex, but all example targets must have a unique name
",
        )
        .run();
}

#[crabgo_test]
fn duplicate_bench_names() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "qqq"
               version = "0.1.0"
               authors = ["A <a@a.a>"]

               [[bench]]
               name = "ex"
               path = "benches/ex.rs"

               [[bench]]
               name = "ex"
               path = "benches/ex2.rs"
            "#,
        )
        .file("benches/ex.rs", r#"fn main () {}"#)
        .file("benches/ex2.rs", r#"fn main () {}"#)
        .build();

    p.crabgo("bench")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  found duplicate bench name ex, but all bench targets must have a unique name
",
        )
        .run();
}

#[crabgo_test]
fn duplicate_deps() {
    let p = project()
        .file("shim-bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("shim-bar/src/lib.rs", "pub fn a() {}")
        .file("linux-bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("linux-bar/src/lib.rs", "pub fn a() {}")
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "qqq"
               version = "0.0.1"
               authors = []

               [dependencies]
               bar = { path = "shim-bar" }

               [target.x86_64-unknown-linux-gnu.dependencies]
               bar = { path = "linux-bar" }
            "#,
        )
        .file("src/main.rs", r#"fn main () {}"#)
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  Dependency 'bar' has different source paths depending on the build target. Each dependency must \
have a single canonical source path irrespective of build target.
",
        )
        .run();
}

#[crabgo_test]
fn duplicate_deps_diff_sources() {
    let p = project()
        .file("shim-bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("shim-bar/src/lib.rs", "pub fn a() {}")
        .file("linux-bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("linux-bar/src/lib.rs", "pub fn a() {}")
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "qqq"
               version = "0.0.1"
               authors = []

               [target.i686-unknown-linux-gnu.dependencies]
               bar = { path = "shim-bar" }

               [target.x86_64-unknown-linux-gnu.dependencies]
               bar = { path = "linux-bar" }
            "#,
        )
        .file("src/main.rs", r#"fn main () {}"#)
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  Dependency 'bar' has different source paths depending on the build target. Each dependency must \
have a single canonical source path irrespective of build target.
",
        )
        .run();
}

#[crabgo_test]
fn unused_keys() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
               [package]
               name = "foo"
               version = "0.1.0"
               authors = []

               [target.foo]
               bar = "3"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
warning: unused manifest key: target.foo.bar
[CHECKING] foo v0.1.0 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]

                name = "foo"
                version = "0.5.0"
                authors = ["wycats@example.com"]
                bulid = "foo"
            "#,
        )
        .file("src/lib.rs", "pub fn foo() {}")
        .build();
    p.crabgo("check")
        .with_stderr(
            "\
warning: unused manifest key: package.bulid
[CHECKING] foo [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    let p = project()
        .at("bar")
        .file(
            "Crabgo.toml",
            r#"
                [package]

                name = "foo"
                version = "0.5.0"
                authors = ["wycats@example.com"]

                [lib]
                build = "foo"
            "#,
        )
        .file("src/lib.rs", "pub fn foo() {}")
        .build();
    p.crabgo("check")
        .with_stderr(
            "\
warning: unused manifest key: lib.build
[CHECKING] foo [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn unused_keys_in_virtual_manifest() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                members = ["bar"]
                bulid = "foo"
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "")
        .build();
    p.crabgo("check --workspace")
        .with_stderr(
            "\
[WARNING] [..]/foo/Crabgo.toml: unused manifest key: workspace.bulid
[CHECKING] bar [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn empty_dependencies() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies]
                bar = {}
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.0.1").publish();

    p.crabgo("check")
        .with_stderr_contains(
            "\
warning: dependency (bar) specified without providing a local path, Git repository, version, \
or workspace dependency to use. This will be considered an error in future versions
",
        )
        .run();
}

#[crabgo_test]
fn invalid_toml_historically_allowed_fails() {
    let p = project()
        .file(".crabgo/config", "[bar] baz = 2")
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: could not load Crabgo configuration

Caused by:
  could not parse TOML configuration in `[..]`

Caused by:
  could not parse input as TOML

Caused by:
  TOML parse error at line 1, column 7
    |
  1 | [bar] baz = 2
    |       ^
  invalid table header
  expected newline, `#`
",
        )
        .run();
}

#[crabgo_test]
fn ambiguous_git_reference() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies.bar]
                git = "http://127.0.0.1"
                branch = "master"
                tag = "some-tag"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check -v")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  dependency (bar) specification is ambiguous. Only one of `branch`, `tag` or `rev` is allowed.
",
        )
        .run();
}

#[crabgo_test]
fn fragment_in_git_url() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies.bar]
                git = "http://127.0.0.1#foo"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check -v")
        .with_status(101)
        .with_stderr_contains(
            "\
[WARNING] URL fragment `#foo` in git URL is ignored for dependency (bar). \
If you were trying to specify a specific git revision, \
use `rev = \"foo\"` in the dependency declaration.
",
        )
        .run();
}

#[crabgo_test]
fn bad_source_config1() {
    let p = project()
        .file("src/lib.rs", "")
        .file(".crabgo/config", "[source.foo]")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr("error: no source location specified for `source.foo`, need [..]")
        .run();
}

#[crabgo_test]
fn bad_source_config2() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
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
                registry = 'http://example.com'
                replace-with = 'bar'
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to get `bar` as a dependency of package `foo v0.0.0 [..]`

Caused by:
  failed to load source for dependency `bar`

Caused by:
  Unable to update registry `crates-io`

Caused by:
  could not find a configured source with the name `bar` \
    when attempting to lookup `crates-io` (configuration in [..])
",
        )
        .run();
}

#[crabgo_test]
fn bad_source_config3() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
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
                registry = 'https://example.com'
                replace-with = 'crates-io'
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to get `bar` as a dependency of package `foo v0.0.0 [..]`

Caused by:
  failed to load source for dependency `bar`

Caused by:
  Unable to update registry `crates-io`

Caused by:
  detected a cycle of `replace-with` sources, [..]
",
        )
        .run();
}

#[crabgo_test]
fn bad_source_config4() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
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
                replace-with = 'bar'

                [source.bar]
                registry = 'https://example.com'
                replace-with = 'crates-io'
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to get `bar` as a dependency of package `foo v0.0.0 ([..])`

Caused by:
  failed to load source for dependency `bar`

Caused by:
  Unable to update registry `crates-io`

Caused by:
  detected a cycle of `replace-with` sources, the source `crates-io` is \
    eventually replaced with itself (configuration in [..])
",
        )
        .run();
}

#[crabgo_test]
fn bad_source_config5() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
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
                registry = 'https://example.com'
                replace-with = 'bar'

                [source.bar]
                registry = 'not a url'
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: configuration key `source.bar.registry` specified an invalid URL (in [..])

Caused by:
  invalid url `not a url`: [..]
",
        )
        .run();
}

#[crabgo_test]
fn both_git_and_path_specified() {
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies.bar]
                git = "http://127.0.0.1"
                path = "bar"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    foo.crabgo("check -v")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  dependency (bar) specification is ambiguous. Only one of `git` or `path` is allowed.
",
        )
        .run();
}

#[crabgo_test]
fn bad_source_config6() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
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
                registry = 'https://example.com'
                replace-with = ['not', 'a', 'string']
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] error in [..]/foo/.crabgo/config: could not load config key `source.crates-io.replace-with`

Caused by:
  error in [..]/foo/.crabgo/config: `source.crates-io.replace-with` expected a string, but found a array
"
        )
        .run();
}

#[crabgo_test]
fn ignored_git_revision() {
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies.bar]
                path = "bar"
                branch = "spam"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    let err_msg = "\
error: failed to parse manifest at `[..]`

Caused by:
  key `branch` is ignored for dependency (bar).
";
    foo.crabgo("check -v")
        .with_status(101)
        .with_stderr(err_msg)
        .run();

    // #11540, check that [target] dependencies fail the same way.
    foo.change_file(
        "Crabgo.toml",
        r#"
            [package]
            name = "foo"
            version = "0.0.0"

            [target.some-target.dependencies]
            bar = { path = "bar", branch = "spam" }
        "#,
    );
    foo.crabgo("check")
        .with_status(101)
        .with_stderr(err_msg)
        .run();
}

#[crabgo_test]
fn bad_source_config7() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies]
                bar = "*"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                [source.foo]
                registry = 'https://example.com'
                local-registry = 'file:///another/file'
            "#,
        )
        .build();

    Package::new("bar", "0.1.0").publish();

    p.crabgo("check")
        .with_status(101)
        .with_stderr("error: more than one source location specified for `source.foo`")
        .run();
}

#[crabgo_test]
fn bad_source_config8() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies]
                bar = "*"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
                [source.foo]
                branch = "somebranch"
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "[ERROR] source definition `source.foo` specifies `branch`, \
             but that requires a `git` key to be specified (in [..]/foo/.crabgo/config)",
        )
        .run();
}

#[crabgo_test]
fn bad_dependency() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [dependencies]
                bar = 3
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  invalid type: integer `3`, expected a version string like [..]
  in `dependencies.bar`
",
        )
        .run();
}

#[crabgo_test]
fn bad_debuginfo() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [profile.dev]
                debug = 'a'
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest [..]

Caused by:
  invalid value: string \"a\", expected a boolean, 0, 1, 2, \"line-tables-only\", or \"line-directives-only\"
  in `profile.dev.debug`
",
        )
        .run();
}

#[crabgo_test]
fn bad_debuginfo2() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []

                [profile.dev]
                debug = 3.6
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  invalid type: floating point `3.6`, expected a boolean, 0, 1, 2, \"line-tables-only\", or \"line-directives-only\"
  in `profile.dev.debug`
",
        )
        .run();
}

#[crabgo_test]
fn bad_opt_level() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.0"
                authors = []
                build = 3
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  expected a boolean or a string
  in `package.build`
",
        )
        .run();
}

#[crabgo_test]
fn warn_semver_metadata() {
    Package::new("bar", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "1.0.0"

            [dependencies]
            bar = "1.0.0+1234"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.crabgo("check")
        .with_stderr_contains("[WARNING] version requirement `1.0.0+1234` for dependency `bar`[..]")
        .run();
}

#[crabgo_test]
fn bad_target_cfg() {
    // Invalid type in a StringList.
    //
    // The error message is a bit unfortunate here. The type here ends up
    // being essentially Value<Value<StringList>>, and each layer of "Value"
    // adds some context to the error message. Also, untagged enums provide
    // strange error messages. Hopefully most users will be able to untangle
    // the message.
    let p = project()
        .file(
            ".crabgo/config",
            r#"
            [target.'cfg(not(target_os = "none"))']
            runner = false
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] error in [..]/foo/.crabgo/config: \
could not load config key `target.\"cfg(not(target_os = \\\"none\\\"))\".runner`

Caused by:
  error in [..]/foo/.crabgo/config: \
  could not load config key `target.\"cfg(not(target_os = \\\"none\\\"))\".runner`

Caused by:
  invalid configuration for key `target.\"cfg(not(target_os = \\\"none\\\"))\".runner`
  expected a string or array of strings, but found a boolean for \
  `target.\"cfg(not(target_os = \\\"none\\\"))\".runner` in [..]/foo/.crabgo/config
",
        )
        .run();
}

#[crabgo_test]
fn bad_target_links_overrides() {
    // Invalid parsing of links overrides.
    //
    // This error message is terrible. Nothing in the deserialization path is
    // using config::Value<>, so nothing is able to report the location. I
    // think this illustrates how the way things break down with how it
    // currently is designed with serde.
    let p = project()
        .file(
            ".crabgo/config",
            &format!(
                r#"
                [target.{}.somelib]
                rustc-flags = 'foo'
                "#,
                rustc_host()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "[ERROR] Only `-l` and `-L` flags are allowed in target config \
             `target.[..].rustc-flags` (in [..]foo/.crabgo/config): `foo`",
        )
        .run();

    p.change_file(
        ".crabgo/config",
        &format!(
            "[target.{}.somelib]
            warning = \"foo\"
            ",
            rustc_host(),
        ),
    );
    p.crabgo("check")
        .with_status(101)
        .with_stderr("[ERROR] `warning` is not supported in build script overrides")
        .run();
}

#[crabgo_test]
fn redefined_sources() {
    // Cannot define a source multiple times.
    let p = project()
        .file(
            ".crabgo/config",
            r#"
            [source.foo]
            registry = "https://github.com/rust-lang/crates.io-index"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] source `foo` defines source registry `crates-io`, \
    but that source is already defined by `crates-io`
note: Sources are not allowed to be defined multiple times.
",
        )
        .run();

    p.change_file(
        ".crabgo/config",
        r#"
        [source.one]
        directory = "index"

        [source.two]
        directory = "index"
        "#,
    );

    // Name is `[..]` because we can't guarantee the order.
    p.crabgo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] source `[..]` defines source dir [..]/foo/index, \
    but that source is already defined by `[..]`
note: Sources are not allowed to be defined multiple times.
",
        )
        .run();
}
