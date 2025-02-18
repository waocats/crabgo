//! Tests for the `crabgo rustdoc` command.

use crabgo_test_support::{basic_manifest, cross_compile, project};

#[crabgo_test]
fn rustdoc_simple() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("rustdoc -v")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc [..]--crate-name foo src/lib.rs [..]\
        -o [CWD]/target/doc \
        [..] \
        -L dependency=[CWD]/target/debug/deps [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustdoc_args() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("rustdoc -v -- --cfg=foo")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc [..]--crate-name foo src/lib.rs [..]\
        -o [CWD]/target/doc \
        [..] \
        --cfg=foo \
        -C metadata=[..] \
        -L dependency=[CWD]/target/debug/deps [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustdoc_binary_args_passed() {
    let p = project().file("src/main.rs", "").build();

    p.crabgo("rustdoc -v")
        .arg("--")
        .arg("--markdown-no-toc")
        .with_stderr_contains("[RUNNING] `rustdoc [..] --markdown-no-toc[..]`")
        .run();
}

#[crabgo_test]
fn rustdoc_foo_with_bar_dependency() {
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "extern crate bar; pub fn foo() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.crabgo("rustdoc -v -- --cfg=foo")
        .with_stderr(
            "\
[CHECKING] bar v0.0.1 ([..])
[RUNNING] `rustc [..]bar/src/lib.rs [..]`
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc [..]--crate-name foo src/lib.rs [..]\
        -o [CWD]/target/doc \
        [..] \
        --cfg=foo \
        -C metadata=[..] \
        -L dependency=[CWD]/target/debug/deps \
        --extern [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustdoc_only_bar_dependency() {
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/main.rs", "extern crate bar; fn main() { bar::baz() }")
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.crabgo("rustdoc -v -p bar -- --cfg=foo")
        .with_stderr(
            "\
[DOCUMENTING] bar v0.0.1 ([..])
[RUNNING] `rustdoc [..]--crate-name bar [..]bar/src/lib.rs [..]\
        -o [CWD]/target/doc \
        [..] \
        --cfg=foo \
        -C metadata=[..] \
        -L dependency=[CWD]/target/debug/deps [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustdoc_same_name_documents_lib() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file("src/lib.rs", r#" "#)
        .build();

    p.crabgo("rustdoc -v -- --cfg=foo")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([..])
[RUNNING] `rustdoc [..]--crate-name foo src/lib.rs [..]\
        -o [CWD]/target/doc \
        [..] \
        --cfg=foo \
        -C metadata=[..] \
        -L dependency=[CWD]/target/debug/deps [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [features]
                quux = []
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("rustdoc --verbose --features quux")
        .with_stderr_contains("[..]feature=[..]quux[..]")
        .run();
}

#[crabgo_test]
fn proc_macro_crate_type() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [lib]
                proc-macro = true

            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("rustdoc --verbose")
        .with_stderr_contains(
            "\
[RUNNING] `rustdoc --crate-type proc-macro [..]`
",
        )
        .run();
}

#[crabgo_test]
fn rustdoc_target() {
    if cross_compile::disabled() {
        return;
    }

    let p = project().file("src/lib.rs", "").build();

    p.crabgo("rustdoc --verbose --target")
        .arg(cross_compile::alternate())
        .with_stderr(format!(
            "\
[DOCUMENTING] foo v0.0.1 ([..])
[RUNNING] `rustdoc [..]--crate-name foo src/lib.rs [..]\
    --target {target} \
    -o [CWD]/target/{target}/doc \
    [..] \
    -L dependency=[CWD]/target/{target}/debug/deps \
    -L dependency=[CWD]/target/debug/deps[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
            target = cross_compile::alternate()
        ))
        .run();
}

#[crabgo_test]
fn fail_with_glob() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                members = ["bar"]
            "#,
        )
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {  break_the_build(); }")
        .build();

    p.crabgo("rustdoc -p '*z'")
        .with_status(101)
        .with_stderr("[ERROR] Glob patterns on package selection are not supported.")
        .run();
}
