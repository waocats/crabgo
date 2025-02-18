//! Tests for --out-dir flag.

use crabgo_test_support::sleep_ms;
use crabgo_test_support::{basic_manifest, project};
use std::env;
use std::fs;
use std::path::Path;

#[crabgo_test]
fn binary_with_debug() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("Hello, World!") }"#)
        .build();

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .enable_mac_dsym()
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["foo"],
        &["foo", "foo.dSYM"],
        &["foo.exe", "foo.pdb"],
        &["foo.exe"],
    );
}

#[crabgo_test]
fn static_library_with_debug() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [lib]
                crate-type = ["staticlib"]
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #[no_mangle]
                pub extern "C" fn foo() { println!("Hello, World!") }
            "#,
        )
        .build();

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["libfoo.a"],
        &["libfoo.a"],
        &["foo.lib"],
        &["libfoo.a"],
    );
}

#[crabgo_test]
fn dynamic_library_with_debug() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [lib]
                crate-type = ["cdylib"]
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #[no_mangle]
                pub extern "C" fn foo() { println!("Hello, World!") }
            "#,
        )
        .build();

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .enable_mac_dsym()
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["libfoo.so"],
        &["libfoo.dylib", "libfoo.dylib.dSYM"],
        &["foo.dll", "foo.dll.exp", "foo.dll.lib", "foo.pdb"],
        &["foo.dll", "libfoo.dll.a"],
    );
}

#[crabgo_test]
fn rlib_with_debug() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [lib]
                crate-type = ["rlib"]
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                pub fn foo() { println!("Hello, World!") }
            "#,
        )
        .build();

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["libfoo.rlib"],
        &["libfoo.rlib"],
        &["libfoo.rlib"],
        &["libfoo.rlib"],
    );
}

#[crabgo_test]
fn include_only_the_binary_from_the_current_package() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [workspace]

                [dependencies]
                utils = { path = "./utils" }
            "#,
        )
        .file("src/lib.rs", "extern crate utils;")
        .file(
            "src/main.rs",
            r#"
                extern crate foo;
                extern crate utils;
                fn main() {
                    println!("Hello, World!")
                }
            "#,
        )
        .file("utils/Crabgo.toml", &basic_manifest("utils", "0.0.1"))
        .file("utils/src/lib.rs", "")
        .build();

    p.crabgo("build -Z unstable-options --bin foo --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .enable_mac_dsym()
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["foo"],
        &["foo", "foo.dSYM"],
        &["foo.exe", "foo.pdb"],
        &["foo.exe"],
    );
}

#[crabgo_test]
fn out_dir_is_a_file() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("Hello, World!") }"#)
        .file("out", "")
        .build();

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .with_status(101)
        .with_stderr_contains("[ERROR] failed to create directory [..]")
        .run();
}

#[crabgo_test]
fn replaces_artifacts() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("foo") }"#)
        .build();

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .run();
    p.process(
        &p.root()
            .join(&format!("out/foo{}", env::consts::EXE_SUFFIX)),
    )
    .with_stdout("foo")
    .run();

    sleep_ms(1000);
    p.change_file("src/main.rs", r#"fn main() { println!("bar") }"#);

    p.crabgo("build -Z unstable-options --out-dir out")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .run();
    p.process(
        &p.root()
            .join(&format!("out/foo{}", env::consts::EXE_SUFFIX)),
    )
    .with_stdout("bar")
    .run();
}

#[crabgo_test]
fn avoid_build_scripts() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [workspace]
            members = ["a", "b"]
            "#,
        )
        .file("a/Crabgo.toml", &basic_manifest("a", "0.0.1"))
        .file("a/src/main.rs", "fn main() {}")
        .file("a/build.rs", r#"fn main() { println!("hello-build-a"); }"#)
        .file("b/Crabgo.toml", &basic_manifest("b", "0.0.1"))
        .file("b/src/main.rs", "fn main() {}")
        .file("b/build.rs", r#"fn main() { println!("hello-build-b"); }"#)
        .build();

    p.crabgo("build -Z unstable-options --out-dir out -vv")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .enable_mac_dsym()
        .with_stdout_contains("[a 0.0.1] hello-build-a")
        .with_stdout_contains("[b 0.0.1] hello-build-b")
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["a", "b"],
        &["a", "a.dSYM", "b", "b.dSYM"],
        &["a.exe", "a.pdb", "b.exe", "b.pdb"],
        &["a.exe", "b.exe"],
    );
}

#[crabgo_test]
fn crabgo_build_out_dir() {
    let p = project()
        .file("src/main.rs", r#"fn main() { println!("Hello, World!") }"#)
        .file(
            ".crabgo/config",
            r#"
            [build]
            out-dir = "out"
            "#,
        )
        .build();

    p.crabgo("build -Z unstable-options")
        .masquerade_as_nightly_crabgo(&["out-dir"])
        .enable_mac_dsym()
        .run();
    check_dir_contents(
        &p.root().join("out"),
        &["foo"],
        &["foo", "foo.dSYM"],
        &["foo.exe", "foo.pdb"],
        &["foo.exe"],
    );
}

fn check_dir_contents(
    out_dir: &Path,
    expected_linux: &[&str],
    expected_mac: &[&str],
    expected_win_msvc: &[&str],
    expected_win_gnu: &[&str],
) {
    let expected = if cfg!(target_os = "windows") {
        if cfg!(target_env = "msvc") {
            expected_win_msvc
        } else {
            expected_win_gnu
        }
    } else if cfg!(target_os = "macos") {
        expected_mac
    } else {
        expected_linux
    };

    let actual = list_dir(out_dir);
    let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

fn list_dir(dir: &Path) -> Vec<String> {
    let mut res = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        res.push(entry.file_name().into_string().unwrap());
    }
    res.sort_unstable();
    res
}
