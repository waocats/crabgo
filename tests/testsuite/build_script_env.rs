//! Tests for build.rs rerun-if-env-changed and rustc-env

use crabgo_test_support::basic_manifest;
use crabgo_test_support::project;
use crabgo_test_support::sleep_ms;

#[crabgo_test]
fn rerun_if_env_changes() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("crabgo:rerun-if-env-changed=FOO");
                }
            "#,
        )
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("check")
        .env("FOO", "bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("check")
        .env("FOO", "baz")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("check")
        .env("FOO", "baz")
        .with_stderr("[FINISHED] [..]")
        .run();
    p.crabgo("check")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn rerun_if_env_or_file_changes() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("crabgo:rerun-if-env-changed=FOO");
                    println!("crabgo:rerun-if-changed=foo");
                }
            "#,
        )
        .file("foo", "")
        .build();

    p.crabgo("check")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("check")
        .env("FOO", "bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("check")
        .env("FOO", "bar")
        .with_stderr("[FINISHED] [..]")
        .run();
    sleep_ms(1000);
    p.change_file("foo", "");
    p.crabgo("check")
        .env("FOO", "bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustc_bootstrap() {
    let build_rs = r#"
        fn main() {
            println!("crabgo:rustc-env=RUSTC_BOOTSTRAP=1");
        }
    "#;
    let p = project()
        .file("Crabgo.toml", &basic_manifest("has-dashes", "0.0.1"))
        .file("src/lib.rs", "#![feature(rustc_attrs)]")
        .file("build.rs", build_rs)
        .build();
    // RUSTC_BOOTSTRAP unset on stable should error
    p.crabgo("check")
        .with_stderr_contains("error: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .with_stderr_contains(
            "help: [..] set the environment variable `RUSTC_BOOTSTRAP=has_dashes` [..]",
        )
        .with_status(101)
        .run();
    // nightly should warn whether or not RUSTC_BOOTSTRAP is set
    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["RUSTC_BOOTSTRAP"])
        // NOTE: uses RUSTC_BOOTSTRAP so it will be propagated to rustc
        // (this matters when tests are being run with a beta or stable crabgo)
        .env("RUSTC_BOOTSTRAP", "1")
        .with_stderr_contains("warning: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .run();
    // RUSTC_BOOTSTRAP set to the name of the library should warn
    p.crabgo("check")
        .env("RUSTC_BOOTSTRAP", "has_dashes")
        .with_stderr_contains("warning: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .run();
    // RUSTC_BOOTSTRAP set to some random value should error
    p.crabgo("check")
        .env("RUSTC_BOOTSTRAP", "bar")
        .with_stderr_contains("error: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .with_stderr_contains(
            "help: [..] set the environment variable `RUSTC_BOOTSTRAP=has_dashes` [..]",
        )
        .with_status(101)
        .run();

    // Tests for binaries instead of libraries
    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "0.0.1"))
        .file("src/main.rs", "#![feature(rustc_attrs)] fn main() {}")
        .file("build.rs", build_rs)
        .build();
    // nightly should warn when there's no library whether or not RUSTC_BOOTSTRAP is set
    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["RUSTC_BOOTSTRAP"])
        // NOTE: uses RUSTC_BOOTSTRAP so it will be propagated to rustc
        // (this matters when tests are being run with a beta or stable crabgo)
        .env("RUSTC_BOOTSTRAP", "1")
        .with_stderr_contains("warning: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .run();
    // RUSTC_BOOTSTRAP conditionally set when there's no library should error (regardless of the value)
    p.crabgo("check")
        .env("RUSTC_BOOTSTRAP", "foo")
        .with_stderr_contains("error: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .with_stderr_contains("help: [..] set the environment variable `RUSTC_BOOTSTRAP=1` [..]")
        .with_status(101)
        .run();
}

#[crabgo_test]
#[cfg(target_arch = "x86_64")]
fn build_script_sees_cfg_target_feature() {
    let build_rs = r#"
        fn main() {
            let cfg = std::env::var("CRABGO_CFG_TARGET_FEATURE").unwrap();
            eprintln!("CRABGO_CFG_TARGET_FEATURE={cfg}");
        }
    "#;

    let configs = [
        r#"
            [build]
            rustflags = ["-Ctarget-feature=+sse4.1,+sse4.2"]
        "#,
        r#"
            [target.'cfg(target_arch = "x86_64")']
            rustflags = ["-Ctarget-feature=+sse4.1,+sse4.2"]
        "#,
    ];

    for config in configs {
        let p = project()
            .file(".crabgo/config.toml", config)
            .file("src/lib.rs", r#""#)
            .file("build.rs", build_rs)
            .build();

        p.crabgo("check -vv")
            .with_stderr_contains("[foo 0.0.1] CRABGO_CFG_TARGET_FEATURE=[..]sse4.2[..]")
            .with_stderr_contains("[..]-Ctarget-feature=[..]+sse4.2[..]")
            .run();
    }
}

/// In this test, the cfg is self-contradictory. There's no *right* answer as to
/// what the value of `RUSTFLAGS` should be in this case. We chose to give a
/// warning. However, no matter what we do, it's important that build scripts
/// and rustc see a consistent picture
#[crabgo_test]
fn cfg_paradox() {
    let build_rs = r#"
        fn main() {
            let cfg = std::env::var("CRABGO_CFG_BERTRAND").is_ok();
            eprintln!("cfg!(bertrand)={cfg}");
        }
    "#;

    let config = r#"
        [target.'cfg(not(bertrand))']
        rustflags = ["--cfg=bertrand"]
    "#;

    let p = project()
        .file(".crabgo/config.toml", config)
        .file("src/lib.rs", r#""#)
        .file("build.rs", build_rs)
        .build();

    p.crabgo("check -vv")
        .with_stderr_contains("[WARNING] non-trivial mutual dependency between target-specific configuration and RUSTFLAGS")
        .with_stderr_contains("[foo 0.0.1] cfg!(bertrand)=true")
        .with_stderr_contains("[..]--cfg=bertrand[..]")
        .run();
}

/// This test checks how Crabgo handles rustc cfgs which are defined both with
/// and without a value. The expected behavior is that the environment variable
/// is going to contain all the values.
///
/// For example, this configuration:
/// ```
/// target_has_atomic
/// target_has_atomic="16"
/// target_has_atomic="32"
/// target_has_atomic="64"
/// target_has_atomic="8"
/// target_has_atomic="ptr"
/// ```
///
/// Should result in the following environment variable:
///
/// ```
/// CRABGO_CFG_TARGET_HAS_ATOMIC=16,32,64,8,ptr
/// ```
///
/// On the other hand, configuration symbols without any value should result in
/// an empty string.
///
/// For example, this configuration:
///
/// ```
/// target_thread_local
/// ```
///
/// Should result in the following environment variable:
///
/// ```
/// CRABGO_CFG_TARGET_THREAD_LOCAL=
/// ```
#[crabgo_test(nightly, reason = "affected rustc cfg is unstable")]
#[cfg(target_arch = "x86_64")]
fn rustc_cfg_with_and_without_value() {
    let build_rs = r#"
        fn main() {
            let cfg = std::env::var("CRABGO_CFG_TARGET_HAS_ATOMIC");
            eprintln!("CRABGO_CFG_TARGET_HAS_ATOMIC={cfg:?}");
            let cfg = std::env::var("CRABGO_CFG_WINDOWS");
            eprintln!("CRABGO_CFG_WINDOWS={cfg:?}");
            let cfg = std::env::var("CRABGO_CFG_UNIX");
            eprintln!("CRABGO_CFG_UNIX={cfg:?}");
        }
    "#;
    let p = project()
        .file("src/lib.rs", r#""#)
        .file("build.rs", build_rs)
        .build();

    let mut check = p.crabgo("check -vv");
    #[cfg(target_has_atomic = "64")]
    check.with_stderr_contains("[foo 0.0.1] CRABGO_CFG_TARGET_HAS_ATOMIC=Ok(\"[..]64[..]\")");
    #[cfg(windows)]
    check.with_stderr_contains("[foo 0.0.1] CRABGO_CFG_WINDOWS=Ok(\"\")");
    #[cfg(unix)]
    check.with_stderr_contains("[foo 0.0.1] CRABGO_CFG_UNIX=Ok(\"\")");
    check.run();
}
