//! Tests for -Zcheck-cfg.

use crabgo_test_support::{basic_manifest, project};

macro_rules! x {
    ($tool:tt => $what:tt $(of $who:tt)?) => {{
        #[cfg(windows)]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg ",
                    $what, '(', $($who,)* ')', "[..]")
        }
        #[cfg(not(windows))]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg '",
                    $what, '(', $($who,)* ')', "'", "[..]")
        }
    }};
    ($tool:tt => $what:tt of $who:tt with $($values:tt)*) => {{
        #[cfg(windows)]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg \"",
                    $what, '(', $who, $(", ", "/\"", $values, "/\"",)* ")", '"', "[..]")
        }
        #[cfg(not(windows))]
        {
            concat!("[RUNNING] [..]", $tool, "[..] --check-cfg '",
                    $what, '(', $who, $(", ", "\"", $values, "\"",)* ")", "'", "[..]")
        }
    }};
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_with_deps() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = { path = "bar/" }

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "#[allow(dead_code)] fn bar() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_with_opt_deps() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = { path = "bar/", optional = true }

                [features]
                default = ["bar"]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "#[allow(dead_code)] fn bar() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "bar" "default" "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_with_namespaced_features() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = { path = "bar/", optional = true }

                [features]
                f_a = ["dep:bar"]
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "#[allow(dead_code)] fn bar() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_names() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=names")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_values() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=values")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn cli_all_options() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=features,names,values")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .with_stderr_contains(x!("rustc" => "values"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_with_crabgo_check() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_names_with_check() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=names")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_values_with_check() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check -v -Zcheck-cfg=values")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_test() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("test -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_doctest() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                default = ["f_a"]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.crabgo("test -v --doc -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "default" "f_a" "f_b"))
        .with_stderr_contains(x!("rustdoc" => "values" of "feature" with "default" "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_names_test() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("test -v -Zcheck-cfg=names")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_values_test() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("test -v -Zcheck-cfg=values")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_names_doctest() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.crabgo("test -v --doc -Zcheck-cfg=names")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .with_stderr_contains(x!("rustdoc" => "names"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn well_known_values_doctest() {
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.crabgo("test -v --doc -Zcheck-cfg=values")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "values"))
        .with_stderr_contains(x!("rustdoc" => "values"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn features_doc() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                default = ["f_a"]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/lib.rs", "#[allow(dead_code)] fn foo() {}")
        .build();

    p.crabgo("doc -v -Zcheck-cfg=features")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustdoc" => "values" of "feature" with "default" "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn build_script_feedback() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"fn main() { println!("crabgo:rustc-check-cfg=names(foo)"); }"#,
        )
        .build();

    p.crabgo("check -v -Zcheck-cfg=output")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names" of "foo"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn build_script_doc() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"fn main() { println!("crabgo:rustc-check-cfg=names(foo)"); }"#,
        )
        .build();
    p.crabgo("doc -v -Zcheck-cfg=output")
        .with_stderr_does_not_contain("rustc [..] --check-cfg [..]")
        .with_stderr_contains(x!("rustdoc" => "names" of "foo"))
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc [..] build.rs [..]`
[RUNNING] `[..]/build-script-build`
[DOCUMENTING] foo [..]
[RUNNING] `rustdoc [..] src/main.rs [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn build_script_override() {
    let target = crabgo_test_support::rustc_host();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.5.0"
                authors = []
                links = "a"
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("build.rs", "")
        .file(
            ".crabgo/config",
            &format!(
                r#"
                    [target.{}.a]
                    rustc-check-cfg = ["names(foo)"]
                "#,
                target
            ),
        )
        .build();

    p.crabgo("check -v -Zcheck-cfg=output")
        .with_stderr_contains(x!("rustc" => "names" of "foo"))
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn build_script_test() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file(
            "build.rs",
            r#"fn main() { 
                println!("crabgo:rustc-check-cfg=names(foo)");
                println!("crabgo:rustc-cfg=foo");
            }"#,
        )
        .file(
            "src/lib.rs",
            r#"
                ///
                /// ```
                /// extern crate foo;
                ///
                /// fn main() {
                ///     foo::foo()
                /// }
                /// ```
                ///
                #[cfg(foo)]
                pub fn foo() {}

                #[cfg(foo)]
                #[test]
                fn test_foo() {
                    foo()
                }
            "#,
        )
        .file("tests/test.rs", "#[cfg(foo)] #[test] fn test_bar() {}")
        .build();

    p.crabgo("test -v -Zcheck-cfg=output")
        .with_stderr_contains(x!("rustc" => "names" of "foo"))
        .with_stderr_contains(x!("rustdoc" => "names" of "foo"))
        .with_stdout_contains("test test_foo ... ok")
        .with_stdout_contains("test test_bar ... ok")
        .with_stdout_contains_n("test [..] ... ok", 3)
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn config_valid() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [features]
                f_a = []
                f_b = []
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            ".crabgo/config.toml",
            r#"
                [unstable]
                check-cfg = ["features", "names", "values"]
            "#,
        )
        .build();

    p.crabgo("check -v -Zcheck-cfg=features,names,values")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains(x!("rustc" => "names"))
        .with_stderr_contains(x!("rustc" => "values"))
        .with_stderr_contains(x!("rustc" => "values" of "feature" with "f_a" "f_b"))
        .run();
}

#[crabgo_test(nightly, reason = "--check-cfg is unstable")]
fn config_invalid() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            ".crabgo/config.toml",
            r#"
                [unstable]
                check-cfg = ["va"]
            "#,
        )
        .build();

    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["check-cfg"])
        .with_stderr_contains("error: unstable check-cfg only takes `features`, `names`, `values` or `output` as valid inputs")
        .with_status(101)
        .run();
}
