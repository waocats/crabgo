//! Tests for the `crabgo tree` command.

use super::features2::switch_to_resolver_2;
use crabgo_test_support::cross_compile::{self, alternate};
use crabgo_test_support::registry::{Dependency, Package};
use crabgo_test_support::{basic_manifest, git, project, rustc_host, Project};

fn make_simple_proj() -> Project {
    Package::new("c", "1.0.0").publish();
    Package::new("b", "1.0.0").dep("c", "1.0").publish();
    Package::new("a", "1.0.0").dep("b", "1.0").publish();
    Package::new("bdep", "1.0.0").dep("b", "1.0").publish();
    Package::new("devdep", "1.0.0").dep("b", "1.0.0").publish();

    project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            a = "1.0"
            c = "1.0"

            [build-dependencies]
            bdep = "1.0"

            [dev-dependencies]
            devdep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build()
}

#[crabgo_test]
fn simple() {
    // A simple test with a few different dependencies.
    let p = make_simple_proj();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
│   └── b v1.0.0
│       └── c v1.0.0
└── c v1.0.0
[build-dependencies]
└── bdep v1.0.0
    └── b v1.0.0 (*)
[dev-dependencies]
└── devdep v1.0.0
    └── b v1.0.0 (*)
",
        )
        .run();

    p.crabgo("tree -p bdep")
        .with_stdout(
            "\
bdep v1.0.0
└── b v1.0.0
    └── c v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn virtual_workspace() {
    // Multiple packages in a virtual workspace.
    Package::new("somedep", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [workspace]
            members = ["a", "baz", "c"]
            "#,
        )
        .file("a/Crabgo.toml", &basic_manifest("a", "1.0.0"))
        .file("a/src/lib.rs", "")
        .file(
            "baz/Crabgo.toml",
            r#"
            [package]
            name = "baz"
            version = "0.1.0"

            [dependencies]
            c = { path = "../c" }
            somedep = "1.0"
            "#,
        )
        .file("baz/src/lib.rs", "")
        .file("c/Crabgo.toml", &basic_manifest("c", "1.0.0"))
        .file("c/src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
a v1.0.0 ([..]/foo/a)

baz v0.1.0 ([..]/foo/baz)
├── c v1.0.0 ([..]/foo/c)
└── somedep v1.0.0

c v1.0.0 ([..]/foo/c)
",
        )
        .run();

    p.crabgo("tree -p a").with_stdout("a v1.0.0 [..]").run();

    p.crabgo("tree")
        .cwd("baz")
        .with_stdout(
            "\
baz v0.1.0 ([..]/foo/baz)
├── c v1.0.0 ([..]/foo/c)
└── somedep v1.0.0
",
        )
        .run();

    // exclude baz
    p.crabgo("tree --workspace --exclude baz")
        .with_stdout(
            "\
a v1.0.0 ([..]/foo/a)

c v1.0.0 ([..]/foo/c)
",
        )
        .run();

    // exclude glob '*z'
    p.crabgo("tree --workspace --exclude '*z'")
        .with_stdout(
            "\
a v1.0.0 ([..]/foo/a)

c v1.0.0 ([..]/foo/c)
",
        )
        .run();

    // include glob '*z'
    p.crabgo("tree -p '*z'")
        .with_stdout(
            "\
baz v0.1.0 ([..]/foo/baz)
├── c v1.0.0 ([..]/foo/c)
└── somedep v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn dedupe_edges() {
    // Works around https://github.com/rust-lang/crabgo/issues/7985
    Package::new("bitflags", "1.0.0").publish();
    Package::new("manyfeat", "1.0.0")
        .feature("f1", &[])
        .feature("f2", &[])
        .feature("f3", &[])
        .dep("bitflags", "1.0")
        .publish();
    Package::new("a", "1.0.0")
        .feature_dep("manyfeat", "1.0", &["f1"])
        .publish();
    Package::new("b", "1.0.0")
        .feature_dep("manyfeat", "1.0", &["f2"])
        .publish();
    Package::new("c", "1.0.0")
        .feature_dep("manyfeat", "1.0", &["f3"])
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            a = "1.0"
            b = "1.0"
            c = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
│   └── manyfeat v1.0.0
│       └── bitflags v1.0.0
├── b v1.0.0
│   └── manyfeat v1.0.0 (*)
└── c v1.0.0
    └── manyfeat v1.0.0 (*)
",
        )
        .run();
}

#[crabgo_test]
fn renamed_deps() {
    // Handles renamed dependencies.
    Package::new("one", "1.0.0").publish();
    Package::new("two", "1.0.0").publish();
    Package::new("bar", "1.0.0").dep("one", "1.0").publish();
    Package::new("bar", "2.0.0").dep("two", "1.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "1.0.0"

            [dependencies]
            bar1 = {version = "1.0", package="bar"}
            bar2 = {version = "2.0", package="bar"}
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v1.0.0 ([..]/foo)
├── bar v1.0.0
│   └── one v1.0.0
└── bar v2.0.0
    └── two v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn source_kinds() {
    // Handles git and path sources.
    Package::new("regdep", "1.0.0").publish();
    let git_project = git::new("gitdep", |p| {
        p.file("Crabgo.toml", &basic_manifest("gitdep", "1.0.0"))
            .file("src/lib.rs", "")
    });
    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                regdep = "1.0"
                pathdep = {{ path = "pathdep" }}
                gitdep = {{ git = "{}" }}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .file("pathdep/Crabgo.toml", &basic_manifest("pathdep", "1.0.0"))
        .file("pathdep/src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── gitdep v1.0.0 (file://[..]/gitdep#[..])
├── pathdep v1.0.0 ([..]/foo/pathdep)
└── regdep v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn features() {
    // Exercises a variety of feature behaviors.
    Package::new("optdep_default", "1.0.0").publish();
    Package::new("optdep", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "a"
            version = "0.1.0"

            [dependencies]
            optdep_default = { version = "1.0", optional = true }
            optdep = { version = "1.0", optional = true }

            [features]
            default = ["optdep_default"]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo)
└── optdep_default v1.0.0
",
        )
        .run();

    p.crabgo("tree --no-default-features")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo)
",
        )
        .run();

    p.crabgo("tree --all-features")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo)
├── optdep v1.0.0
└── optdep_default v1.0.0
",
        )
        .run();

    p.crabgo("tree --features optdep")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo)
├── optdep v1.0.0
└── optdep_default v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn filters_target() {
    // --target flag
    if cross_compile::disabled() {
        return;
    }
    Package::new("targetdep", "1.0.0").publish();
    Package::new("hostdep", "1.0.0").publish();
    Package::new("devdep", "1.0.0").publish();
    Package::new("build_target_dep", "1.0.0").publish();
    Package::new("build_host_dep", "1.0.0")
        .target_dep("targetdep", "1.0", alternate())
        .target_dep("hostdep", "1.0", rustc_host())
        .publish();
    Package::new("pm_target", "1.0.0")
        .proc_macro(true)
        .publish();
    Package::new("pm_host", "1.0.0").proc_macro(true).publish();

    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [target.'{alt}'.dependencies]
                targetdep = "1.0"
                pm_target = "1.0"

                [target.'{host}'.dependencies]
                hostdep = "1.0"
                pm_host = "1.0"

                [target.'{alt}'.dev-dependencies]
                devdep = "1.0"

                [target.'{alt}'.build-dependencies]
                build_target_dep = "1.0"

                [target.'{host}'.build-dependencies]
                build_host_dep = "1.0"
                "#,
                alt = alternate(),
                host = rustc_host()
            ),
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── hostdep v1.0.0
└── pm_host v1.0.0 (proc-macro)
[build-dependencies]
└── build_host_dep v1.0.0
    └── hostdep v1.0.0
",
        )
        .run();

    p.crabgo("tree --target")
        .arg(alternate())
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── pm_target v1.0.0 (proc-macro)
└── targetdep v1.0.0
[build-dependencies]
└── build_host_dep v1.0.0
    └── hostdep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
",
        )
        .run();

    p.crabgo("tree --target")
        .arg(rustc_host())
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── hostdep v1.0.0
└── pm_host v1.0.0 (proc-macro)
[build-dependencies]
└── build_host_dep v1.0.0
    └── hostdep v1.0.0
",
        )
        .run();

    p.crabgo("tree --target=all")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── hostdep v1.0.0
├── pm_host v1.0.0 (proc-macro)
├── pm_target v1.0.0 (proc-macro)
└── targetdep v1.0.0
[build-dependencies]
├── build_host_dep v1.0.0
│   ├── hostdep v1.0.0
│   └── targetdep v1.0.0
└── build_target_dep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
",
        )
        .run();

    // no-proc-macro
    p.crabgo("tree --target=all -e no-proc-macro")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── hostdep v1.0.0
└── targetdep v1.0.0
[build-dependencies]
├── build_host_dep v1.0.0
│   ├── hostdep v1.0.0
│   └── targetdep v1.0.0
└── build_target_dep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn no_selected_target_dependency() {
    // --target flag
    if cross_compile::disabled() {
        return;
    }
    Package::new("targetdep", "1.0.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [target.'{alt}'.dependencies]
                targetdep = "1.0"

                "#,
                alt = alternate(),
            ),
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
",
        )
        .run();

    p.crabgo("tree -i targetdep")
        .with_stderr(
            "\
[WARNING] nothing to print.

To find dependencies that require specific target platforms, \
try to use option `--target all` first, and then narrow your search scope accordingly.
",
        )
        .run();
    p.crabgo("tree -i targetdep --target all")
        .with_stdout(
            "\
targetdep v1.0.0
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn dep_kinds() {
    Package::new("inner-devdep", "1.0.0").publish();
    Package::new("inner-builddep", "1.0.0").publish();
    Package::new("inner-normal", "1.0.0").publish();
    Package::new("inner-pm", "1.0.0").proc_macro(true).publish();
    Package::new("inner-buildpm", "1.0.0")
        .proc_macro(true)
        .publish();
    Package::new("normaldep", "1.0.0")
        .dep("inner-normal", "1.0")
        .dev_dep("inner-devdep", "1.0")
        .build_dep("inner-builddep", "1.0")
        .publish();
    Package::new("devdep", "1.0.0")
        .dep("inner-normal", "1.0")
        .dep("inner-pm", "1.0")
        .dev_dep("inner-devdep", "1.0")
        .build_dep("inner-builddep", "1.0")
        .build_dep("inner-buildpm", "1.0")
        .publish();
    Package::new("builddep", "1.0.0")
        .dep("inner-normal", "1.0")
        .dev_dep("inner-devdep", "1.0")
        .build_dep("inner-builddep", "1.0")
        .publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            normaldep = "1.0"

            [dev-dependencies]
            devdep = "1.0"

            [build-dependencies]
            builddep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── normaldep v1.0.0
    └── inner-normal v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
[build-dependencies]
└── builddep v1.0.0
    └── inner-normal v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
    ├── inner-normal v1.0.0
    └── inner-pm v1.0.0 (proc-macro)
    [build-dependencies]
    ├── inner-builddep v1.0.0
    └── inner-buildpm v1.0.0 (proc-macro)
",
        )
        .run();

    p.crabgo("tree -e no-dev")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── normaldep v1.0.0
    └── inner-normal v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
[build-dependencies]
└── builddep v1.0.0
    └── inner-normal v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
",
        )
        .run();

    p.crabgo("tree -e normal")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── normaldep v1.0.0
    └── inner-normal v1.0.0
",
        )
        .run();

    p.crabgo("tree -e dev,build")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
[build-dependencies]
└── builddep v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
    [build-dependencies]
    ├── inner-builddep v1.0.0
    └── inner-buildpm v1.0.0 (proc-macro)
",
        )
        .run();

    p.crabgo("tree -e dev,build,no-proc-macro")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
[build-dependencies]
└── builddep v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
    [build-dependencies]
    └── inner-builddep v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn cyclic_dev_dep() {
    // Cyclical dev-dependency and inverse flag.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dev-dependencies]
            dev-dep = { path = "dev-dep" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "dev-dep/Crabgo.toml",
            r#"
            [package]
            name = "dev-dep"
            version = "0.1.0"

            [dependencies]
            foo = { path=".." }
            "#,
        )
        .file("dev-dep/src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
[dev-dependencies]
└── dev-dep v0.1.0 ([..]/foo/dev-dep)
    └── foo v0.1.0 ([..]/foo) (*)
",
        )
        .run();

    p.crabgo("tree --invert foo")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── dev-dep v0.1.0 ([..]/foo/dev-dep)
    [dev-dependencies]
    └── foo v0.1.0 ([..]/foo) (*)
",
        )
        .run();
}

#[crabgo_test]
fn invert() {
    Package::new("b1", "1.0.0").dep("c", "1.0").publish();
    Package::new("b2", "1.0.0").dep("d", "1.0").publish();
    Package::new("c", "1.0.0").publish();
    Package::new("d", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            b1 = "1.0"
            b2 = "1.0"
            c = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── b1 v1.0.0
│   └── c v1.0.0
├── b2 v1.0.0
│   └── d v1.0.0
└── c v1.0.0
",
        )
        .run();

    p.crabgo("tree --invert c")
        .with_stdout(
            "\
c v1.0.0
├── b1 v1.0.0
│   └── foo v0.1.0 ([..]/foo)
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn invert_with_build_dep() {
    // -i for a common dependency between normal and build deps.
    Package::new("common", "1.0.0").publish();
    Package::new("bdep", "1.0.0").dep("common", "1.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            common = "1.0"

            [build-dependencies]
            bdep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── common v1.0.0
[build-dependencies]
└── bdep v1.0.0
    └── common v1.0.0
",
        )
        .run();

    p.crabgo("tree -i common")
        .with_stdout(
            "\
common v1.0.0
├── bdep v1.0.0
│   [build-dependencies]
│   └── foo v0.1.0 ([..]/foo)
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn no_indent() {
    let p = make_simple_proj();

    p.crabgo("tree --prefix=none")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
a v1.0.0
b v1.0.0
c v1.0.0
c v1.0.0
bdep v1.0.0
b v1.0.0 (*)
devdep v1.0.0
b v1.0.0 (*)
",
        )
        .run();
}

#[crabgo_test]
fn prefix_depth() {
    let p = make_simple_proj();

    p.crabgo("tree --prefix=depth")
        .with_stdout(
            "\
0foo v0.1.0 ([..]/foo)
1a v1.0.0
2b v1.0.0
3c v1.0.0
1c v1.0.0
1bdep v1.0.0
2b v1.0.0 (*)
1devdep v1.0.0
2b v1.0.0 (*)
",
        )
        .run();
}

#[crabgo_test]
fn no_dedupe() {
    let p = make_simple_proj();

    p.crabgo("tree --no-dedupe")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
│   └── b v1.0.0
│       └── c v1.0.0
└── c v1.0.0
[build-dependencies]
└── bdep v1.0.0
    └── b v1.0.0
        └── c v1.0.0
[dev-dependencies]
└── devdep v1.0.0
    └── b v1.0.0
        └── c v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn no_dedupe_cycle() {
    // --no-dedupe with a dependency cycle
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dev-dependencies]
            bar = {path = "bar"}
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "bar/Crabgo.toml",
            r#"
            [package]
            name = "bar"
            version = "0.1.0"

            [dependencies]
            foo = {path=".."}
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
[dev-dependencies]
└── bar v0.1.0 ([..]/foo/bar)
    └── foo v0.1.0 ([..]/foo) (*)
",
        )
        .run();

    p.crabgo("tree --no-dedupe")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
[dev-dependencies]
└── bar v0.1.0 ([..]/foo/bar)
    └── foo v0.1.0 ([..]/foo) (*)
",
        )
        .run();
}

#[crabgo_test]
fn duplicates() {
    Package::new("dog", "1.0.0").publish();
    Package::new("dog", "2.0.0").publish();
    Package::new("cat", "1.0.0").publish();
    Package::new("cat", "2.0.0").publish();
    Package::new("dep", "1.0.0")
        .dep("dog", "1.0")
        .dep("cat", "1.0")
        .publish();
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

            [dependencies]
            dog1 = { version = "1.0", package = "dog" }
            dog2 = { version = "2.0", package = "dog" }
            "#,
        )
        .file("a/src/lib.rs", "")
        .file(
            "b/Crabgo.toml",
            r#"
            [package]
            name = "b"
            version = "0.1.0"

            [dependencies]
            dep = "1.0"
            cat = "2.0"
            "#,
        )
        .file("b/src/lib.rs", "")
        .build();

    p.crabgo("tree -p a")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo/a)
├── dog v1.0.0
└── dog v2.0.0
",
        )
        .run();

    p.crabgo("tree -p b")
        .with_stdout(
            "\
b v0.1.0 ([..]/foo/b)
├── cat v2.0.0
└── dep v1.0.0
    ├── cat v1.0.0
    └── dog v1.0.0
",
        )
        .run();

    p.crabgo("tree -p a -d")
        .with_stdout(
            "\
dog v1.0.0
└── a v0.1.0 ([..]/foo/a)

dog v2.0.0
└── a v0.1.0 ([..]/foo/a)
",
        )
        .run();

    p.crabgo("tree -p b -d")
        .with_stdout(
            "\
cat v1.0.0
└── dep v1.0.0
    └── b v0.1.0 ([..]/foo/b)

cat v2.0.0
└── b v0.1.0 ([..]/foo/b)
",
        )
        .run();
}

#[crabgo_test]
fn duplicates_with_target() {
    // --target flag
    if cross_compile::disabled() {
        return;
    }
    Package::new("a", "1.0.0").publish();
    Package::new("dog", "1.0.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            a = "1.0"
            dog = "1.0"

            [build-dependencies]
            a = "1.0"
            dog = "1.0"

            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();
    p.crabgo("tree -d").with_stdout("").run();

    p.crabgo("tree -d --target")
        .arg(alternate())
        .with_stdout("")
        .run();

    p.crabgo("tree -d --target")
        .arg(rustc_host())
        .with_stdout("")
        .run();

    p.crabgo("tree -d --target=all").with_stdout("").run();
}

#[crabgo_test]
fn duplicates_with_proc_macro() {
    Package::new("dupe-dep", "1.0.0").publish();
    Package::new("dupe-dep", "2.0.0").publish();
    Package::new("proc", "1.0.0")
        .proc_macro(true)
        .dep("dupe-dep", "1.0")
        .publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            proc = "1.0"
            dupe-dep = "2.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── dupe-dep v2.0.0
└── proc v1.0.0 (proc-macro)
    └── dupe-dep v1.0.0
",
        )
        .run();

    p.crabgo("tree --duplicates")
        .with_stdout(
            "\
dupe-dep v1.0.0
└── proc v1.0.0 (proc-macro)
    └── foo v0.1.0 ([..]/foo)

dupe-dep v2.0.0
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();

    p.crabgo("tree --duplicates --edges no-proc-macro")
        .with_stdout("")
        .run();
}

#[crabgo_test]
fn charset() {
    let p = make_simple_proj();
    p.crabgo("tree --charset ascii")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
|-- a v1.0.0
|   `-- b v1.0.0
|       `-- c v1.0.0
`-- c v1.0.0
[build-dependencies]
`-- bdep v1.0.0
    `-- b v1.0.0 (*)
[dev-dependencies]
`-- devdep v1.0.0
    `-- b v1.0.0 (*)
",
        )
        .run();
}

#[crabgo_test]
fn format() {
    Package::new("dep", "1.0.0").publish();
    Package::new("other-dep", "1.0.0").publish();

    Package::new("dep_that_is_awesome", "1.0.0")
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "dep_that_is_awesome"
                version = "1.0.0"

                [lib]
                name = "awesome_dep"
            "#,
        )
        .file("src/lib.rs", "pub struct Straw;")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"
            license = "MIT"
            repository = "https://github.com/rust-lang/crabgo"

            [dependencies]
            dep = {version="1.0", optional=true}
            other-dep = {version="1.0", optional=true}
            dep_that_is_awesome = {version="1.0", optional=true}


            [features]
            default = ["foo"]
            foo = ["bar"]
            bar = []
            "#,
        )
        .file("src/main.rs", "")
        .build();

    p.crabgo("tree --format <<<{p}>>>")
        .with_stdout("<<<foo v0.1.0 ([..]/foo)>>>")
        .run();

    p.crabgo("tree --format {}")
        .with_stderr(
            "\
[ERROR] tree format `{}` not valid

Caused by:
  unsupported pattern ``
",
        )
        .with_status(101)
        .run();

    p.crabgo("tree --format {p}-{{hello}}")
        .with_stdout("foo v0.1.0 ([..]/foo)-{hello}")
        .run();

    p.crabgo("tree --format")
        .arg("{p} {l} {r}")
        .with_stdout("foo v0.1.0 ([..]/foo) MIT https://github.com/rust-lang/crabgo")
        .run();

    p.crabgo("tree --format")
        .arg("{p} {f}")
        .with_stdout("foo v0.1.0 ([..]/foo) bar,default,foo")
        .run();

    p.crabgo("tree --all-features --format")
        .arg("{p} [{f}]")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo) [bar,default,dep,dep_that_is_awesome,foo,other-dep]
├── dep v1.0.0 []
├── dep_that_is_awesome v1.0.0 []
└── other-dep v1.0.0 []
",
        )
        .run();

    p.crabgo("tree")
        .arg("--features=other-dep,dep_that_is_awesome")
        .arg("--format={lib}")
        .with_stdout(
            "
├── awesome_dep
└── other_dep
",
        )
        .run();
}

#[crabgo_test]
fn dev_dep_feature() {
    // New feature resolver with optional dep
    Package::new("optdep", "1.0.0").publish();
    Package::new("bar", "1.0.0")
        .add_dep(Dependency::new("optdep", "1.0").optional(true))
        .publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dev-dependencies]
            bar = { version = "1.0", features = ["optdep"] }

            [dependencies]
            bar = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    // Old behavior.
    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── bar v1.0.0
    └── optdep v1.0.0
[dev-dependencies]
└── bar v1.0.0 (*)
",
        )
        .run();

    p.crabgo("tree -e normal")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── bar v1.0.0
    └── optdep v1.0.0
",
        )
        .run();

    // New behavior.
    switch_to_resolver_2(&p);

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── bar v1.0.0
    └── optdep v1.0.0
[dev-dependencies]
└── bar v1.0.0 (*)
",
        )
        .run();

    p.crabgo("tree -e normal")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── bar v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn host_dep_feature() {
    // New feature resolver with optional build dep
    Package::new("optdep", "1.0.0").publish();
    Package::new("bar", "1.0.0")
        .add_dep(Dependency::new("optdep", "1.0").optional(true))
        .publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [build-dependencies]
            bar = { version = "1.0", features = ["optdep"] }

            [dependencies]
            bar = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();

    // Old behavior
    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── bar v1.0.0
    └── optdep v1.0.0
[build-dependencies]
└── bar v1.0.0 (*)
",
        )
        .run();

    // -p
    p.crabgo("tree -p bar")
        .with_stdout(
            "\
bar v1.0.0
└── optdep v1.0.0
",
        )
        .run();

    // invert
    p.crabgo("tree -i optdep")
        .with_stdout(
            "\
optdep v1.0.0
└── bar v1.0.0
    └── foo v0.1.0 ([..]/foo)
    [build-dependencies]
    └── foo v0.1.0 ([..]/foo)
",
        )
        .run();

    // New behavior.
    switch_to_resolver_2(&p);

    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── bar v1.0.0
[build-dependencies]
└── bar v1.0.0
    └── optdep v1.0.0
",
        )
        .run();

    p.crabgo("tree -p bar")
        .with_stdout(
            "\
bar v1.0.0

bar v1.0.0
└── optdep v1.0.0
",
        )
        .run();

    p.crabgo("tree -i optdep")
        .with_stdout(
            "\
optdep v1.0.0
└── bar v1.0.0
    [build-dependencies]
    └── foo v0.1.0 ([..]/foo)
",
        )
        .run();

    // Check that -d handles duplicates with features.
    p.crabgo("tree -d")
        .with_stdout(
            "\
bar v1.0.0
└── foo v0.1.0 ([..]/foo)

bar v1.0.0
[build-dependencies]
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn proc_macro_features() {
    // New feature resolver with a proc-macro
    Package::new("optdep", "1.0.0").publish();
    Package::new("somedep", "1.0.0")
        .add_dep(Dependency::new("optdep", "1.0").optional(true))
        .publish();
    Package::new("pm", "1.0.0")
        .proc_macro(true)
        .feature_dep("somedep", "1.0", &["optdep"])
        .publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            pm = "1.0"
            somedep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    // Old behavior
    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── pm v1.0.0 (proc-macro)
│   └── somedep v1.0.0
│       └── optdep v1.0.0
└── somedep v1.0.0 (*)
",
        )
        .run();

    // Old behavior + no-proc-macro
    p.crabgo("tree -e no-proc-macro")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── somedep v1.0.0
    └── optdep v1.0.0
",
        )
        .run();

    // -p
    p.crabgo("tree -p somedep")
        .with_stdout(
            "\
somedep v1.0.0
└── optdep v1.0.0
",
        )
        .run();

    // -p -e no-proc-macro
    p.crabgo("tree -p somedep -e no-proc-macro")
        .with_stdout(
            "\
somedep v1.0.0
└── optdep v1.0.0
",
        )
        .run();

    // invert
    p.crabgo("tree -i somedep")
        .with_stdout(
            "\
somedep v1.0.0
├── foo v0.1.0 ([..]/foo)
└── pm v1.0.0 (proc-macro)
    └── foo v0.1.0 ([..]/foo)
",
        )
        .run();

    // invert + no-proc-macro
    p.crabgo("tree -i somedep -e no-proc-macro")
        .with_stdout(
            "\
somedep v1.0.0
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();

    // New behavior.
    switch_to_resolver_2(&p);

    // Note the missing (*)
    p.crabgo("tree")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── pm v1.0.0 (proc-macro)
│   └── somedep v1.0.0
│       └── optdep v1.0.0
└── somedep v1.0.0
",
        )
        .run();

    p.crabgo("tree -e no-proc-macro")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── somedep v1.0.0
",
        )
        .run();

    p.crabgo("tree -p somedep")
        .with_stdout(
            "\
somedep v1.0.0

somedep v1.0.0
└── optdep v1.0.0
",
        )
        .run();

    p.crabgo("tree -i somedep")
        .with_stdout(
            "\
somedep v1.0.0
└── foo v0.1.0 ([..]/foo)

somedep v1.0.0
└── pm v1.0.0 (proc-macro)
    └── foo v0.1.0 ([..]/foo)
",
        )
        .run();

    p.crabgo("tree -i somedep -e no-proc-macro")
        .with_stdout(
            "\
somedep v1.0.0
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn itarget_opt_dep() {
    // New feature resolver with optional target dep
    Package::new("optdep", "1.0.0").publish();
    Package::new("common", "1.0.0")
        .add_dep(Dependency::new("optdep", "1.0").optional(true))
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "1.0.0"

            [dependencies]
            common = "1.0"

            [target.'cfg(whatever)'.dependencies]
            common = { version = "1.0", features = ["optdep"] }

            "#,
        )
        .file("src/lib.rs", "")
        .build();

    // Old behavior
    p.crabgo("tree")
        .with_stdout(
            "\
foo v1.0.0 ([..]/foo)
└── common v1.0.0
    └── optdep v1.0.0
",
        )
        .run();

    // New behavior.
    switch_to_resolver_2(&p);

    p.crabgo("tree")
        .with_stdout(
            "\
foo v1.0.0 ([..]/foo)
└── common v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn ambiguous_name() {
    // -p that is ambiguous.
    Package::new("dep", "1.0.0").publish();
    Package::new("dep", "2.0.0").publish();
    Package::new("bar", "1.0.0").dep("dep", "2.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            dep = "1.0"
            bar = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree -p dep")
        .with_stderr_contains(
            "\
error: There are multiple `dep` packages in your project, and the specification `dep` is ambiguous.
Please re-run this command with `-p <spec>` where `<spec>` is one of the following:
  dep@1.0.0
  dep@2.0.0
",
        )
        .with_status(101)
        .run();
}

#[crabgo_test]
fn workspace_features_are_local() {
    // The features for workspace packages should be the same as `crabgo build`
    // (i.e., the features selected depend on the "current" package).
    Package::new("optdep", "1.0.0").publish();
    Package::new("somedep", "1.0.0")
        .add_dep(Dependency::new("optdep", "1.0").optional(true))
        .publish();
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

            [dependencies]
            somedep = {version="1.0", features=["optdep"]}
            "#,
        )
        .file("a/src/lib.rs", "")
        .file(
            "b/Crabgo.toml",
            r#"
            [package]
            name = "b"
            version = "0.1.0"

            [dependencies]
            somedep = "1.0"
            "#,
        )
        .file("b/src/lib.rs", "")
        .build();

    p.crabgo("tree")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo/a)
└── somedep v1.0.0
    └── optdep v1.0.0

b v0.1.0 ([..]/foo/b)
└── somedep v1.0.0 (*)
",
        )
        .run();

    p.crabgo("tree -p a")
        .with_stdout(
            "\
a v0.1.0 ([..]/foo/a)
└── somedep v1.0.0
    └── optdep v1.0.0
",
        )
        .run();

    p.crabgo("tree -p b")
        .with_stdout(
            "\
b v0.1.0 ([..]/foo/b)
└── somedep v1.0.0
",
        )
        .run();
}

#[crabgo_test]
fn unknown_edge_kind() {
    let p = project()
        .file("Crabgo.toml", "")
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree -e unknown")
        .with_stderr(
            "\
[ERROR] unknown edge kind `unknown`, valid values are \
\"normal\", \"build\", \"dev\", \
\"no-normal\", \"no-build\", \"no-dev\", \"no-proc-macro\", \
\"features\", or \"all\"
",
        )
        .with_status(101)
        .run();
}

#[crabgo_test]
fn mixed_no_edge_kinds() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree -e no-build,normal")
        .with_stderr(
            "\
[ERROR] `normal` dependency kind cannot be mixed with \
\"no-normal\", \"no-build\", or \"no-dev\" dependency kinds
",
        )
        .with_status(101)
        .run();

    // `no-proc-macro` can be mixed with others
    p.crabgo("tree -e no-proc-macro,normal")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn depth_limit() {
    let p = make_simple_proj();

    p.crabgo("tree --depth 0")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
[build-dependencies]
[dev-dependencies]
",
        )
        .run();

    p.crabgo("tree --depth 1")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
└── c v1.0.0
[build-dependencies]
└── bdep v1.0.0
[dev-dependencies]
└── devdep v1.0.0
",
        )
        .run();

    p.crabgo("tree --depth 2")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
│   └── b v1.0.0
└── c v1.0.0
[build-dependencies]
└── bdep v1.0.0
    └── b v1.0.0 (*)
[dev-dependencies]
└── devdep v1.0.0
    └── b v1.0.0 (*)
",
        )
        .run();

    // specify a package
    p.crabgo("tree -p bdep --depth 1")
        .with_stdout(
            "\
bdep v1.0.0
└── b v1.0.0
",
        )
        .run();

    // different prefix
    p.crabgo("tree --depth 1 --prefix depth")
        .with_stdout(
            "\
0foo v0.1.0 ([..]/foo)
1a v1.0.0
1c v1.0.0
1bdep v1.0.0
1devdep v1.0.0
",
        )
        .run();

    // with edge-kinds
    p.crabgo("tree --depth 1 -e no-dev")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
└── c v1.0.0
[build-dependencies]
└── bdep v1.0.0
",
        )
        .run();

    // invert
    p.crabgo("tree --depth 1 --invert c")
        .with_stdout(
            "\
c v1.0.0
├── b v1.0.0
└── foo v0.1.0 ([..]/foo)
",
        )
        .run();
}

#[crabgo_test]
fn prune() {
    let p = make_simple_proj();

    p.crabgo("tree --prune c")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── a v1.0.0
    └── b v1.0.0
[build-dependencies]
└── bdep v1.0.0
    └── b v1.0.0 (*)
[dev-dependencies]
└── devdep v1.0.0
    └── b v1.0.0 (*)
",
        )
        .run();

    // multiple prune
    p.crabgo("tree --prune c --prune bdep")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── a v1.0.0
    └── b v1.0.0
[build-dependencies]
[dev-dependencies]
└── devdep v1.0.0
    └── b v1.0.0 (*)
",
        )
        .run();

    // with edge-kinds
    p.crabgo("tree --prune c -e normal")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
└── a v1.0.0
    └── b v1.0.0
",
        )
        .run();

    // pruning self does not works
    p.crabgo("tree --prune foo")
        .with_stdout(
            "\
foo v0.1.0 ([..]/foo)
├── a v1.0.0
│   └── b v1.0.0
│       └── c v1.0.0
└── c v1.0.0
[build-dependencies]
└── bdep v1.0.0
    └── b v1.0.0 (*)
[dev-dependencies]
└── devdep v1.0.0
    └── b v1.0.0 (*)
",
        )
        .run();

    // dep not exist
    p.crabgo("tree --prune no-dep")
        .with_stderr(
            "\
[ERROR] package ID specification `no-dep` did not match any packages

<tab>Did you mean `bdep`?
",
        )
        .with_status(101)
        .run();
}

#[crabgo_test]
fn cyclic_features() {
    // Check for stack overflow with cyclic features (oops!).
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "1.0.0"

                [features]
                a = ["b"]
                b = ["a"]
                default = ["a"]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("tree -e features")
        .with_stdout("foo v1.0.0 ([ROOT]/foo)")
        .run();

    p.crabgo("tree -e features -i foo")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
├── foo feature \"a\"
│   ├── foo feature \"b\"
│   │   └── foo feature \"a\" (*)
│   └── foo feature \"default\" (command-line)
├── foo feature \"b\" (*)
└── foo feature \"default\" (command-line)
",
        )
        .run();
}

#[crabgo_test]
fn dev_dep_cycle_with_feature() {
    // Cycle with features and a dev-dependency.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "1.0.0"

                [dev-dependencies]
                bar = { path = "bar" }

                [features]
                a = ["bar/feat1"]
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "1.0.0"

                [dependencies]
                foo = { path = ".." }

                [features]
                feat1 = ["foo/a"]
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("tree -e features --features a")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
[dev-dependencies]
└── bar feature \"default\"
    └── bar v1.0.0 ([ROOT]/foo/bar)
        └── foo feature \"default\" (command-line)
            └── foo v1.0.0 ([ROOT]/foo) (*)
",
        )
        .run();

    p.crabgo("tree -e features --features a -i foo")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
├── foo feature \"a\" (command-line)
│   └── bar feature \"feat1\"
│       └── foo feature \"a\" (command-line) (*)
└── foo feature \"default\" (command-line)
    └── bar v1.0.0 ([ROOT]/foo/bar)
        ├── bar feature \"default\"
        │   [dev-dependencies]
        │   └── foo v1.0.0 ([ROOT]/foo) (*)
        └── bar feature \"feat1\" (*)
",
        )
        .run();
}

#[crabgo_test]
fn dev_dep_cycle_with_feature_nested() {
    // Checks for an issue where a cyclic dev dependency tries to activate a
    // feature on its parent that tries to activate the feature back on the
    // dev-dependency.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "1.0.0"

                [dev-dependencies]
                bar = { path = "bar" }

                [features]
                a = ["bar/feat1"]
                b = ["a"]
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "1.0.0"

                [dependencies]
                foo = { path = ".." }

                [features]
                feat1 = ["foo/b"]
            "#,
        )
        .file("bar/src/lib.rs", "")
        .build();

    p.crabgo("tree -e features")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
[dev-dependencies]
└── bar feature \"default\"
    └── bar v1.0.0 ([ROOT]/foo/bar)
        └── foo feature \"default\" (command-line)
            └── foo v1.0.0 ([ROOT]/foo) (*)
",
        )
        .run();

    p.crabgo("tree -e features --features a -i foo")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
├── foo feature \"a\" (command-line)
│   └── foo feature \"b\"
│       └── bar feature \"feat1\"
│           └── foo feature \"a\" (command-line) (*)
├── foo feature \"b\" (*)
└── foo feature \"default\" (command-line)
    └── bar v1.0.0 ([ROOT]/foo/bar)
        ├── bar feature \"default\"
        │   [dev-dependencies]
        │   └── foo v1.0.0 ([ROOT]/foo) (*)
        └── bar feature \"feat1\" (*)
",
        )
        .run();

    p.crabgo("tree -e features --features b -i foo")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
├── foo feature \"a\"
│   └── foo feature \"b\" (command-line)
│       └── bar feature \"feat1\"
│           └── foo feature \"a\" (*)
├── foo feature \"b\" (command-line) (*)
└── foo feature \"default\" (command-line)
    └── bar v1.0.0 ([ROOT]/foo/bar)
        ├── bar feature \"default\"
        │   [dev-dependencies]
        │   └── foo v1.0.0 ([ROOT]/foo) (*)
        └── bar feature \"feat1\" (*)
",
        )
        .run();

    p.crabgo("tree -e features --features bar/feat1 -i foo")
        .with_stdout(
            "\
foo v1.0.0 ([ROOT]/foo)
├── foo feature \"a\"
│   └── foo feature \"b\"
│       └── bar feature \"feat1\" (command-line)
│           └── foo feature \"a\" (*)
├── foo feature \"b\" (*)
└── foo feature \"default\" (command-line)
    └── bar v1.0.0 ([ROOT]/foo/bar)
        ├── bar feature \"default\"
        │   [dev-dependencies]
        │   └── foo v1.0.0 ([ROOT]/foo) (*)
        └── bar feature \"feat1\" (command-line) (*)
",
        )
        .run();
}
