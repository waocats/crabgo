use crabgo_test_support::paths;
use crabgo_test_support::prelude::*;
use std::fs;

use crabgo_test_support::curr_dir;

#[crabgo_test]
fn case() {
    let foo = &paths::root().join("foo.bar");
    fs::create_dir_all(foo).unwrap();

    snapbox::cmd::Command::crabgo_ui()
        .arg_line("init")
        .current_dir(foo)
        .assert()
        .code(101)
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert!(!foo.join("Crabgo.toml").is_file());
}
