//! Tests for profiles.

use crabgo_test_support::project;
use crabgo_test_support::registry::Package;
use std::env;

#[crabgo_test]
fn profile_overrides() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]

                name = "test"
                version = "0.0.0"
                authors = []

                [profile.dev]
                opt-level = 1
                debug = false
                rpath = true
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.crabgo("build -v")
        .with_stderr(
            "\
[COMPILING] test v0.0.0 ([CWD])
[RUNNING] `rustc --crate-name test src/lib.rs [..]--crate-type lib \
        --emit=[..]link[..]\
        -C opt-level=1[..]\
        -C debug-assertions=on \
        -C metadata=[..] \
        -C rpath \
        --out-dir [..] \
        -L dependency=[CWD]/target/debug/deps`
[FINISHED] dev [optimized] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn opt_level_override_0() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]

                name = "test"
                version = "0.0.0"
                authors = []

                [profile.dev]
                opt-level = 0
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.crabgo("build -v")
        .with_stderr(
            "\
[COMPILING] test v0.0.0 ([CWD])
[RUNNING] `rustc --crate-name test src/lib.rs [..]--crate-type lib \
        --emit=[..]link[..]\
        -C debuginfo=2 \
        -C metadata=[..] \
        --out-dir [..] \
        -L dependency=[CWD]/target/debug/deps`
[FINISHED] [..] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn debug_override_1() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "test"
                version = "0.0.0"
                authors = []

                [profile.dev]
                debug = 1
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.crabgo("build -v")
        .with_stderr(
            "\
[COMPILING] test v0.0.0 ([CWD])
[RUNNING] `rustc --crate-name test src/lib.rs [..]--crate-type lib \
        --emit=[..]link[..]\
        -C debuginfo=1 \
        -C metadata=[..] \
        --out-dir [..] \
        -L dependency=[CWD]/target/debug/deps`
[FINISHED] [..] target(s) in [..]
",
        )
        .run();
}

fn check_opt_level_override(profile_level: &str, rustc_level: &str) {
    let p = project()
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                    [package]

                    name = "test"
                    version = "0.0.0"
                    authors = []

                    [profile.dev]
                    opt-level = {level}
                "#,
                level = profile_level
            ),
        )
        .file("src/lib.rs", "")
        .build();
    p.crabgo("build -v")
        .with_stderr(&format!(
            "\
[COMPILING] test v0.0.0 ([CWD])
[RUNNING] `rustc --crate-name test src/lib.rs [..]--crate-type lib \
        --emit=[..]link \
        -C opt-level={level}[..]\
        -C debuginfo=2 \
        -C debug-assertions=on \
        -C metadata=[..] \
        --out-dir [..] \
        -L dependency=[CWD]/target/debug/deps`
[FINISHED] [..] target(s) in [..]
",
            level = rustc_level
        ))
        .run();
}

#[crabgo_test]
fn opt_level_overrides() {
    for &(profile_level, rustc_level) in &[
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("\"s\"", "s"),
        ("\"z\"", "z"),
    ] {
        check_opt_level_override(profile_level, rustc_level)
    }
}

#[crabgo_test]
fn top_level_overrides_deps() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]

                name = "test"
                version = "0.0.0"
                authors = []

                [profile.release]
                opt-level = 1
                debug = true

                [dependencies.foo]
                path = "foo"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Crabgo.toml",
            r#"
                [package]

                name = "foo"
                version = "0.0.0"
                authors = []

                [profile.release]
                opt-level = 0
                debug = false

                [lib]
                name = "foo"
                crate_type = ["dylib", "rlib"]
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();
    p.crabgo("build -v --release")
        .with_stderr(&format!(
            "\
[COMPILING] foo v0.0.0 ([CWD]/foo)
[RUNNING] `rustc --crate-name foo foo/src/lib.rs [..]\
        --crate-type dylib --crate-type rlib \
        --emit=[..]link \
        -C prefer-dynamic \
        -C opt-level=1[..]\
        -C debuginfo=2 \
        -C metadata=[..] \
        --out-dir [CWD]/target/release/deps \
        -L dependency=[CWD]/target/release/deps`
[COMPILING] test v0.0.0 ([CWD])
[RUNNING] `rustc --crate-name test src/lib.rs [..]--crate-type lib \
        --emit=[..]link \
        -C opt-level=1[..]\
        -C debuginfo=2 \
        -C metadata=[..] \
        --out-dir [..] \
        -L dependency=[CWD]/target/release/deps \
        --extern foo=[CWD]/target/release/deps/\
                     {prefix}foo[..]{suffix} \
        --extern foo=[CWD]/target/release/deps/libfoo.rlib`
[FINISHED] release [optimized + debuginfo] target(s) in [..]
",
            prefix = env::consts::DLL_PREFIX,
            suffix = env::consts::DLL_SUFFIX
        ))
        .run();
}

#[crabgo_test]
fn profile_in_non_root_manifest_triggers_a_warning() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["bar"]

                [profile.dev]
                debug = false
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []
                workspace = ".."

                [profile.dev]
                opt-level = 1
            "#,
        )
        .file("bar/src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build -v")
        .cwd("bar")
        .with_stderr(
            "\
[WARNING] profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   [..]
workspace: [..]
[COMPILING] bar v0.1.0 ([..])
[RUNNING] `rustc [..]`
[FINISHED] dev [unoptimized] target(s) in [..]",
        )
        .run();
}

#[crabgo_test]
fn profile_in_virtual_manifest_works() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                members = ["bar"]

                [profile.dev]
                opt-level = 1
                debug = false
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []
                workspace = ".."
            "#,
        )
        .file("bar/src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build -v")
        .cwd("bar")
        .with_stderr(
            "\
[COMPILING] bar v0.1.0 ([..])
[RUNNING] `rustc [..]`
[FINISHED] dev [optimized] target(s) in [..]",
        )
        .run();
}

#[crabgo_test]
fn profile_lto_string_bool_dev() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [profile.dev]
                lto = "true"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("build")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[ROOT]/foo/Crabgo.toml`

Caused by:
  `lto` setting of string `\"true\"` for `dev` profile is not a valid setting, \
must be a boolean (`true`/`false`) or a string (`\"thin\"`/`\"fat\"`/`\"off\"`) or omitted.
",
        )
        .run();
}

#[crabgo_test]
fn profile_panic_test_bench() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [profile.test]
                panic = "abort"

                [profile.bench]
                panic = "abort"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("build")
        .with_stderr_contains(
            "\
[WARNING] `panic` setting is ignored for `bench` profile
[WARNING] `panic` setting is ignored for `test` profile
",
        )
        .run();
}

#[crabgo_test]
fn profile_doc_deprecated() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [profile.doc]
                opt-level = 0
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("build")
        .with_stderr_contains("[WARNING] profile `doc` is deprecated and has no effect")
        .run();
}

#[crabgo_test]
fn panic_unwind_does_not_build_twice() {
    // Check for a bug where `lib` was built twice, once with panic set and
    // once without. Since "unwind" is the default, they are the same and
    // should only be built once.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [profile.dev]
            panic = "unwind"
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .file("tests/t1.rs", "")
        .build();

    p.crabgo("test -v --tests --no-run")
        .with_stderr_unordered(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc --crate-name foo src/lib.rs [..]--crate-type lib [..]
[RUNNING] `rustc --crate-name foo src/lib.rs [..] --test [..]
[RUNNING] `rustc --crate-name foo src/main.rs [..]--crate-type bin [..]
[RUNNING] `rustc --crate-name foo src/main.rs [..] --test [..]
[RUNNING] `rustc --crate-name t1 tests/t1.rs [..]
[FINISHED] [..]
[EXECUTABLE] `[..]/target/debug/deps/t1-[..][EXE]`
[EXECUTABLE] `[..]/target/debug/deps/foo-[..][EXE]`
[EXECUTABLE] `[..]/target/debug/deps/foo-[..][EXE]`
",
        )
        .run();
}

#[crabgo_test]
fn debug_0_report() {
    // The finished line handles 0 correctly.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [profile.dev]
            debug = 0
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("build -v")
        .with_stderr(
            "\
[COMPILING] foo v0.1.0 [..]
[RUNNING] `rustc --crate-name foo src/lib.rs [..]-C debuginfo=0 [..]
[FINISHED] dev [unoptimized] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn thin_lto_works() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "top"
                version = "0.5.0"
                authors = []

                [profile.release]
                lto = 'thin'
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build --release -v")
        .with_stderr(
            "\
[COMPILING] top [..]
[RUNNING] `rustc [..] -C lto=thin [..]`
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn strip_works() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [profile.release]
                strip = 'symbols'
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build --release -v")
        .with_stderr(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc [..] -C strip=symbols [..]`
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn strip_passes_unknown_option_to_rustc() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [profile.release]
                strip = 'unknown'
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build --release -v")
        .with_status(101)
        .with_stderr_contains(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc [..] -C strip=unknown [..]`
error: incorrect value `unknown` for [..] `strip` [..] was expected
",
        )
        .run();
}

#[crabgo_test]
fn strip_accepts_true_to_strip_symbols() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [profile.release]
                strip = true
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build --release -v")
        .with_stderr(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc [..] -C strip=symbols [..]`
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn strip_accepts_false_to_disable_strip() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [profile.release]
                strip = false
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build --release -v")
        .with_stderr_does_not_contain("-C strip")
        .run();
}

#[crabgo_test]
fn rustflags_works() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            crabgo-features = ["profile-rustflags"]

            [profile.dev]
            rustflags = ["-C", "link-dead-code=yes"]

            [package]
            name = "foo"
            version = "0.0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build -v")
        .masquerade_as_nightly_crabgo(&["profile-rustflags"])
        .with_stderr(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc --crate-name foo [..] -C link-dead-code=yes [..]
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustflags_works_with_env() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            crabgo-features = ["profile-rustflags"]

            [package]
            name = "foo"
            version = "0.0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build -v")
        .env("CRABGO_PROFILE_DEV_RUSTFLAGS", "-C link-dead-code=yes")
        .masquerade_as_nightly_crabgo(&["profile-rustflags"])
        .with_stderr(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc --crate-name foo [..] -C link-dead-code=yes [..]
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustflags_requires_crabgo_feature() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [profile.dev]
                rustflags = ["-C", "link-dead-code=yes"]

                [package]
                name = "foo"
                version = "0.0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build -v")
        .masquerade_as_nightly_crabgo(&["profile-rustflags"])
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[CWD]/Crabgo.toml`

Caused by:
  feature `profile-rustflags` is required

  The package requires the Crabgo feature called `profile-rustflags`, but that feature is \
  not stabilized in this version of Crabgo (1.[..]).
  Consider adding `crabgo-features = [\"profile-rustflags\"]` to the top of Crabgo.toml \
  (above the [package] table) to tell Crabgo you are opting in to use this unstable feature.
  See https://doc.rust-lang.org/nightly/crabgo/reference/unstable.html#profile-rustflags-option \
  for more information about the status of this feature.
",
        )
        .run();

    Package::new("bar", "1.0.0").publish();
    p.change_file(
        "Crabgo.toml",
        r#"
            [package]
            name = "foo"
            version = "0.0.1"

            [dependencies]
            bar = "1.0"

            [profile.dev.package.bar]
            rustflags = ["-C", "link-dead-code=yes"]
        "#,
    );
    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["profile-rustflags"])
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[ROOT]/foo/Crabgo.toml`

Caused by:
  feature `profile-rustflags` is required

  The package requires the Crabgo feature called `profile-rustflags`, but that feature is \
  not stabilized in this version of Crabgo (1.[..]).
  Consider adding `crabgo-features = [\"profile-rustflags\"]` to the top of Crabgo.toml \
  (above the [package] table) to tell Crabgo you are opting in to use this unstable feature.
  See https://doc.rust-lang.org/nightly/crabgo/reference/unstable.html#profile-rustflags-option \
  for more information about the status of this feature.
",
        )
        .run();
}
