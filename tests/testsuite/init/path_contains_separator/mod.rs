use crabgo_test_support::compare::assert_ui;
use crabgo_test_support::prelude::*;
use crabgo_test_support::{t, Project};

use crabgo_test_support::curr_dir;

#[crabgo_test]
fn case() {
    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = &project.root().join("test:ing");

    if !project_root.exists() {
        t!(std::fs::create_dir(&project_root));
    }

    snapbox::cmd::Command::crabgo_ui()
        .arg_line("init --bin --vcs none --edition 2015 --name testing")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), project_root);
    assert!(!project_root.join(".gitignore").is_file());
}
