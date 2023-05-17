//! Tests for the `crabgo install` command.

use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use crabgo_test_support::compare;
use crabgo_test_support::cross_compile;
use crabgo_test_support::git;
use crabgo_test_support::registry::{self, registry_path, Package};
use crabgo_test_support::{
    basic_manifest, crabgo_process, no_such_file_err_msg, project, project_in, symlink_supported, t,
};
use crabgo_util::ProcessError;

use crabgo_test_support::install::{
    assert_has_installed_exe, assert_has_not_installed_exe, cargo_home,
};
use crabgo_test_support::paths::{self, CrabgoPathExt};
use std::env;
use std::path::PathBuf;

fn pkg(name: &str, vers: &str) {
    Package::new(name, vers)
        .file("src/lib.rs", "")
        .file(
            "src/main.rs",
            &format!("extern crate {}; fn main() {{}}", name),
        )
        .publish();
}

#[crabgo_test]
fn simple() {
    pkg("foo", "0.0.1");

    crabgo_process("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry [..])
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");

    crabgo_process("uninstall foo")
        .with_stderr("[REMOVING] [CWD]/home/.crabgo/bin/foo[EXE]")
        .run();
    assert_has_not_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn simple_with_message_format() {
    pkg("foo", "0.0.1");

    crabgo_process("install foo --message-format=json")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry [..])
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .with_json(
            r#"
            {
                "reason": "compiler-artifact",
                "package_id": "foo 0.0.1 ([..])",
                "manifest_path": "[..]",
                "target": {
                    "kind": [
                        "lib"
                    ],
                    "crate_types": [
                        "lib"
                    ],
                    "name": "foo",
                    "src_path": "[..]/foo-0.0.1/src/lib.rs",
                    "edition": "2015",
                    "doc": true,
                    "doctest": true,
                    "test": true
                },
                "profile": "{...}",
                "features": [],
                "filenames": "{...}",
                "executable": null,
                "fresh": false
            }

            {
                "reason": "compiler-artifact",
                "package_id": "foo 0.0.1 ([..])",
                "manifest_path": "[..]",
                "target": {
                    "kind": [
                        "bin"
                    ],
                    "crate_types": [
                        "bin"
                    ],
                    "name": "foo",
                    "src_path": "[..]/foo-0.0.1/src/main.rs",
                    "edition": "2015",
                    "doc": true,
                    "doctest": false,
                    "test": true
                },
                "profile": "{...}",
                "features": [],
                "filenames": "{...}",
                "executable": "[..]",
                "fresh": false
            }

            {"reason":"build-finished","success":true}
            "#,
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn with_index() {
    let registry = registry::init();
    pkg("foo", "0.0.1");

    crabgo_process("install foo --index")
        .arg(registry.index_url().as_str())
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `{reg}`)
[INSTALLING] foo v0.0.1 (registry `{reg}`)
[COMPILING] foo v0.0.1 (registry `{reg}`)
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1 (registry `{reg}`)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
            reg = registry_path().to_str().unwrap()
        ))
        .run();
    assert_has_installed_exe(cargo_home(), "foo");

    crabgo_process("uninstall foo")
        .with_stderr("[REMOVING] [CWD]/home/.crabgo/bin/foo[EXE]")
        .run();
    assert_has_not_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn multiple_pkgs() {
    pkg("foo", "0.0.1");
    pkg("bar", "0.0.2");

    crabgo_process("install foo bar baz")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `dummy-registry`)
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.2 (registry `dummy-registry`)
[ERROR] could not find `baz` in registry `[..]` with version `*`
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[INSTALLING] bar v0.0.2
[COMPILING] bar v0.0.2
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/bar[EXE]
[INSTALLED] package `bar v0.0.2` (executable `bar[EXE]`)
[SUMMARY] Successfully installed foo, bar! Failed to install baz (see error(s) above).
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
[ERROR] some crates failed to install
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
    assert_has_installed_exe(cargo_home(), "bar");

    crabgo_process("uninstall foo bar")
        .with_stderr(
            "\
[REMOVING] [CWD]/home/.crabgo/bin/foo[EXE]
[REMOVING] [CWD]/home/.crabgo/bin/bar[EXE]
[SUMMARY] Successfully uninstalled foo, bar!
",
        )
        .run();

    assert_has_not_installed_exe(cargo_home(), "foo");
    assert_has_not_installed_exe(cargo_home(), "bar");
}

fn path() -> Vec<PathBuf> {
    env::split_paths(&env::var_os("PATH").unwrap_or_default()).collect()
}

#[crabgo_test]
fn multiple_pkgs_path_set() {
    // confirm partial failure results in 101 status code and does not have the
    //      '[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries'
    //  even if CRABGO_HOME/bin is in the PATH
    pkg("foo", "0.0.1");
    pkg("bar", "0.0.2");

    // add CRABGO_HOME/bin to path
    let mut path = path();
    path.push(cargo_home().join("bin"));
    let new_path = env::join_paths(path).unwrap();
    crabgo_process("install foo bar baz")
        .env("PATH", new_path)
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `dummy-registry`)
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.2 (registry `dummy-registry`)
[ERROR] could not find `baz` in registry `[..]` with version `*`
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[INSTALLING] bar v0.0.2
[COMPILING] bar v0.0.2
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/bar[EXE]
[INSTALLED] package `bar v0.0.2` (executable `bar[EXE]`)
[SUMMARY] Successfully installed foo, bar! Failed to install baz (see error(s) above).
[ERROR] some crates failed to install
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
    assert_has_installed_exe(cargo_home(), "bar");

    crabgo_process("uninstall foo bar")
        .with_stderr(
            "\
[REMOVING] [CWD]/home/.crabgo/bin/foo[EXE]
[REMOVING] [CWD]/home/.crabgo/bin/bar[EXE]
[SUMMARY] Successfully uninstalled foo, bar!
",
        )
        .run();

    assert_has_not_installed_exe(cargo_home(), "foo");
    assert_has_not_installed_exe(cargo_home(), "bar");
}

#[crabgo_test]
fn pick_max_version() {
    pkg("foo", "0.1.0");
    pkg("foo", "0.2.0");
    pkg("foo", "0.2.1");
    pkg("foo", "0.2.1-pre.1");
    pkg("foo", "0.3.0-pre.2");

    crabgo_process("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.2.1 (registry [..])
[INSTALLING] foo v0.2.1
[COMPILING] foo v0.2.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.2.1` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn installs_beta_version_by_explicit_name_from_git() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.3.0-beta.1"))
        .file("src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .arg("foo")
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn missing() {
    pkg("foo", "0.0.1");
    crabgo_process("install bar")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..] index
[ERROR] could not find `bar` in registry `[..]` with version `*`
",
        )
        .run();
}

#[crabgo_test]
fn missing_current_working_directory() {
    crabgo_process("install .")
        .with_status(101)
        .with_stderr(
            "error: To install the binaries for the package in current working \
            directory use `crabgo install --path .`. \n\
            Use `crabgo build` if you want to simply build the package.",
        )
        .run();
}

#[crabgo_test]
fn bad_version() {
    pkg("foo", "0.0.1");
    crabgo_process("install foo --version=0.2.0")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..] index
[ERROR] could not find `foo` in registry `[..]` with version `=0.2.0`
",
        )
        .run();
}

#[crabgo_test]
fn bad_paths() {
    crabgo_process("install")
        .with_status(101)
        .with_stderr("[ERROR] `[CWD]` is not a crate root; specify a crate to install [..]")
        .run();

    crabgo_process("install --path .")
        .with_status(101)
        .with_stderr("[ERROR] `[CWD]` does not contain a Crabgo.toml file[..]")
        .run();

    let toml = paths::root().join("Crabgo.toml");
    fs::write(toml, "").unwrap();
    crabgo_process("install --path Crabgo.toml")
        .with_status(101)
        .with_stderr("[ERROR] `[CWD]/Crabgo.toml` is not a directory[..]")
        .run();

    crabgo_process("install --path .")
        .with_status(101)
        .with_stderr_contains("[ERROR] failed to parse manifest at `[CWD]/Crabgo.toml`")
        .run();
}

#[crabgo_test]
fn install_location_precedence() {
    pkg("foo", "0.0.1");

    let root = paths::root();
    let t1 = root.join("t1");
    let t2 = root.join("t2");
    let t3 = root.join("t3");
    let t4 = cargo_home();

    fs::create_dir(root.join(".crabgo")).unwrap();
    fs::write(
        root.join(".crabgo/config"),
        &format!(
            "[install]
             root = '{}'
            ",
            t3.display()
        ),
    )
    .unwrap();

    println!("install --root");

    crabgo_process("install foo --root")
        .arg(&t1)
        .env("CRABGO_INSTALL_ROOT", &t2)
        .run();
    assert_has_installed_exe(&t1, "foo");
    assert_has_not_installed_exe(&t2, "foo");

    println!("install CRABGO_INSTALL_ROOT");

    crabgo_process("install foo")
        .env("CRABGO_INSTALL_ROOT", &t2)
        .run();
    assert_has_installed_exe(&t2, "foo");
    assert_has_not_installed_exe(&t3, "foo");

    println!("install install.root");

    crabgo_process("install foo").run();
    assert_has_installed_exe(&t3, "foo");
    assert_has_not_installed_exe(&t4, "foo");

    fs::remove_file(root.join(".crabgo/config")).unwrap();

    println!("install crabgo home");

    crabgo_process("install foo").run();
    assert_has_installed_exe(&t4, "foo");
}

#[crabgo_test]
fn install_path() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    crabgo_process("install --path").arg(p.root()).run();
    assert_has_installed_exe(cargo_home(), "foo");
    // path-style installs force a reinstall
    p.crabgo("install --path .")
        .with_stderr(
            "\
[INSTALLING] foo v0.0.1 [..]
[FINISHED] release [..]
[REPLACING] [..]/.crabgo/bin/foo[EXE]
[REPLACED] package `foo v0.0.1 [..]` with `foo v0.0.1 [..]` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[crabgo_test]
fn install_target_dir() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.crabgo("install --target-dir td_test")
        .with_stderr(
            "\
[WARNING] Using `crabgo install` [..]
[INSTALLING] foo v0.0.1 [..]
[COMPILING] foo v0.0.1 [..]
[FINISHED] release [..]
[INSTALLING] [..]foo[EXE]
[INSTALLED] package `foo v0.0.1 [..]foo[..]` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();

    let mut path = p.root();
    path.push("td_test");
    assert!(path.exists());

    #[cfg(not(windows))]
    path.push("release/foo");
    #[cfg(windows)]
    path.push("release/foo.exe");
    assert!(path.exists());
}

#[crabgo_test]
#[cfg(target_os = "linux")]
fn install_path_with_lowercase_crabgo_toml() {
    let toml = paths::root().join("crabgo.toml");
    fs::write(toml, "").unwrap();

    crabgo_process("install --path .")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] `[CWD]` does not contain a Crabgo.toml file, \
but found crabgo.toml please try to rename it to Crabgo.toml. --path must point to a directory containing a Crabgo.toml file.
",
        )
        .run();
}

#[crabgo_test]
fn install_relative_path_outside_current_ws() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["baz"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "baz/Crabgo.toml",
            r#"
                [package]
                name = "baz"
                version = "0.1.0"
                authors = []
                edition = "2021"

                [dependencies]
                foo = "1"
            "#,
        )
        .file("baz/src/lib.rs", "")
        .build();

    let _bin_project = project_in("bar")
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("install --path ../bar/foo")
        .with_stderr(&format!(
            "\
[INSTALLING] foo v0.0.1 ([..]/bar/foo)
[COMPILING] foo v0.0.1 ([..]/bar/foo)
[FINISHED] release [..]
[INSTALLING] {home}/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1 ([..]/bar/foo)` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
            home = cargo_home().display(),
        ))
        .run();

    // Validate the workspace error message to display available targets.
    p.crabgo("install --path ../bar/foo --bin")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] \"--bin\" takes one argument.
Available binaries:
    foo

",
        )
        .run();
}

#[crabgo_test]
fn multiple_packages_containing_binaries() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .file("a/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/main.rs", "fn main() {}")
        .build();

    let git_url = p.url().to_string();
    crabgo_process("install --git")
        .arg(p.url().to_string())
        .with_status(101)
        .with_stderr(format!(
            "\
[UPDATING] git repository [..]
[ERROR] multiple packages with binaries found: bar, foo. \
When installing a git repository, crabgo will always search the entire repo for any Crabgo.toml.
Please specify a package, e.g. `crabgo install --git {git_url} bar`.
"
        ))
        .run();
}

#[crabgo_test]
fn multiple_packages_matching_example() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "")
        .file("examples/ex1.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "")
        .file("bar/examples/ex1.rs", "fn main() {}")
        .build();

    let git_url = p.url().to_string();
    crabgo_process("install --example ex1 --git")
        .arg(p.url().to_string())
        .with_status(101)
        .with_stderr(format!(
            "\
[UPDATING] git repository [..]
[ERROR] multiple packages with examples found: bar, foo. \
When installing a git repository, crabgo will always search the entire repo for any Crabgo.toml.
Please specify a package, e.g. `crabgo install --git {git_url} bar`."
        ))
        .run();
}

#[crabgo_test]
fn multiple_binaries_deep_select_uses_package_name() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .file("bar/baz/Crabgo.toml", &basic_manifest("baz", "0.1.0"))
        .file("bar/baz/src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .arg("baz")
        .run();
}

#[crabgo_test]
fn multiple_binaries_in_selected_package_installs_all() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/bin/bin1.rs", "fn main() {}")
        .file("bar/src/bin/bin2.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .arg("bar")
        .run();

    let cargo_home = cargo_home();
    assert_has_installed_exe(&cargo_home, "bin1");
    assert_has_installed_exe(&cargo_home, "bin2");
}

#[crabgo_test]
fn multiple_binaries_in_selected_package_with_bin_option_installs_only_one() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/lib.rs", "")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/bin/bin1.rs", "fn main() {}")
        .file("bar/src/bin/bin2.rs", "fn main() {}")
        .build();

    crabgo_process("install --bin bin1 --git")
        .arg(p.url().to_string())
        .arg("bar")
        .run();

    let cargo_home = cargo_home();
    assert_has_installed_exe(&cargo_home, "bin1");
    assert_has_not_installed_exe(&cargo_home, "bin2");
}

#[crabgo_test]
fn multiple_crates_select() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .file("a/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .arg("foo")
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
    assert_has_not_installed_exe(cargo_home(), "bar");

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .arg("bar")
        .run();
    assert_has_installed_exe(cargo_home(), "bar");
}

#[crabgo_test]
fn multiple_crates_git_all() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Crabgo.toml",
            r#"
            [workspace]
            members = ["bin1", "bin2"]
            "#,
        )
        .file("bin1/Crabgo.toml", &basic_manifest("bin1", "0.1.0"))
        .file("bin2/Crabgo.toml", &basic_manifest("bin2", "0.1.0"))
        .file(
            "bin1/src/main.rs",
            r#"fn main() { println!("Hello, world!"); }"#,
        )
        .file(
            "bin2/src/main.rs",
            r#"fn main() { println!("Hello, world!"); }"#,
        )
        .build();

    crabgo_process(&format!("install --git {} bin1 bin2", p.url().to_string())).run();
}

#[crabgo_test]
fn multiple_crates_auto_binaries() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/main.rs", "extern crate bar; fn main() {}")
        .file("a/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    crabgo_process("install --path").arg(p.root()).run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn multiple_crates_auto_examples() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/lib.rs", "extern crate bar;")
        .file(
            "examples/foo.rs",
            "
            extern crate bar;
            extern crate foo;
            fn main() {}
        ",
        )
        .file("a/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    crabgo_process("install --path")
        .arg(p.root())
        .arg("--example=foo")
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn no_binaries_or_examples() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("a/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    crabgo_process("install --path")
        .arg(p.root())
        .with_status(101)
        .with_stderr("[ERROR] no packages found with binaries or examples")
        .run();
}

#[crabgo_test]
fn no_binaries() {
    let p = project()
        .file("src/lib.rs", "")
        .file("examples/foo.rs", "fn main() {}")
        .build();

    crabgo_process("install --path")
        .arg(p.root())
        .arg("foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] there is nothing to install in `foo v0.0.1 ([..])`, because it has no binaries[..]
[..]
To use a library crate, add it as a dependency to a Crabgo project with `crabgo add`.",
        )
        .run();
}

#[crabgo_test]
fn examples() {
    let p = project()
        .file("src/lib.rs", "")
        .file("examples/foo.rs", "extern crate foo; fn main() {}")
        .build();

    crabgo_process("install --path")
        .arg(p.root())
        .arg("--example=foo")
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn install_force() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    crabgo_process("install --path").arg(p.root()).run();

    let p = project()
        .at("foo2")
        .file("Crabgo.toml", &basic_manifest("foo", "0.2.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --force --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.2.0 ([..])
[COMPILING] foo v0.2.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[REPLACING] [CWD]/home/.crabgo/bin/foo[EXE]
[REPLACED] package `foo v0.0.1 ([..]/foo)` with `foo v0.2.0 ([..]/foo2)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    crabgo_process("install --list")
        .with_stdout(
            "\
foo v0.2.0 ([..]):
    foo[..]
",
        )
        .run();
}

#[crabgo_test]
fn install_force_partial_overlap() {
    let p = project()
        .file("src/bin/foo-bin1.rs", "fn main() {}")
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .build();

    crabgo_process("install --path").arg(p.root()).run();

    let p = project()
        .at("foo2")
        .file("Crabgo.toml", &basic_manifest("foo", "0.2.0"))
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .file("src/bin/foo-bin3.rs", "fn main() {}")
        .build();

    crabgo_process("install --force --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.2.0 ([..])
[COMPILING] foo v0.2.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo-bin3[EXE]
[REPLACING] [CWD]/home/.crabgo/bin/foo-bin2[EXE]
[REMOVING] executable `[..]/bin/foo-bin1[EXE]` from previous version foo v0.0.1 [..]
[INSTALLED] package `foo v0.2.0 ([..]/foo2)` (executable `foo-bin3[EXE]`)
[REPLACED] package `foo v0.0.1 ([..]/foo)` with `foo v0.2.0 ([..]/foo2)` (executable `foo-bin2[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    crabgo_process("install --list")
        .with_stdout(
            "\
foo v0.2.0 ([..]):
    foo-bin2[..]
    foo-bin3[..]
",
        )
        .run();
}

#[crabgo_test]
fn install_force_bin() {
    let p = project()
        .file("src/bin/foo-bin1.rs", "fn main() {}")
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .build();

    crabgo_process("install --path").arg(p.root()).run();

    let p = project()
        .at("foo2")
        .file("Crabgo.toml", &basic_manifest("foo", "0.2.0"))
        .file("src/bin/foo-bin1.rs", "fn main() {}")
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .build();

    crabgo_process("install --force --bin foo-bin2 --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.2.0 ([..])
[COMPILING] foo v0.2.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[REPLACING] [CWD]/home/.crabgo/bin/foo-bin2[EXE]
[REPLACED] package `foo v0.0.1 ([..]/foo)` with `foo v0.2.0 ([..]/foo2)` (executable `foo-bin2[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    crabgo_process("install --list")
        .with_stdout(
            "\
foo v0.0.1 ([..]):
    foo-bin1[..]
foo v0.2.0 ([..]):
    foo-bin2[..]
",
        )
        .run();
}

#[crabgo_test]
fn compile_failure() {
    let p = project().file("src/main.rs", "").build();

    crabgo_process("install --path")
        .arg(p.root())
        .with_status(101)
        .with_stderr_contains(
            "\
[ERROR] could not compile `foo` (bin \"foo\") due to previous error
[ERROR] failed to compile `foo v0.0.1 ([..])`, intermediate artifacts can be \
    found at `[..]target`
",
        )
        .run();
}

#[crabgo_test]
fn git_repo() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    // Use `--locked` to test that we don't even try to write a lock file.
    crabgo_process("install --locked --git")
        .arg(p.url().to_string())
        .with_stderr(
            "\
[UPDATING] git repository `[..]`
[WARNING] no Crabgo.lock file published in foo v0.1.0 ([..])
[INSTALLING] foo v0.1.0 ([..])
[COMPILING] foo v0.1.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.1.0 ([..]/foo#[..])` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
#[cfg(target_os = "linux")]
fn git_repo_with_lowercase_crabgo_toml() {
    let p = git::repo(&paths::root().join("foo"))
        .file("crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] git repository [..]
[ERROR] Could not find Crabgo.toml in `[..]`, but found crabgo.toml please try to rename it to Crabgo.toml
",
        )
        .run();
}

#[crabgo_test]
fn list() {
    pkg("foo", "0.0.1");
    pkg("bar", "0.2.1");
    pkg("bar", "0.2.2");

    crabgo_process("install --list").with_stdout("").run();

    crabgo_process("install bar --version =0.2.1").run();
    crabgo_process("install foo").run();
    crabgo_process("install --list")
        .with_stdout(
            "\
bar v0.2.1:
    bar[..]
foo v0.0.1:
    foo[..]
",
        )
        .run();
}

#[crabgo_test]
fn list_error() {
    pkg("foo", "0.0.1");
    crabgo_process("install foo").run();
    crabgo_process("install --list")
        .with_stdout(
            "\
foo v0.0.1:
    foo[..]
",
        )
        .run();
    let mut worldfile_path = cargo_home();
    worldfile_path.push(".crates.toml");
    let mut worldfile = OpenOptions::new()
        .write(true)
        .open(worldfile_path)
        .expect(".crates.toml should be there");
    worldfile.write_all(b"\x00").unwrap();
    drop(worldfile);
    crabgo_process("install --list --verbose")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse crate metadata at `[..]`

Caused by:
  invalid TOML found for metadata

Caused by:
  TOML parse error at line 1, column 1
    |
  1 | [..]
    | ^
  invalid key
",
        )
        .run();
}

#[crabgo_test]
fn uninstall_pkg_does_not_exist() {
    crabgo_process("uninstall foo")
        .with_status(101)
        .with_stderr("[ERROR] package ID specification `foo` did not match any packages")
        .run();
}

#[crabgo_test]
fn uninstall_bin_does_not_exist() {
    pkg("foo", "0.0.1");

    crabgo_process("install foo").run();
    crabgo_process("uninstall foo --bin=bar")
        .with_status(101)
        .with_stderr("[ERROR] binary `bar[..]` not installed as part of `foo v0.0.1`")
        .run();
}

#[crabgo_test]
fn uninstall_piecemeal() {
    let p = project()
        .file("src/bin/foo.rs", "fn main() {}")
        .file("src/bin/bar.rs", "fn main() {}")
        .build();

    crabgo_process("install --path").arg(p.root()).run();
    assert_has_installed_exe(cargo_home(), "foo");
    assert_has_installed_exe(cargo_home(), "bar");

    crabgo_process("uninstall foo --bin=bar")
        .with_stderr("[REMOVING] [..]bar[..]")
        .run();

    assert_has_installed_exe(cargo_home(), "foo");
    assert_has_not_installed_exe(cargo_home(), "bar");

    crabgo_process("uninstall foo --bin=foo")
        .with_stderr("[REMOVING] [..]foo[..]")
        .run();
    assert_has_not_installed_exe(cargo_home(), "foo");

    crabgo_process("uninstall foo")
        .with_status(101)
        .with_stderr("[ERROR] package ID specification `foo` did not match any packages")
        .run();
}

#[crabgo_test]
fn subcommand_works_out_of_the_box() {
    Package::new("crabgo-foo", "1.0.0")
        .file("src/main.rs", r#"fn main() { println!("bar"); }"#)
        .publish();
    crabgo_process("install crabgo-foo").run();
    crabgo_process("foo").with_stdout("bar\n").run();
    crabgo_process("--list")
        .with_stdout_contains("    foo\n")
        .run();
}

#[crabgo_test]
fn installs_from_cwd_by_default() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.crabgo("install")
        .with_stderr_contains(
            "warning: Using `crabgo install` to install the binaries for the \
             package in current working directory is deprecated, \
             use `crabgo install --path .` instead. \
             Use `crabgo build` if you want to simply build the package.",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn installs_from_cwd_with_2018_warnings() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []
                edition = "2018"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("install")
        .with_status(101)
        .with_stderr_contains(
            "error: Using `crabgo install` to install the binaries for the \
             package in current working directory is no longer supported, \
             use `crabgo install --path .` instead. \
             Use `crabgo build` if you want to simply build the package.",
        )
        .run();
    assert_has_not_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn uninstall_cwd() {
    let p = project().file("src/main.rs", "fn main() {}").build();
    p.crabgo("install --path .")
        .with_stderr(&format!(
            "\
[INSTALLING] foo v0.0.1 ([CWD])
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] {home}/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1 ([..]/foo)` (executable `foo[EXE]`)
[WARNING] be sure to add `{home}/bin` to your PATH to be able to run the installed binaries",
            home = cargo_home().display(),
        ))
        .run();
    assert_has_installed_exe(cargo_home(), "foo");

    p.crabgo("uninstall")
        .with_stdout("")
        .with_stderr(&format!(
            "[REMOVING] {home}/bin/foo[EXE]",
            home = cargo_home().display()
        ))
        .run();
    assert_has_not_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn uninstall_cwd_not_installed() {
    let p = project().file("src/main.rs", "fn main() {}").build();
    p.crabgo("uninstall")
        .with_status(101)
        .with_stdout("")
        .with_stderr("error: package `foo v0.0.1 ([CWD])` is not installed")
        .run();
}

#[crabgo_test]
fn uninstall_cwd_no_project() {
    crabgo_process("uninstall")
        .with_status(101)
        .with_stdout("")
        .with_stderr(format!(
            "\
[ERROR] failed to read `[CWD]/Crabgo.toml`

Caused by:
  {err_msg}",
            err_msg = no_such_file_err_msg(),
        ))
        .run();
}

#[crabgo_test]
fn do_not_rebuilds_on_local_install() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.crabgo("build --release").run();
    crabgo_process("install --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] [..]
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]
[INSTALLED] package `foo v0.0.1 ([..]/foo)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    assert!(p.build_dir().exists());
    assert!(p.release_bin("foo").exists());
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn reports_unsuccessful_subcommand_result() {
    Package::new("crabgo-fail", "1.0.0")
        .file("src/main.rs", "fn main() { panic!(); }")
        .publish();
    crabgo_process("install crabgo-fail").run();
    crabgo_process("--list")
        .with_stdout_contains("    fail\n")
        .run();
    crabgo_process("fail")
        .with_status(101)
        .with_stderr_contains("thread '[..]' panicked at 'explicit panic', [..]")
        .run();
}

#[crabgo_test]
fn git_with_lockfile() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "bar" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "fn main() {}")
        .file(
            "Crabgo.lock",
            r#"
                [[package]]
                name = "foo"
                version = "0.1.0"
                dependencies = [ "bar 0.1.0" ]

                [[package]]
                name = "bar"
                version = "0.1.0"
            "#,
        )
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .run();
}

#[crabgo_test]
fn q_silences_warnings() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    crabgo_process("install -q --path")
        .arg(p.root())
        .with_stderr("")
        .run();
}

#[crabgo_test]
fn readonly_dir() {
    pkg("foo", "0.0.1");

    let root = paths::root();
    let dir = &root.join("readonly");
    fs::create_dir(root.join("readonly")).unwrap();
    let mut perms = fs::metadata(dir).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir, perms).unwrap();

    crabgo_process("install foo").cwd(dir).run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn use_path_workspace() {
    Package::new("foo", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["baz"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "baz/Crabgo.toml",
            r#"
                [package]
                name = "baz"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = "1"
            "#,
        )
        .file("baz/src/lib.rs", "")
        .build();

    p.crabgo("build").run();
    let lock = p.read_lockfile();
    p.crabgo("install").run();
    let lock2 = p.read_lockfile();
    assert_eq!(lock, lock2, "different lockfiles");
}

#[crabgo_test]
fn path_install_workspace_root_despite_default_members() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "ws-root"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["ws-member"]
                default-members = ["ws-member"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "ws-member/Crabgo.toml",
            r#"
                [package]
                name = "ws-member"
                version = "0.1.0"
                authors = []
            "#,
        )
        .file("ws-member/src/main.rs", "fn main() {}")
        .build();

    p.crabgo("install --path")
        .arg(p.root())
        .arg("ws-root")
        .with_stderr_contains(
            "[INSTALLED] package `ws-root v0.1.0 ([..])` (executable `ws-root[EXE]`)",
        )
        // Particularly avoid "Installed package `ws-root v0.1.0 ([..]])` (executable `ws-member`)":
        .with_stderr_does_not_contain("ws-member")
        .run();
}

#[crabgo_test]
fn git_install_workspace_root_despite_default_members() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "ws-root"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["ws-member"]
                default-members = ["ws-member"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "ws-member/Crabgo.toml",
            r#"
                [package]
                name = "ws-member"
                version = "0.1.0"
                authors = []
            "#,
        )
        .file("ws-member/src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .arg("ws-root")
        .with_stderr_contains(
            "[INSTALLED] package `ws-root v0.1.0 ([..])` (executable `ws-root[EXE]`)",
        )
        // Particularly avoid "Installed package `ws-root v0.1.0 ([..]])` (executable `ws-member`)":
        .with_stderr_does_not_contain("ws-member")
        .run();
}

#[crabgo_test]
fn dev_dependencies_no_check() {
    Package::new("foo", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [dev-dependencies]
                baz = "1.0.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build")
        .with_status(101)
        .with_stderr_contains("[..] no matching package named `baz` found")
        .run();
    p.crabgo("install").run();
}

#[crabgo_test]
fn dev_dependencies_lock_file_untouched() {
    Package::new("foo", "1.0.0").publish();
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dev-dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("a/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    p.crabgo("build").run();
    let lock = p.read_lockfile();
    p.crabgo("install").run();
    let lock2 = p.read_lockfile();
    assert!(lock == lock2, "different lockfiles");
}

#[crabgo_test]
fn install_target_native() {
    pkg("foo", "0.1.0");

    crabgo_process("install foo --target")
        .arg(crabgo_test_support::rustc_host())
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn install_target_foreign() {
    if cross_compile::disabled() {
        return;
    }

    pkg("foo", "0.1.0");

    crabgo_process("install foo --target")
        .arg(cross_compile::alternate())
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn vers_precise() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    crabgo_process("install foo --vers 0.1.1")
        .with_stderr_contains("[DOWNLOADED] foo v0.1.1 (registry [..])")
        .run();
}

#[crabgo_test]
fn version_precise() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    crabgo_process("install foo --version 0.1.1")
        .with_stderr_contains("[DOWNLOADED] foo v0.1.1 (registry [..])")
        .run();
}

#[crabgo_test]
fn inline_version_precise() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    crabgo_process("install foo@0.1.1")
        .with_stderr_contains("[DOWNLOADED] foo v0.1.1 (registry [..])")
        .run();
}

#[crabgo_test]
fn inline_version_multiple() {
    pkg("foo", "0.1.0");
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");
    pkg("bar", "0.2.0");
    pkg("bar", "0.2.1");
    pkg("bar", "0.2.2");

    crabgo_process("install foo@0.1.1 bar@0.2.1")
        .with_stderr_contains("[DOWNLOADED] foo v0.1.1 (registry [..])")
        .with_stderr_contains("[DOWNLOADED] bar v0.2.1 (registry [..])")
        .run();
}

#[crabgo_test]
fn inline_version_without_name() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    crabgo_process("install @0.1.1")
        .with_status(101)
        .with_stderr("error: missing crate name for `@0.1.1`")
        .run();
}

#[crabgo_test]
fn inline_and_explicit_version() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    crabgo_process("install foo@0.1.1 --version 0.1.1")
        .with_status(101)
        .with_stderr("error: cannot specify both `@0.1.1` and `--version`")
        .run();
}

#[crabgo_test]
fn not_both_vers_and_version() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    crabgo_process("install foo --version 0.1.1 --vers 0.1.2")
        .with_status(1)
        .with_stderr_contains(
            "\
[ERROR] the argument '--version <VERSION>' cannot be used multiple times
",
        )
        .run();
}

#[crabgo_test]
fn test_install_git_cannot_be_a_base_url() {
    crabgo_process("install --git github.com:rust-lang/rustfmt.git")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] invalid url `github.com:rust-lang/rustfmt.git`: cannot-be-a-base-URLs are not supported",
        )
        .run();
}

#[crabgo_test]
fn uninstall_multiple_and_specifying_bin() {
    crabgo_process("uninstall foo bar --bin baz")
        .with_status(101)
        .with_stderr("\
[ERROR] A binary can only be associated with a single installed package, specifying multiple specs with --bin is redundant.")
        .run();
}

#[crabgo_test]
fn uninstall_with_empty_package_option() {
    crabgo_process("uninstall -p")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] \"--package <SPEC>\" requires a SPEC format value.
Run `crabgo help pkgid` for more information about SPEC format.
",
        )
        .run();
}

#[crabgo_test]
fn uninstall_multiple_and_some_pkg_does_not_exist() {
    pkg("foo", "0.0.1");

    crabgo_process("install foo").run();

    crabgo_process("uninstall foo bar")
        .with_status(101)
        .with_stderr(
            "\
[REMOVING] [CWD]/home/.crabgo/bin/foo[EXE]
error: package ID specification `bar` did not match any packages
[SUMMARY] Successfully uninstalled foo! Failed to uninstall bar (see error(s) above).
error: some packages failed to uninstall
",
        )
        .run();

    assert_has_not_installed_exe(cargo_home(), "foo");
    assert_has_not_installed_exe(cargo_home(), "bar");
}

#[crabgo_test]
fn custom_target_dir_for_git_source() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    crabgo_process("install --git")
        .arg(p.url().to_string())
        .run();
    assert!(!paths::root().join("target/release").is_dir());

    crabgo_process("install --force --git")
        .arg(p.url().to_string())
        .env("CRABGO_TARGET_DIR", "target")
        .run();
    assert!(paths::root().join("target/release").is_dir());
}

#[crabgo_test]
fn install_respects_lock_file() {
    // `crabgo install` now requires --locked to use a Crabgo.lock.
    Package::new("bar", "0.1.0").publish();
    Package::new("bar", "0.1.1")
        .file("src/lib.rs", "not rust")
        .publish();
    Package::new("foo", "0.1.0")
        .dep("bar", "0.1")
        .file("src/lib.rs", "")
        .file(
            "src/main.rs",
            "extern crate foo; extern crate bar; fn main() {}",
        )
        .file(
            "Crabgo.lock",
            r#"
[[package]]
name = "bar"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "foo"
version = "0.1.0"
dependencies = [
 "bar 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)",
]
"#,
        )
        .publish();

    crabgo_process("install foo")
        .with_stderr_contains("[..]not rust[..]")
        .with_status(101)
        .run();
    crabgo_process("install --locked foo").run();
}

#[crabgo_test]
fn install_path_respects_lock_file() {
    // --path version of install_path_respects_lock_file, --locked is required
    // to use Crabgo.lock.
    Package::new("bar", "0.1.0").publish();
    Package::new("bar", "0.1.1")
        .file("src/lib.rs", "not rust")
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
        .file("src/main.rs", "extern crate bar; fn main() {}")
        .file(
            "Crabgo.lock",
            r#"
[[package]]
name = "bar"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "foo"
version = "0.1.0"
dependencies = [
 "bar 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)",
]
"#,
        )
        .build();

    p.crabgo("install --path .")
        .with_stderr_contains("[..]not rust[..]")
        .with_status(101)
        .run();
    p.crabgo("install --path . --locked").run();
}

#[crabgo_test]
fn lock_file_path_deps_ok() {
    Package::new("bar", "0.1.0").publish();

    Package::new("foo", "0.1.0")
        .dep("bar", "0.1")
        .file("src/lib.rs", "")
        .file(
            "src/main.rs",
            "extern crate foo; extern crate bar; fn main() {}",
        )
        .file(
            "Crabgo.lock",
            r#"
            [[package]]
            name = "bar"
            version = "0.1.0"

            [[package]]
            name = "foo"
            version = "0.1.0"
            dependencies = [
             "bar 0.1.0",
            ]
            "#,
        )
        .publish();

    crabgo_process("install foo").run();
}

#[crabgo_test]
fn install_empty_argument() {
    // Bug 5229
    crabgo_process("install")
        .arg("")
        .with_status(1)
        .with_stderr_contains("[ERROR] a value is required for '[crate]...' but none was supplied")
        .run();
}

#[crabgo_test]
fn git_repo_replace() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Crabgo.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();
    let repo = git2::Repository::open(&p.root()).unwrap();
    let old_rev = repo.revparse_single("HEAD").unwrap().id();
    crabgo_process("install --git")
        .arg(p.url().to_string())
        .run();
    git::commit(&repo);
    let new_rev = repo.revparse_single("HEAD").unwrap().id();
    let mut path = paths::home();
    path.push(".crabgo/.crates.toml");

    assert_ne!(old_rev, new_rev);
    assert!(fs::read_to_string(path.clone())
        .unwrap()
        .contains(&format!("{}", old_rev)));
    crabgo_process("install --force --git")
        .arg(p.url().to_string())
        .run();
    assert!(fs::read_to_string(path)
        .unwrap()
        .contains(&format!("{}", new_rev)));
}

#[crabgo_test]
fn workspace_uses_workspace_target_dir() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [workspace]

                [dependencies]
                bar = { path = 'bar' }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Crabgo.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/main.rs", "fn main() {}")
        .build();

    p.crabgo("build --release").cwd("bar").run();
    crabgo_process("install --path")
        .arg(p.root().join("bar"))
        .with_stderr(
            "[INSTALLING] [..]
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]
[INSTALLED] package `bar v0.1.0 ([..]/bar)` (executable `bar[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
}

#[crabgo_test]
fn install_ignores_local_crabgo_config() {
    pkg("bar", "0.0.1");

    let p = project()
        .file(
            ".crabgo/config",
            r#"
                [build]
                target = "non-existing-target"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("install bar").run();
    assert_has_installed_exe(cargo_home(), "bar");
}

#[crabgo_test]
fn install_ignores_unstable_table_in_local_crabgo_config() {
    pkg("bar", "0.0.1");

    let p = project()
        .file(
            ".crabgo/config",
            r#"
                [unstable]
                build-std = ["core"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("install bar")
        .masquerade_as_nightly_crabgo(&["build-std"])
        .run();
    assert_has_installed_exe(cargo_home(), "bar");
}

#[crabgo_test]
fn install_global_crabgo_config() {
    pkg("bar", "0.0.1");

    let config = cargo_home().join("config");
    let mut toml = fs::read_to_string(&config).unwrap_or_default();

    toml.push_str(
        r#"
            [build]
            target = 'nonexistent'
        "#,
    );
    fs::write(&config, toml).unwrap();

    crabgo_process("install bar")
        .with_status(101)
        .with_stderr_contains("[..]--target nonexistent[..]")
        .run();
}

#[crabgo_test]
fn install_path_config() {
    project()
        .file(
            ".crabgo/config",
            r#"
            [build]
            target = 'nonexistent'
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();
    crabgo_process("install --path foo")
        .with_status(101)
        .with_stderr_contains("[..]--target nonexistent[..]")
        .run();
}

#[crabgo_test]
fn install_version_req() {
    // Try using a few versionreq styles.
    pkg("foo", "0.0.3");
    pkg("foo", "1.0.4");
    pkg("foo", "1.0.5");
    crabgo_process("install foo --version=*")
        .with_stderr_does_not_contain("[WARNING][..]is not a valid semver[..]")
        .with_stderr_contains("[INSTALLING] foo v1.0.5")
        .run();
    crabgo_process("uninstall foo").run();
    crabgo_process("install foo --version=^1.0")
        .with_stderr_does_not_contain("[WARNING][..]is not a valid semver[..]")
        .with_stderr_contains("[INSTALLING] foo v1.0.5")
        .run();
    crabgo_process("uninstall foo").run();
    crabgo_process("install foo --version=0.0.*")
        .with_stderr_does_not_contain("[WARNING][..]is not a valid semver[..]")
        .with_stderr_contains("[INSTALLING] foo v0.0.3")
        .run();
}

#[crabgo_test]
fn git_install_reads_workspace_manifest() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Crabgo.toml",
            r#"
            [workspace]
            members = ["bin1"]

            [profile.release]
            incremental = 3
            "#,
        )
        .file("bin1/Crabgo.toml", &basic_manifest("bin1", "0.1.0"))
        .file(
            "bin1/src/main.rs",
            r#"fn main() { println!("Hello, world!"); }"#,
        )
        .build();

    crabgo_process(&format!("install --git {}", p.url().to_string()))
        .with_status(101)
        .with_stderr_contains("  invalid type: integer `3`[..]")
        .run();
}

#[crabgo_test]
fn install_git_with_symlink_home() {
    // Ensure that `crabgo install` with a git repo is OK when CRABGO_HOME is a
    // symlink, and uses an build script.
    if !symlink_supported() {
        return;
    }
    let p = git::new("foo", |p| {
        p.file("Crabgo.toml", &basic_manifest("foo", "1.0.0"))
            .file("src/main.rs", "fn main() {}")
            // This triggers discover_git_and_list_files for detecting changed files.
            .file("build.rs", "fn main() {}")
    });
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(windows)]
    use std::os::windows::fs::symlink_dir as symlink;

    let actual = paths::root().join("actual-home");
    t!(std::fs::create_dir(&actual));
    t!(symlink(&actual, paths::home().join(".crabgo")));
    crabgo_process("install --git")
        .arg(p.url().to_string())
        .with_stderr(
            "\
[UPDATING] git repository [..]
[INSTALLING] foo v1.0.0 [..]
[COMPILING] foo v1.0.0 [..]
[FINISHED] [..]
[INSTALLING] [..]home/.crabgo/bin/foo[..]
[INSTALLED] package `foo [..]
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[crabgo_test]
fn install_yanked_crabgo_package() {
    Package::new("baz", "0.0.1").yanked(true).publish();
    crabgo_process("install baz --version 0.0.1")
        .with_status(101)
        .with_stderr_contains(
            "\
[ERROR] cannot install package `baz`, it has been yanked from registry `crates-io`
",
        )
        .run();
}

#[crabgo_test]
fn install_crabgo_package_in_a_patched_workspace() {
    pkg("foo", "0.1.0");
    pkg("fizz", "1.0.0");

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["baz"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "baz/Crabgo.toml",
            r#"
                [package]
                name = "baz"
                version = "0.1.0"
                authors = []

                [dependencies]
                fizz = "1"

                [patch.crates-io]
                fizz = { version = "=1.0.0" }
            "#,
        )
        .file("baz/src/lib.rs", "")
        .build();

    let stderr = "\
[WARNING] patch for the non root package will be ignored, specify patch at the workspace root:
package:   [..]/foo/baz/Crabgo.toml
workspace: [..]/foo/Crabgo.toml
";
    p.crabgo("check").with_stderr_contains(&stderr).run();

    // A crate installation must not emit any message from a workspace under
    // current working directory.
    // See https://github.com/rust-lang/crabgo/issues/8619
    p.crabgo("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.1.0 (registry [..])
[INSTALLING] foo v0.1.0
[COMPILING] foo v0.1.0
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]foo[EXE]
[INSTALLED] package `foo v0.1.0` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn locked_install_without_published_lockfile() {
    Package::new("foo", "0.1.0")
        .file("src/main.rs", "//! Some docs\nfn main() {}")
        .publish();

    crabgo_process("install foo --locked")
        .with_stderr_contains("[WARNING] no Crabgo.lock file published in foo v0.1.0")
        .run();
}

#[crabgo_test]
fn install_semver_metadata() {
    // Check trying to install a package that uses semver metadata.
    // This uses alt registry because the bug this is exercising doesn't
    // trigger with a replaced source.
    registry::alt_init();
    Package::new("foo", "1.0.0+abc")
        .alternative(true)
        .file("src/main.rs", "fn main() {}")
        .publish();

    crabgo_process("install foo --registry alternative --version 1.0.0+abc").run();
    crabgo_process("install foo --registry alternative")
        .with_stderr("\
[UPDATING] `alternative` index
[IGNORED] package `foo v1.0.0+abc (registry `alternative`)` is already installed, use --force to override
[WARNING] be sure to add [..]
")
        .run();
    // "Updating" is not displayed here due to the --version fast-path.
    crabgo_process("install foo --registry alternative --version 1.0.0+abc")
        .with_stderr("\
[IGNORED] package `foo v1.0.0+abc (registry `alternative`)` is already installed, use --force to override
[WARNING] be sure to add [..]
")
        .run();
    crabgo_process("install foo --registry alternative --version 1.0.0 --force")
        .with_stderr(
            "\
[UPDATING] `alternative` index
[INSTALLING] foo v1.0.0+abc (registry `alternative`)
[COMPILING] foo v1.0.0+abc (registry `alternative`)
[FINISHED] [..]
[REPLACING] [ROOT]/home/.crabgo/bin/foo[EXE]
[REPLACED] package [..]
[WARNING] be sure to add [..]
",
        )
        .run();
    // Check that from a fresh cache will work without metadata, too.
    paths::home().join(".crabgo/registry").rm_rf();
    paths::home().join(".crabgo/bin").rm_rf();
    crabgo_process("install foo --registry alternative --version 1.0.0")
        .with_stderr(
            "\
[UPDATING] `alternative` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v1.0.0+abc (registry `alternative`)
[INSTALLING] foo v1.0.0+abc (registry `alternative`)
[COMPILING] foo v1.0.0+abc (registry `alternative`)
[FINISHED] [..]
[INSTALLING] [ROOT]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v1.0.0+abc (registry `alternative`)` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[crabgo_test]
fn no_auto_fix_note() {
    Package::new("auto_fix", "0.0.1")
        .file("src/lib.rs", "use std::io;")
        .file(
            "src/main.rs",
            &format!("extern crate {}; use std::io; fn main() {{}}", "auto_fix"),
        )
        .publish();

    // This should not contain a suggestion to run `crabgo fix`
    //
    // This is checked by matching the full output as `with_stderr_does_not_contain`
    // can be brittle
    crabgo_process("install auto_fix")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] auto_fix v0.0.1 (registry [..])
[INSTALLING] auto_fix v0.0.1
[COMPILING] auto_fix v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/auto_fix[EXE]
[INSTALLED] package `auto_fix v0.0.1` (executable `auto_fix[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "auto_fix");

    crabgo_process("uninstall auto_fix")
        .with_stderr("[REMOVING] [CWD]/home/.crabgo/bin/auto_fix[EXE]")
        .run();
    assert_has_not_installed_exe(cargo_home(), "auto_fix");
}

#[crabgo_test]
fn failed_install_retains_temp_directory() {
    // Verifies that the temporary directory persists after a build failure.
    Package::new("foo", "0.0.1")
        .file("src/main.rs", "x")
        .publish();
    let err = crabgo_process("install foo").exec_with_output().unwrap_err();
    let err = err.downcast::<ProcessError>().unwrap();
    let stderr = String::from_utf8(err.stderr.unwrap()).unwrap();
    compare::match_contains(
        "\
[UPDATING] `dummy-registry` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `dummy-registry`)
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
",
        &stderr,
        None,
    )
    .unwrap();
    compare::match_contains(
        "error: failed to compile `foo v0.0.1`, intermediate artifacts can be found at `[..]`",
        &stderr,
        None,
    )
    .unwrap();

    // Find the path in the output.
    let start = stderr.find("found at `").unwrap() + 10;
    let end = stderr[start..].find('\n').unwrap() - 1;
    let path = Path::new(&stderr[start..(end + start)]);
    assert!(path.exists());
    assert!(path.join("release/deps").exists());
}

#[crabgo_test]
fn sparse_install() {
    // Checks for an issue where uninstalling something corrupted
    // the SourceIds of sparse registries.
    // See https://github.com/rust-lang/crabgo/issues/11751
    let _registry = registry::RegistryBuilder::new().http_index().build();

    pkg("foo", "0.0.1");
    pkg("bar", "0.0.1");

    crabgo_process("install foo --registry dummy-registry")
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `dummy-registry`)
[INSTALLING] foo v0.0.1 (registry `dummy-registry`)
[UPDATING] `dummy-registry` index
[COMPILING] foo v0.0.1 (registry `dummy-registry`)
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [ROOT]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1 (registry `dummy-registry`)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
    let assert_v1 = |expected| {
        let v1 = fs::read_to_string(paths::home().join(".crabgo/.crates.toml")).unwrap();
        compare::assert_match_exact(expected, &v1);
    };
    assert_v1(
        r#"[v1]
"foo 0.0.1 (sparse+http://127.0.0.1:[..]/index/)" = ["foo[EXE]"]
"#,
    );
    crabgo_process("install bar").run();
    assert_has_installed_exe(cargo_home(), "bar");
    assert_v1(
        r#"[v1]
"bar 0.0.1 (registry+https://github.com/rust-lang/crates.io-index)" = ["bar[EXE]"]
"foo 0.0.1 (sparse+http://127.0.0.1:[..]/index/)" = ["foo[EXE]"]
"#,
    );

    crabgo_process("uninstall bar")
        .with_stderr("[REMOVING] [CWD]/home/.crabgo/bin/bar[EXE]")
        .run();
    assert_has_not_installed_exe(cargo_home(), "bar");
    assert_v1(
        r#"[v1]
"foo 0.0.1 (sparse+http://127.0.0.1:[..]/index/)" = ["foo[EXE]"]
"#,
    );
    crabgo_process("uninstall foo")
        .with_stderr("[REMOVING] [CWD]/home/.crabgo/bin/foo[EXE]")
        .run();
    assert_has_not_installed_exe(cargo_home(), "foo");
    assert_v1(
        r#"[v1]
"#,
    );
}

#[crabgo_test]
fn self_referential() {
    // Some packages build-dep on prior versions of themselves.
    Package::new("foo", "0.0.1")
        .file("src/lib.rs", "fn hello() {}")
        .file("src/main.rs", "fn main() {}")
        .file("build.rs", "fn main() {}")
        .publish();
    Package::new("foo", "0.0.2")
        .file("src/lib.rs", "fn hello() {}")
        .file("src/main.rs", "fn main() {}")
        .file("build.rs", "fn main() {}")
        .build_dep("foo", "0.0.1")
        .publish();

    crabgo_process("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.2 (registry [..])
[INSTALLING] foo v0.0.2
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry [..])
[COMPILING] foo v0.0.1
[COMPILING] foo v0.0.2
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.2` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}

#[crabgo_test]
fn ambiguous_registry_vs_local_package() {
    // Correctly install 'foo' from a local package, even if that package also
    // depends on a registry dependency named 'foo'.
    Package::new("foo", "0.0.1")
        .file("src/lib.rs", "fn hello() {}")
        .publish();

    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            "Crabgo.toml",
            r#"
        [package]
        name = "foo"
        version = "0.1.0"
        authors = []
        edition = "2021"

        [dependencies]
        foo = "0.0.1"
    "#,
        )
        .build();

    crabgo_process("install --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.1.0 ([..])
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry [..])
[COMPILING] foo v0.0.1
[COMPILING] foo v0.1.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.crabgo/bin/foo[EXE]
[INSTALLED] package `foo v0.1.0 ([..])` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(cargo_home(), "foo");
}
