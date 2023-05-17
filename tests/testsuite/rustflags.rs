//! Tests for setting custom rustc flags.

use crabgo_test_support::registry::Package;
use crabgo_test_support::{
    basic_lib_manifest, basic_manifest, paths, project, project_in_home, rustc_host,
};
use std::fs;

#[crabgo_test]
fn env_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .build();

    // Use RUSTFLAGS to pass an argument that will generate an error
    p.crabgo("check --lib")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --bin=a")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --example=b")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("test")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("bench")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn env_rustflags_build_script() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(cfg!(foo)); }
            "#,
        )
        .build();

    p.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
}

#[crabgo_test]
fn env_rustflags_build_script_dep() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
}

#[crabgo_test]
fn env_rustflags_plugin() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(not(foo))]
                fn main() { }
            "#,
        )
        .build();

    p.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
}

#[crabgo_test]
fn env_rustflags_plugin_dep() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
}

#[crabgo_test]
fn env_rustflags_normal_source_with_target() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .build();

    let host = &rustc_host();

    // Use RUSTFLAGS to pass an argument that will generate an error
    p.crabgo("check --lib --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --bin=a --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --example=b --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("test --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("bench --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn env_rustflags_build_script_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(!cfg!(foo)); }
            "#,
        )
        .build();

    let host = rustc_host();
    p.crabgo("check --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[crabgo_test]
fn env_rustflags_build_script_with_target_doesnt_apply_to_host_kind() {
    // RUSTFLAGS should *not* be passed to rustc for build scripts when --target is specified as the
    // host triple even if target-applies-to-host-kind is enabled, to match legacy Crabgo behavior.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(!cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config.toml",
            r#"
                target-applies-to-host = true
            "#,
        )
        .build();

    let host = rustc_host();
    p.crabgo("check --target")
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg(host)
        .arg("-Ztarget-applies-to-host")
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[crabgo_test]
fn env_rustflags_build_script_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.crabgo("check --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[crabgo_test]
fn env_rustflags_plugin_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(foo)]
                fn main() { }
            "#,
        )
        .build();

    let host = rustc_host();
    p.crabgo("check --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[crabgo_test]
fn env_rustflags_plugin_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.crabgo("check --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[crabgo_test]
fn env_rustflags_recompile() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("check").run();
    // Setting RUSTFLAGS forces a recompile
    p.crabgo("check")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn env_rustflags_recompile2() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
    // Setting RUSTFLAGS forces a recompile
    p.crabgo("check")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn env_rustflags_no_recompile() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
    p.crabgo("check")
        .env("RUSTFLAGS", "--cfg foo")
        .with_stdout("")
        .run();
}

#[crabgo_test]
fn build_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["-Z", "bogus"]
            "#,
        )
        .build();

    p.crabgo("check --lib")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --bin=a")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --example=b")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("test")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("bench")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn build_rustflags_build_script() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.crabgo("check").run();
}

#[crabgo_test]
fn build_rustflags_build_script_dep() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.crabgo("check").run();
}

#[crabgo_test]
fn build_rustflags_plugin() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(not(foo))]
                fn main() { }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.crabgo("check").run();
}

#[crabgo_test]
fn build_rustflags_plugin_dep() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.crabgo("check").run();
}

#[crabgo_test]
fn build_rustflags_normal_source_with_target() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["-Z", "bogus"]
            "#,
        )
        .build();

    let host = &rustc_host();

    // Use build.rustflags to pass an argument that will generate an error
    p.crabgo("check --lib --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --bin=a --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --example=b --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("test --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("bench --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn build_rustflags_build_script_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(!cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    let host = rustc_host();
    p.crabgo("check --target").arg(host).run();
}

#[crabgo_test]
fn build_rustflags_build_script_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.crabgo("check --target").arg(host).run();
}

#[crabgo_test]
fn build_rustflags_plugin_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(foo)]
                fn main() { }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    let host = rustc_host();
    p.crabgo("check --target").arg(host).run();
}

#[crabgo_test]
fn build_rustflags_plugin_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Crabgo.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.crabgo("check --target").arg(host).run();
}

#[crabgo_test]
fn build_rustflags_recompile() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("check").run();

    // Setting RUSTFLAGS forces a recompile
    let config = r#"
        [build]
        rustflags = ["-Z", "bogus"]
        "#;
    let config_file = paths::root().join("foo/.crabgo/config");
    fs::create_dir_all(config_file.parent().unwrap()).unwrap();
    fs::write(config_file, config).unwrap();

    p.crabgo("check")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn build_rustflags_recompile2() {
    let p = project().file("src/lib.rs", "").build();

    p.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();

    // Setting RUSTFLAGS forces a recompile
    let config = r#"
        [build]
        rustflags = ["-Z", "bogus"]
        "#;
    let config_file = paths::root().join("foo/.crabgo/config");
    fs::create_dir_all(config_file.parent().unwrap()).unwrap();
    fs::write(config_file, config).unwrap();

    p.crabgo("check")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn build_rustflags_no_recompile() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.crabgo("check").env("RUSTFLAGS", "--cfg foo").run();
    p.crabgo("check")
        .env("RUSTFLAGS", "--cfg foo")
        .with_stdout("")
        .run();
}

#[crabgo_test]
fn build_rustflags_with_home_config() {
    // We need a config file inside the home directory
    let home = paths::home();
    let home_config = home.join(".crabgo");
    fs::create_dir(&home_config).unwrap();
    fs::write(
        &home_config.join("config"),
        r#"
            [build]
            rustflags = ["-Cllvm-args=-x86-asm-syntax=intel"]
        "#,
    )
    .unwrap();

    // And we need the project to be inside the home directory
    // so the walking process finds the home project twice.
    let p = project_in_home("foo").file("src/lib.rs", "").build();

    p.crabgo("check -v").run();
}

#[crabgo_test]
fn target_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .file(
            ".crabgo/config",
            &format!(
                "
            [target.{}]
            rustflags = [\"-Z\", \"bogus\"]
            ",
                rustc_host()
            ),
        )
        .build();

    p.crabgo("check --lib")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --bin=a")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --example=b")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("test")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("bench")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn target_rustflags_also_for_build_scripts() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config",
            &format!(
                "
            [target.{}]
            rustflags = [\"--cfg=foo\"]
            ",
                rustc_host()
            ),
        )
        .build();

    p.crabgo("check").run();
}

#[crabgo_test]
fn target_rustflags_not_for_build_scripts_with_target() {
    let host = rustc_host();
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(!cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config",
            &format!(
                "
            [target.{}]
            rustflags = [\"--cfg=foo\"]
            ",
                host
            ),
        )
        .build();

    p.crabgo("check --target").arg(host).run();

    // Enabling -Ztarget-applies-to-host should not make a difference without the config setting
    p.crabgo("check --target")
        .arg(host)
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg("-Ztarget-applies-to-host")
        .run();

    // Even with the setting, the rustflags from `target.` should not apply, to match the legacy
    // Crabgo behavior.
    p.change_file(
        ".crabgo/config",
        &format!(
            "
        target-applies-to-host = true

        [target.{}]
        rustflags = [\"--cfg=foo\"]
        ",
            host
        ),
    );
    p.crabgo("check --target")
        .arg(host)
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg("-Ztarget-applies-to-host")
        .run();
}

#[crabgo_test]
fn build_rustflags_for_build_scripts() {
    let host = rustc_host();
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { assert!(cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config",
            "
            [build]
            rustflags = [\"--cfg=foo\"]
            ",
        )
        .build();

    // With "legacy" behavior, build.rustflags should apply to build scripts without --target
    p.crabgo("check").run();

    // But should _not_ apply _with_ --target
    p.crabgo("check --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]assertion failed[..]")
        .run();

    // Enabling -Ztarget-applies-to-host should not make a difference without the config setting
    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg("-Ztarget-applies-to-host")
        .run();
    p.crabgo("check --target")
        .arg(host)
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg("-Ztarget-applies-to-host")
        .with_status(101)
        .with_stderr_contains("[..]assertion failed[..]")
        .run();

    // When set to false though, the "proper" behavior where host artifacts _only_ pick up on
    // [host] should be applied.
    p.change_file(
        ".crabgo/config",
        "
        target-applies-to-host = false

        [build]
        rustflags = [\"--cfg=foo\"]
        ",
    );
    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg("-Ztarget-applies-to-host")
        .with_status(101)
        .with_stderr_contains("[..]assertion failed[..]")
        .run();
    p.crabgo("check --target")
        .arg(host)
        .masquerade_as_nightly_crabgo(&["target-applies-to-host"])
        .arg("-Ztarget-applies-to-host")
        .with_status(101)
        .with_stderr_contains("[..]assertion failed[..]")
        .run();
}

#[crabgo_test]
fn host_rustflags_for_build_scripts() {
    let host = rustc_host();
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                // Ensure that --cfg=foo is passed.
                fn main() { assert!(cfg!(foo)); }
            "#,
        )
        .file(
            ".crabgo/config",
            &format!(
                "
                target-applies-to-host = false

                [host.{}]
                rustflags = [\"--cfg=foo\"]
                ",
                host
            ),
        )
        .build();

    p.crabgo("check --target")
        .arg(host)
        .masquerade_as_nightly_crabgo(&["target-applies-to-host", "host-config"])
        .arg("-Ztarget-applies-to-host")
        .arg("-Zhost-config")
        .run();
}

// target.{}.rustflags takes precedence over build.rustflags
#[crabgo_test]
fn target_rustflags_precedence() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            ".crabgo/config",
            &format!(
                "
            [build]
            rustflags = [\"--cfg\", \"foo\"]

            [target.{}]
            rustflags = [\"-Z\", \"bogus\"]
            ",
                rustc_host()
            ),
        )
        .build();

    p.crabgo("check --lib")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --bin=a")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("check --example=b")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("test")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.crabgo("bench")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[crabgo_test]
fn cfg_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "pub fn t() {}")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            ".crabgo/config",
            &format!(
                r#"
                [target.'cfg({})']
                rustflags = ["--cfg", "bar"]
                "#,
                if rustc_host().contains("-windows-") {
                    "windows"
                } else {
                    "not(windows)"
                }
            ),
        )
        .build();

    p.crabgo("build --lib -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("build --bin=a -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("build --example=b -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("test --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[EXECUTABLE] `[..]/target/debug/deps/foo-[..][EXE]`
[EXECUTABLE] `[..]/target/debug/deps/a-[..][EXE]`
[EXECUTABLE] `[..]/target/debug/deps/c-[..][EXE]`
",
        )
        .run();

    p.crabgo("bench --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] bench [optimized] target(s) in [..]
[EXECUTABLE] `[..]/target/release/deps/foo-[..][EXE]`
[EXECUTABLE] `[..]/target/release/deps/a-[..][EXE]`
",
        )
        .run();
}

// target.'cfg(...)'.rustflags takes precedence over build.rustflags
#[crabgo_test]
fn cfg_rustflags_precedence() {
    let p = project()
        .file("src/lib.rs", "pub fn t() {}")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            ".crabgo/config",
            &format!(
                r#"
                [build]
                rustflags = ["--cfg", "foo"]

                [target.'cfg({})']
                rustflags = ["--cfg", "bar"]
                "#,
                if rustc_host().contains("-windows-") {
                    "windows"
                } else {
                    "not(windows)"
                }
            ),
        )
        .build();

    p.crabgo("build --lib -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("build --bin=a -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("build --example=b -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.crabgo("test --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[EXECUTABLE] `[..]/target/debug/deps/foo-[..][EXE]`
[EXECUTABLE] `[..]/target/debug/deps/a-[..][EXE]`
[EXECUTABLE] `[..]/target/debug/deps/c-[..][EXE]`
",
        )
        .run();

    p.crabgo("bench --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] bench [optimized] target(s) in [..]
[EXECUTABLE] `[..]/target/release/deps/foo-[..][EXE]`
[EXECUTABLE] `[..]/target/release/deps/a-[..][EXE]`
",
        )
        .run();
}

#[crabgo_test]
fn target_rustflags_string_and_array_form1() {
    let p1 = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p1.crabgo("check -v")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    let p2 = project()
        .file("src/lib.rs", "")
        .file(
            ".crabgo/config",
            r#"
            [build]
            rustflags = "--cfg foo"
            "#,
        )
        .build();

    p2.crabgo("check -v")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn target_rustflags_string_and_array_form2() {
    let p1 = project()
        .file(
            ".crabgo/config",
            &format!(
                r#"
                    [target.{}]
                    rustflags = ["--cfg", "foo"]
                "#,
                rustc_host()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p1.crabgo("check -v")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    let p2 = project()
        .file(
            ".crabgo/config",
            &format!(
                r#"
                    [target.{}]
                    rustflags = "--cfg foo"
                "#,
                rustc_host()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p2.crabgo("check -v")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[crabgo_test]
fn two_matching_in_config() {
    let p1 = project()
        .file(
            ".crabgo/config",
            r#"
                [target.'cfg(unix)']
                rustflags = ["--cfg", 'foo="a"']
                [target.'cfg(windows)']
                rustflags = ["--cfg", 'foo="a"']
                [target.'cfg(target_pointer_width = "32")']
                rustflags = ["--cfg", 'foo="b"']
                [target.'cfg(target_pointer_width = "64")']
                rustflags = ["--cfg", 'foo="b"']
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                fn main() {
                    if cfg!(foo = "a") {
                        println!("a");
                    } else if cfg!(foo = "b") {
                        println!("b");
                    } else {
                        panic!()
                    }
                }
            "#,
        )
        .build();

    p1.crabgo("run").run();
    p1.crabgo("build").with_stderr("[FINISHED] [..]").run();
}

#[crabgo_test]
fn env_rustflags_misspelled() {
    let p = project().file("src/main.rs", "fn main() { }").build();

    for cmd in &["check", "build", "run", "test", "bench"] {
        p.crabgo(cmd)
            .env("RUST_FLAGS", "foo")
            .with_stderr_contains("[WARNING] Crabgo does not read `RUST_FLAGS` environment variable. Did you mean `RUSTFLAGS`?")
            .run();
    }
}

#[crabgo_test]
fn env_rustflags_misspelled_build_script() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() { }")
        .build();

    p.crabgo("check")
        .env("RUST_FLAGS", "foo")
        .with_stderr_contains("[WARNING] Crabgo does not read `RUST_FLAGS` environment variable. Did you mean `RUSTFLAGS`?")
        .run();
}

#[crabgo_test]
fn remap_path_prefix_ignored() {
    // Ensure that --remap-path-prefix does not affect metadata hash.
    let p = project().file("src/lib.rs", "").build();
    p.crabgo("build").run();
    let rlibs = p
        .glob("target/debug/deps/*.rlib")
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rlibs.len(), 1);
    p.crabgo("clean").run();

    let check_metadata_same = || {
        let rlibs2 = p
            .glob("target/debug/deps/*.rlib")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rlibs, rlibs2);
    };

    p.crabgo("build")
        .env(
            "RUSTFLAGS",
            "--remap-path-prefix=/abc=/zoo --remap-path-prefix /spaced=/zoo",
        )
        .run();
    check_metadata_same();

    p.crabgo("clean").run();
    p.crabgo("rustc -- --remap-path-prefix=/abc=/zoo --remap-path-prefix /spaced=/zoo")
        .run();
    check_metadata_same();
}

#[crabgo_test]
fn remap_path_prefix_works() {
    // Check that remap-path-prefix works.
    Package::new("bar", "0.1.0")
        .file("src/lib.rs", "pub fn f() -> &'static str { file!() }")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            bar = "0.1"
            "#,
        )
        .file(
            "src/main.rs",
            r#"
            fn main() {
                println!("{}", bar::f());
            }
            "#,
        )
        .build();

    p.crabgo("run")
        .env(
            "RUSTFLAGS",
            format!("--remap-path-prefix={}=/foo", paths::root().display()),
        )
        .with_stdout("/foo/home/.crabgo/registry/src/[..]/bar-0.1.0/src/lib.rs")
        .run();
}

#[crabgo_test]
fn host_config_rustflags_with_target() {
    // regression test for https://github.com/rust-lang/crabgo/issues/10206
    let p = project()
        .file("src/lib.rs", "")
        .file("build.rs.rs", "fn main() { assert!(cfg!(foo)); }")
        .file(".crabgo/config.toml", "target-applies-to-host = false")
        .build();

    p.crabgo("check")
        .masquerade_as_nightly_crabgo(&["target-applies-to-host", "host-config"])
        .arg("-Zhost-config")
        .arg("-Ztarget-applies-to-host")
        .arg("-Zunstable-options")
        .arg("--config")
        .arg("host.rustflags=[\"--cfg=foo\"]")
        .run();
}
