use crabgo_test_support::paths;
use crabgo_test_support::prelude::*;
use std::fs;

use crabgo_test_support::curr_dir;

#[crabgo_test]
fn case() {
    let project_root = &paths::root().join("test");
    fs::create_dir_all(project_root).unwrap();

    snapbox::cmd::Command::crabgo_ui()
        .arg_line("init")
        .current_dir(project_root)
        .assert()
        .code(101)
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert!(!project_root.join("Crabgo.toml").is_file());
}
