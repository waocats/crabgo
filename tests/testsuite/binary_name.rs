use crabgo_test_support::install::{
    assert_has_installed_exe, assert_has_not_installed_exe, cargo_home,
};
use crabgo_test_support::project;

#[crabgo_test]
fn gated() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name =  "foo"
                version = "0.0.1"

                [[bin]]
                name = "foo"
                filename = "007bar"
                path = "src/main.rs"
            "#,
        )
        .file("src/main.rs", "fn main() { assert!(true) }")
        .build();

    // Run crabgo build.
    p.crabgo("build")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_status(101)
        .with_stderr_contains("[..]feature `different-binary-name` is required")
        .run();
}

#[crabgo_test]
// This test checks if:
// 1. The correct binary is produced
// 2. The deps file has the correct content
// 3. Fingerprinting works
// 4. `crabgo clean` command works
fn binary_name1() {
    // Create the project.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["different-binary-name"]

                [package]
                name =  "foo"
                version = "0.0.1"

                [[bin]]
                name = "foo"
                filename = "007bar"
                path = "src/main.rs"
            "#,
        )
        .file("src/main.rs", "fn main() { assert!(true) }")
        .build();

    // Run crabgo build.
    p.crabgo("build")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .run();

    // Check the name of the binary that crabgo has generated.
    // A binary with the name of the crate should NOT be created.
    let foo_path = p.bin("foo");
    assert!(!foo_path.is_file());
    // A binary with the name provided in `filename` parameter should be created.
    let bar_path = p.bin("007bar");
    assert!(bar_path.is_file());

    // Check if deps file exists.
    let deps_path = p.bin("007bar").with_extension("d");
    assert!(deps_path.is_file(), "{:?}", bar_path);

    let depinfo = p.read_file(deps_path.to_str().unwrap());

    // Prepare what content we expect to be present in deps file.
    let deps_exp = format!(
        "{}: {}",
        p.bin("007bar").to_str().unwrap(),
        p.root().join("src").join("main.rs").to_str().unwrap()
    );

    // Compare actual deps content with expected deps content.
    assert!(
        depinfo.lines().any(|line| line == deps_exp),
        "Content of `{}` is incorrect",
        deps_path.to_string_lossy()
    );

    // Run crabgo second time, to verify fingerprint.
    p.crabgo("build -p foo -v")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_stderr(
            "\
[FRESH] foo [..]
[FINISHED] [..]
",
        )
        .run();

    // Run crabgo clean.
    p.crabgo("clean -p foo")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .run();

    // Check if the appropriate file was removed.
    assert!(
        !bar_path.is_file(),
        "`crabgo clean` did not remove the correct files"
    );
}

#[crabgo_test]
// This test checks if:
// 1. Check `crabgo run`
// 2. Check `crabgo test`
// 3. Check `crabgo install/uninstall`
fn binary_name2() {
    // Create the project.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["different-binary-name"]

                [package]
                name =  "foo"
                version = "0.0.1"

                [[bin]]
                name = "foo"
                filename = "007bar"
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                fn hello(name: &str) -> String {
                    format!("Hello, {}!", name)
                }

                fn main() {
                    println!("{}", hello("crabs"));
                }

                #[cfg(test)]
                mod tests {
                    use super::*;

                    #[test]
                    fn check_crabs() {
                        assert_eq!(hello("crabs"), "Hello, crabs!");
                    }
                }
            "#,
        )
        .build();

    // Run crabgo build.
    p.crabgo("build")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .run();

    // Check the name of the binary that crabgo has generated.
    // A binary with the name of the crate should NOT be created.
    let foo_path = p.bin("foo");
    assert!(!foo_path.is_file());
    // A binary with the name provided in `filename` parameter should be created.
    let bar_path = p.bin("007bar");
    assert!(bar_path.is_file());

    // Check if `crabgo test` works
    p.crabgo("test")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[RUNNING] [..] (target/debug/deps/foo-[..][EXE])",
        )
        .with_stdout_contains("test tests::check_crabs ... ok")
        .run();

    // Check if `crabgo run` is able to execute the binary
    p.crabgo("run")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_stdout("Hello, crabs!")
        .run();

    p.crabgo("install")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .run();

    assert_has_installed_exe(cargo_home(), "007bar");

    p.crabgo("uninstall")
        .with_stderr("[REMOVING] [ROOT]/home/.crabgo/bin/007bar[EXE]")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .run();

    assert_has_not_installed_exe(cargo_home(), "007bar");
}

#[crabgo_test]
fn check_env_vars() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["different-binary-name"]

                [package]
                name =  "foo"
                version = "0.0.1"

                [[bin]]
                name = "foo"
                filename = "007bar"
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                fn main() {
                    println!("{}", option_env!("CRABGO_BIN_NAME").unwrap());
                }
            "#,
        )
        .file(
            "tests/integration.rs",
            r#"
                #[test]
                fn check_env_vars2() {
                    let value = option_env!("CRABGO_BIN_EXE_007bar").expect("Could not find environment variable.");
                    assert!(value.contains("007bar"));
                }
            "#
        )
        .build();

    // Run crabgo build.
    p.crabgo("build")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .run();
    p.crabgo("run")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_stdout("007bar")
        .run();
    p.crabgo("test")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_status(0)
        .run();
}

#[crabgo_test]
fn check_msg_format_json() {
    // Create the project.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["different-binary-name"]

                [package]
                name =  "foo"
                version = "0.0.1"

                [[bin]]
                name = "foo"
                filename = "007bar"
                path = "src/main.rs"
            "#,
        )
        .file("src/main.rs", "fn main() { assert!(true) }")
        .build();

    let output = r#"
{
    "reason": "compiler-artifact",
    "package_id": "foo 0.0.1 [..]",
    "manifest_path": "[CWD]/Crabgo.toml",
    "target": "{...}",
    "profile": "{...}",
    "features": [],
    "filenames": "{...}",
    "executable": "[ROOT]/foo/target/debug/007bar[EXE]",
    "fresh": false
}

{"reason":"build-finished", "success":true}
"#;

    // Run crabgo build.
    p.crabgo("build --message-format=json")
        .masquerade_as_nightly_crabgo(&["different-binary-name"])
        .with_json(output)
        .run();
}
