use crabgo_test_support::compare::assert_ui;
use crabgo_test_support::prelude::*;
use crabgo_test_support::Project;

use crate::crabgo_add::init_registry;
use crabgo_test_support::curr_dir;

#[crabgo_test]
fn case() {
    init_registry();
    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::crabgo_ui()
        .arg("add")
        .arg_line("crabgo-list-test-fixture --path ./tests/fixtures/local")
        .current_dir(cwd)
        .assert()
        .code(101)
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
