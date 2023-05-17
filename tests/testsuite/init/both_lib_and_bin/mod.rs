use crabgo_test_support::paths;
use crabgo_test_support::prelude::*;

use crabgo_test_support::curr_dir;

#[crabgo_test]
fn case() {
    let cwd = paths::root();

    snapbox::cmd::Command::crabgo_ui()
        .arg_line("init --lib --bin")
        .current_dir(&cwd)
        .assert()
        .code(101)
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert!(!cwd.join("Crabgo.toml").is_file());
}
