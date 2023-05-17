//! Tests for the `crabgo doc` command with `-Zrustdoc-scrape-examples`.

use crabgo_test_support::project;

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn basic() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
            "#,
        )
        .file("examples/ex.rs", "fn main() { foo::foo(); }")
        .file("src/lib.rs", "pub fn foo() {}\npub fn bar() { foo(); }")
        .build();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[SCRAPING] foo v0.0.1 ([CWD])
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("doc -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr("[FINISHED] [..]")
        .run();

    let doc_html = p.read_file("target/doc/foo/fn.foo.html");
    assert!(doc_html.contains("Examples found in repository"));
    assert!(!doc_html.contains("More examples"));

    // Ensure that the reverse-dependency has its sources generated
    assert!(p.build_dir().join("doc/src/ex/ex.rs.html").exists());
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn avoid_build_script_cycle() {
    let p = project()
        // package with build dependency
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                links = "foo"

                [workspace]
                members = ["bar"]

                [build-dependencies]
                bar = {path = "bar"}
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main(){}")
        // dependency
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []
                links = "bar"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .file("bar/build.rs", "fn main(){}")
        .build();

    p.crabgo("doc --workspace -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn complex_reverse_dependencies() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dev-dependencies]
                a = {path = "a", features = ["feature"]}
                b = {path = "b"}

                [workspace]
                members = ["b"]
            "#,
        )
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "fn main() {}")
        .file(
            "a/Crabgo.toml",
            r#"
                [package]
                name = "a"
                version = "0.0.1"
                authors = []

                [lib]
                proc-macro = true

                [dependencies]
                b = {path = "../b"}

                [features]
                feature = []
            "#,
        )
        .file("a/src/lib.rs", "")
        .file(
            "b/Crabgo.toml",
            r#"
                [package]
                name = "b"
                version = "0.0.1"
                authors = []
            "#,
        )
        .file("b/src/lib.rs", "")
        .build();

    p.crabgo("doc --workspace --examples -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn crate_with_dash() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "da-sh"
                version = "0.0.1"
                authors = []
            "#,
        )
        .file("src/lib.rs", "pub fn foo() {}")
        .file("examples/a.rs", "fn main() { da_sh::foo(); }")
        .build();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();

    let doc_html = p.read_file("target/doc/da_sh/fn.foo.html");
    assert!(doc_html.contains("Examples found in repository"));
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn configure_target() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [lib]
                doc-scrape-examples = true

                [[bin]]
                name = "a_bin"
                doc-scrape-examples = true

                [[example]]
                name = "a"
                doc-scrape-examples = false
            "#,
        )
        .file(
            "src/lib.rs",
            "pub fn foo() {} fn lib_must_appear() { foo(); }",
        )
        .file(
            "examples/a.rs",
            "fn example_must_not_appear() { foo::foo(); }",
        )
        .file(
            "src/bin/a_bin.rs",
            "fn bin_must_appear() { foo::foo(); } fn main(){}",
        )
        .build();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();

    let doc_html = p.read_file("target/doc/foo/fn.foo.html");
    assert!(doc_html.contains("lib_must_appear"));
    assert!(doc_html.contains("bin_must_appear"));
    assert!(!doc_html.contains("example_must_not_appear"));
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn configure_profile_issue_10500() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [profile.dev]
                panic = "abort"
            "#,
        )
        .file("examples/ex.rs", "fn main() { foo::foo(); }")
        .file("src/lib.rs", "pub fn foo() {}\npub fn bar() { foo(); }")
        .build();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();

    let doc_html = p.read_file("target/doc/foo/fn.foo.html");
    assert!(doc_html.contains("Examples found in repository"));
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn issue_10545() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                resolver = "2"
                members = ["a", "b"]
            "#,
        )
        .file(
            "a/Crabgo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []
            edition = "2021"

            [features]
            default = ["foo"]
            foo = []
        "#,
        )
        .file("a/src/lib.rs", "")
        .file(
            "b/Crabgo.toml",
            r#"
                [package]
                name = "b"
                version = "0.0.1"
                authors = []
                edition = "2021"

                [lib]
                proc-macro = true
            "#,
        )
        .file("b/src/lib.rs", "")
        .build();

    p.crabgo("doc --workspace -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn cache() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
            "#,
        )
        .file("examples/ex.rs", "fn main() { foo::foo(); }")
        .file("src/lib.rs", "pub fn foo() {}\npub fn bar() { foo(); }")
        .build();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[SCRAPING] foo v0.0.1 ([CWD])
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr(
            "\
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn no_fail_bad_lib() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
            "#,
        )
        .file("src/lib.rs", "pub fn foo() { CRASH_THE_BUILD() }")
        .file("examples/ex.rs", "fn main() { foo::foo(); }")
        .file("examples/ex2.rs", "fn main() { foo::foo(); }")
        .build();

    p.crabgo("doc -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr_unordered(
        "\
[CHECKING] foo v0.0.1 ([CWD])
[SCRAPING] foo v0.0.1 ([CWD])
warning: failed to check lib in package `foo` as a prerequisite for scraping examples from: example \"ex\", example \"ex2\"
    Try running with `--verbose` to see the error message.
    If an example should not be scanned, then consider adding `doc-scrape-examples = false` to its `[[example]]` definition in Crabgo.toml
warning: `foo` (lib) generated 1 warning
warning: failed to scan example \"ex\" in package `foo` for example code usage
    Try running with `--verbose` to see the error message.
    If an example should not be scanned, then consider adding `doc-scrape-examples = false` to its `[[example]]` definition in Crabgo.toml
warning: `foo` (example \"ex\") generated 1 warning
warning: failed to scan example \"ex2\" in package `foo` for example code usage
    Try running with `--verbose` to see the error message.
    If an example should not be scanned, then consider adding `doc-scrape-examples = false` to its `[[example]]` definition in Crabgo.toml
warning: `foo` (example \"ex2\") generated 1 warning
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
    )
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn fail_bad_build_script() {
    // See rust-lang/crabgo#11623
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() { panic!(\"You shall not pass\")}")
        .file("examples/ex.rs", "fn main() {}")
        .build();

    // `crabgo doc` fails
    p.crabgo("doc")
        .with_status(101)
        .with_stderr_contains("[..]You shall not pass[..]")
        .run();

    // scrape examples should fail whenever `crabgo doc` fails.
    p.crabgo("doc -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_status(101)
        .with_stderr_contains("[..]You shall not pass[..]")
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn no_fail_bad_example() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
            "#,
        )
        .file("examples/ex1.rs", "DOES NOT COMPILE")
        .file("examples/ex2.rs", "fn main() { foo::foo(); }")
        .file("src/lib.rs", "pub fn foo(){}")
        .build();

    p.crabgo("doc -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[SCRAPING] foo v0.0.1 ([CWD])
warning: failed to scan example \"ex1\" in package `foo` for example code usage
    Try running with `--verbose` to see the error message.
    If an example should not be scanned, then consider adding `doc-scrape-examples = false` to its `[[example]]` definition in Crabgo.toml
warning: `foo` (example \"ex1\") generated 1 warning
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .run();

    p.crabgo("clean").run();

    p.crabgo("doc -v -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr_unordered(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name foo[..]
[SCRAPING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc[..] --crate-name ex1[..]
[RUNNING] `rustdoc[..] --crate-name ex2[..]
[RUNNING] `rustdoc[..] --crate-name foo[..]
error: expected one of `!` or `::`, found `NOT`
 --> examples/ex1.rs:1:6
  |
1 | DOES NOT COMPILE
  |      ^^^ expected one of `!` or `::`

[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .run();

    let doc_html = p.read_file("target/doc/foo/fn.foo.html");
    assert!(doc_html.contains("Examples found in repository"));
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn no_scrape_with_dev_deps() {
    // Tests that a crate with dev-dependencies does not have its examples
    // scraped unless explicitly prompted to check them. See
    // `UnitGenerator::create_docscrape_proposals` for details on why.

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dev-dependencies]
            a = {path = "a"}
        "#,
        )
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "fn main() { a::f(); }")
        .file(
            "a/Crabgo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []
        "#,
        )
        .file("a/src/lib.rs", "pub fn f() {}")
        .build();

    // If --examples is not provided, then the example is not scanned, and a warning
    // should be raised.
    p.crabgo("doc -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr(
            "\
warning: Rustdoc did not scrape the following examples because they require dev-dependencies: ex
    If you want Rustdoc to scrape these examples, then add `doc-scrape-examples = true`
    to the [[example]] target configuration of at least one example.
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .run();

    // If --examples is provided, then the example is scanned.
    p.crabgo("doc --examples -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr_unordered(
            "\
[CHECKING] a v0.0.1 ([CWD]/a)
[CHECKING] foo v0.0.1 ([CWD])
[DOCUMENTING] a v0.0.1 ([CWD]/a)
[SCRAPING] foo v0.0.1 ([CWD])
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn use_dev_deps_if_explicitly_enabled() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [[example]]
            name = "ex"
            doc-scrape-examples = true

            [dev-dependencies]
            a = {path = "a"}
        "#,
        )
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "fn main() { a::f(); }")
        .file(
            "a/Crabgo.toml",
            r#"
            [package]
            name = "a"
            version = "0.0.1"
            authors = []
        "#,
        )
        .file("a/src/lib.rs", "pub fn f() {}")
        .build();

    // If --examples is not provided, then the example is never scanned.
    p.crabgo("doc -Zunstable-options -Z rustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .with_stderr_unordered(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[CHECKING] a v0.0.1 ([CWD]/a)
[SCRAPING] foo v0.0.1 ([CWD])
[DOCUMENTING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .run();
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn only_scrape_documented_targets() {
    // package bar has doc = false and should not be eligible for documtation.
    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
            [package]
            name = "bar"
            version = "0.0.1"
            authors = []            

            [lib]
            doc = false

            [workspace]
            members = ["foo"]

            [dependencies]
            foo = {{ path = "foo" }}
        "#
            ),
        )
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "pub fn main() { foo::foo(); }")
        .file(
            "foo/Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []      
        "#,
        )
        .file("foo/src/lib.rs", "pub fn foo() {}")
        .build();

    p.crabgo("doc --workspace -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();

    let doc_html = p.read_file("target/doc/foo/fn.foo.html");
    let example_found = doc_html.contains("Examples found in repository");
    assert!(!example_found);
}

#[crabgo_test(nightly, reason = "rustdoc scrape examples flags are unstable")]
fn issue_11496() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "repro"
                version = "0.1.0"
                edition = "2021"
                
                [lib]
                proc-macro = true
            "#,
        )
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "fn main(){}")
        .build();

    p.crabgo("doc -Zunstable-options -Zrustdoc-scrape-examples")
        .masquerade_as_nightly_crabgo(&["rustdoc-scrape-examples"])
        .run();
}
