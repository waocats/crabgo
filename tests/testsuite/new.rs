//! Tests for the `crabgo new` command.

use crabgo_test_support::crabgo_process;
use crabgo_test_support::paths;
use std::env;
use std::fs::{self, File};

fn create_default_gitconfig() {
    // This helps on Windows where libgit2 is very aggressive in attempting to
    // find a git config file.
    let gitconfig = paths::home().join(".gitconfig");
    File::create(gitconfig).unwrap();

    // If we're running this under a user account that has a different default branch set up
    // then tests that assume the default branch is master will fail. We set the default branch
    // to master explicitly so that tests that rely on this behavior still pass.
    fs::write(
        paths::home().join(".gitconfig"),
        r#"
        [init]
            defaultBranch = master
        "#,
    )
    .unwrap();
}

#[crabgo_test]
fn simple_lib() {
    crabgo_process("new --lib foo --vcs none --edition 2015")
        .with_stderr("[CREATED] library `foo` package")
        .run();

    assert!(paths::root().join("foo").is_dir());
    assert!(paths::root().join("foo/Crabgo.toml").is_file());
    assert!(paths::root().join("foo/src/lib.rs").is_file());
    assert!(!paths::root().join("foo/.gitignore").is_file());

    let lib = paths::root().join("foo/src/lib.rs");
    let contents = fs::read_to_string(&lib).unwrap();
    assert_eq!(
        contents,
        r#"pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
"#
    );

    crabgo_process("build").cwd(&paths::root().join("foo")).run();
}

#[crabgo_test]
fn simple_bin() {
    crabgo_process("new --bin foo --edition 2015")
        .with_stderr("[CREATED] binary (application) `foo` package")
        .run();

    assert!(paths::root().join("foo").is_dir());
    assert!(paths::root().join("foo/Crabgo.toml").is_file());
    assert!(paths::root().join("foo/src/main.rs").is_file());

    crabgo_process("build").cwd(&paths::root().join("foo")).run();
    assert!(paths::root()
        .join(&format!("foo/target/debug/foo{}", env::consts::EXE_SUFFIX))
        .is_file());
}

#[crabgo_test]
fn both_lib_and_bin() {
    crabgo_process("new --lib --bin foo")
        .with_status(101)
        .with_stderr("[ERROR] can't specify both lib and binary outputs")
        .run();
}

#[crabgo_test]
fn simple_git() {
    crabgo_process("new --lib foo --edition 2015").run();

    assert!(paths::root().is_dir());
    assert!(paths::root().join("foo/Crabgo.toml").is_file());
    assert!(paths::root().join("foo/src/lib.rs").is_file());
    assert!(paths::root().join("foo/.git").is_dir());
    assert!(paths::root().join("foo/.gitignore").is_file());

    let fp = paths::root().join("foo/.gitignore");
    let contents = fs::read_to_string(&fp).unwrap();
    assert_eq!(contents, "/target\n/Crabgo.lock\n",);

    crabgo_process("build").cwd(&paths::root().join("foo")).run();
}

#[crabgo_test(requires_hg)]
fn simple_hg() {
    crabgo_process("new --lib foo --edition 2015 --vcs hg").run();

    assert!(paths::root().is_dir());
    assert!(paths::root().join("foo/Crabgo.toml").is_file());
    assert!(paths::root().join("foo/src/lib.rs").is_file());
    assert!(paths::root().join("foo/.hg").is_dir());
    assert!(paths::root().join("foo/.hgignore").is_file());

    let fp = paths::root().join("foo/.hgignore");
    let contents = fs::read_to_string(&fp).unwrap();
    assert_eq!(contents, "^target$\n^Crabgo.lock$\n",);

    crabgo_process("build").cwd(&paths::root().join("foo")).run();
}

#[crabgo_test]
fn no_argument() {
    crabgo_process("new")
        .with_status(1)
        .with_stderr_contains(
            "\
error: the following required arguments were not provided:
  <path>
",
        )
        .run();
}

#[crabgo_test]
fn existing() {
    let dst = paths::root().join("foo");
    fs::create_dir(&dst).unwrap();
    crabgo_process("new foo")
        .with_status(101)
        .with_stderr(
            "[ERROR] destination `[CWD]/foo` already exists\n\n\
             Use `crabgo init` to initialize the directory",
        )
        .run();
}

#[crabgo_test]
fn invalid_characters() {
    crabgo_process("new foo.rs")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] invalid character `.` in package name: `foo.rs`, [..]
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"foo.rs\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/foo.rs.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"foo.rs\"
    path = \"src/main.rs\"

",
        )
        .run();
}

#[crabgo_test]
fn reserved_name() {
    crabgo_process("new test")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the name `test` cannot be used as a package name, it conflicts [..]
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"test\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/test.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"test\"
    path = \"src/main.rs\"

",
        )
        .run();
}

#[crabgo_test]
fn reserved_binary_name() {
    crabgo_process("new --bin incremental")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the name `incremental` cannot be used as a package name, it conflicts [..]
If you need a package name to not match the directory name, consider using --name flag.
",
        )
        .run();

    crabgo_process("new --lib incremental")
        .with_stderr(
            "\
[WARNING] the name `incremental` will not support binary executables with that name, \
it conflicts with crabgo's build directory names
[CREATED] library `incremental` package
",
        )
        .run();
}

#[crabgo_test]
fn keyword_name() {
    crabgo_process("new pub")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the name `pub` cannot be used as a package name, it is a Rust keyword
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"pub\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/pub.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"pub\"
    path = \"src/main.rs\"

",
        )
        .run();
}

#[crabgo_test]
fn std_name() {
    crabgo_process("new core")
        .with_stderr(
            "\
[WARNING] the name `core` is part of Rust's standard library
It is recommended to use a different name to avoid problems.
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"core\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/core.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"core\"
    path = \"src/main.rs\"

[CREATED] binary (application) `core` package
",
        )
        .run();
}

#[crabgo_test]
fn git_prefers_command_line() {
    let root = paths::root();
    fs::create_dir(&root.join(".crabgo")).unwrap();
    fs::write(
        &root.join(".crabgo/config"),
        r#"
            [crabgo-new]
            vcs = "none"
            name = "foo"
            email = "bar"
        "#,
    )
    .unwrap();

    crabgo_process("new foo --vcs git").run();
    assert!(paths::root().join("foo/.gitignore").exists());
    assert!(!fs::read_to_string(paths::root().join("foo/Crabgo.toml"))
        .unwrap()
        .contains("authors ="));
}

#[crabgo_test]
fn subpackage_no_git() {
    crabgo_process("new foo").run();

    assert!(paths::root().join("foo/.git").is_dir());
    assert!(paths::root().join("foo/.gitignore").is_file());

    let subpackage = paths::root().join("foo").join("components");
    fs::create_dir(&subpackage).unwrap();
    crabgo_process("new foo/components/subcomponent").run();

    assert!(!paths::root()
        .join("foo/components/subcomponent/.git")
        .is_file());
    assert!(!paths::root()
        .join("foo/components/subcomponent/.gitignore")
        .is_file());
}

#[crabgo_test]
fn subpackage_git_with_gitignore() {
    crabgo_process("new foo").run();

    assert!(paths::root().join("foo/.git").is_dir());
    assert!(paths::root().join("foo/.gitignore").is_file());

    let gitignore = paths::root().join("foo/.gitignore");
    fs::write(gitignore, b"components").unwrap();

    let subpackage = paths::root().join("foo/components");
    fs::create_dir(&subpackage).unwrap();
    crabgo_process("new foo/components/subcomponent").run();

    assert!(paths::root()
        .join("foo/components/subcomponent/.git")
        .is_dir());
    assert!(paths::root()
        .join("foo/components/subcomponent/.gitignore")
        .is_file());
}

#[crabgo_test]
fn subpackage_git_with_vcs_arg() {
    crabgo_process("new foo").run();

    let subpackage = paths::root().join("foo").join("components");
    fs::create_dir(&subpackage).unwrap();
    crabgo_process("new foo/components/subcomponent --vcs git").run();

    assert!(paths::root()
        .join("foo/components/subcomponent/.git")
        .is_dir());
    assert!(paths::root()
        .join("foo/components/subcomponent/.gitignore")
        .is_file());
}

#[crabgo_test]
fn unknown_flags() {
    crabgo_process("new foo --flag")
        .with_status(1)
        .with_stderr_contains("error: unexpected argument '--flag' found")
        .run();
}

#[crabgo_test]
fn explicit_invalid_name_not_suggested() {
    crabgo_process("new --name 10-invalid a")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the name `10-invalid` cannot be used as a package name, \
the name cannot start with a digit\n\
If you need a binary with the name \"10-invalid\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/10-invalid.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"10-invalid\"
    path = \"src/main.rs\"

",
        )
        .run();
}

#[crabgo_test]
fn explicit_project_name() {
    crabgo_process("new --lib foo --name bar")
        .with_stderr("[CREATED] library `bar` package")
        .run();
}

#[crabgo_test]
fn new_with_edition_2015() {
    crabgo_process("new --edition 2015 foo").run();
    let manifest = fs::read_to_string(paths::root().join("foo/Crabgo.toml")).unwrap();
    assert!(manifest.contains("edition = \"2015\""));
}

#[crabgo_test]
fn new_with_edition_2018() {
    crabgo_process("new --edition 2018 foo").run();
    let manifest = fs::read_to_string(paths::root().join("foo/Crabgo.toml")).unwrap();
    assert!(manifest.contains("edition = \"2018\""));
}

#[crabgo_test]
fn new_default_edition() {
    crabgo_process("new foo").run();
    let manifest = fs::read_to_string(paths::root().join("foo/Crabgo.toml")).unwrap();
    assert!(manifest.contains("edition = \"2021\""));
}

#[crabgo_test]
fn new_with_bad_edition() {
    crabgo_process("new --edition something_else foo")
        .with_stderr_contains("error: invalid value 'something_else' for '--edition <YEAR>'")
        .with_status(1)
        .run();
}

#[crabgo_test]
fn new_with_reference_link() {
    crabgo_process("new foo").run();

    let contents = fs::read_to_string(paths::root().join("foo/Crabgo.toml")).unwrap();
    assert!(contents.contains("# See more keys and their definitions at https://doc.rust-lang.org/crabgo/reference/manifest.html"))
}

#[crabgo_test]
fn lockfile_constant_during_new() {
    crabgo_process("new foo").run();

    crabgo_process("build").cwd(&paths::root().join("foo")).run();
    let before = fs::read_to_string(paths::root().join("foo/Crabgo.lock")).unwrap();
    crabgo_process("build").cwd(&paths::root().join("foo")).run();
    let after = fs::read_to_string(paths::root().join("foo/Crabgo.lock")).unwrap();
    assert_eq!(before, after);
}

#[crabgo_test]
fn restricted_windows_name() {
    if cfg!(windows) {
        crabgo_process("new nul")
            .with_status(101)
            .with_stderr(
                "\
[ERROR] cannot use name `nul`, it is a reserved Windows filename
If you need a package name to not match the directory name, consider using --name flag.
",
            )
            .run();
    } else {
        crabgo_process("new nul")
            .with_stderr(
                "\
[WARNING] the name `nul` is a reserved Windows filename
This package will not work on Windows platforms.
[CREATED] binary (application) `nul` package
",
            )
            .run();
    }
}

#[crabgo_test]
fn non_ascii_name() {
    crabgo_process("new Привет")
        .with_stderr(
            "\
[WARNING] the name `Привет` contains non-ASCII characters
Non-ASCII crate names are not supported by Rust.
[CREATED] binary (application) `Привет` package
",
        )
        .run();
}

#[crabgo_test]
fn non_ascii_name_invalid() {
    // These are alphanumeric characters, but not Unicode XID.
    crabgo_process("new ⒶⒷⒸ")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] invalid character `Ⓐ` in package name: `ⒶⒷⒸ`, \
the first character must be a Unicode XID start character (most letters or `_`)
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"ⒶⒷⒸ\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/ⒶⒷⒸ.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"ⒶⒷⒸ\"
    path = \"src/main.rs\"

",
        )
        .run();

    crabgo_process("new a¼")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] invalid character `¼` in package name: `a¼`, \
characters must be Unicode XID characters (numbers, `-`, `_`, or most letters)
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"a¼\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/a¼.rs` \
or change the name in Crabgo.toml with:

    [[bin]]
    name = \"a¼\"
    path = \"src/main.rs\"

",
        )
        .run();
}

#[crabgo_test]
fn git_default_branch() {
    // Check for init.defaultBranch support.
    create_default_gitconfig();

    crabgo_process("new foo").run();
    let repo = git2::Repository::open(paths::root().join("foo")).unwrap();
    let head = repo.find_reference("HEAD").unwrap();
    assert_eq!(head.symbolic_target().unwrap(), "refs/heads/master");

    fs::write(
        paths::home().join(".gitconfig"),
        r#"
        [init]
            defaultBranch = hello
        "#,
    )
    .unwrap();
    crabgo_process("new bar").run();
    let repo = git2::Repository::open(paths::root().join("bar")).unwrap();
    let head = repo.find_reference("HEAD").unwrap();
    assert_eq!(head.symbolic_target().unwrap(), "refs/heads/hello");
}

#[crabgo_test]
fn non_utf8_str_in_ignore_file() {
    let gitignore = paths::home().join(".gitignore");
    File::create(gitignore).unwrap();

    fs::write(paths::home().join(".gitignore"), &[0xFF, 0xFE]).unwrap();

    crabgo_process(&format!("init {} --vcs git", paths::home().display()))
        .with_status(101)
        .with_stderr(
            "\
error: Failed to create package `home` at `[..]`

Caused by:
  Character at line 0 is invalid. Crabgo only supports UTF-8.
",
        )
        .run();
}

#[cfg(unix)]
#[crabgo_test]
fn path_with_invalid_character() {
    crabgo_process("new --name testing test:ing")
        .with_stderr(
            "\
[WARNING] the path `[CWD]/test:ing` contains invalid PATH characters (usually `:`, `;`, or `\"`)
It is recommended to use a different name to avoid problems.
[CREATED] binary (application) `testing` package
",
        )
        .run();
}
