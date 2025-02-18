use crabgo_test_support::compare::assert_ui;
use crabgo_test_support::prelude::*;
use crabgo_test_support::Project;

use crate::crabgo_add::init_alt_registry;

#[crabgo_test]
fn case() {
    init_alt_registry();
    let project =
        Project::from_template("tests/testsuite/crabgo_add/build_prefer_existing_version/in");
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::crabgo_ui()
        .arg("add")
        .arg_line("crabgo-list-test-fixture-dependency --build")
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path("tests/testsuite/crabgo_add/build_prefer_existing_version/stdout.log")
        .stderr_matches_path("tests/testsuite/crabgo_add/build_prefer_existing_version/stderr.log");

    assert_ui().subset_matches(
        "tests/testsuite/crabgo_add/build_prefer_existing_version/out",
        &project_root,
    );
}
