use crabgo_test_support::compare::assert_ui;
use crabgo_test_support::prelude::*;
use crabgo_test_support::{process, Project};

use crabgo_test_support::curr_dir;

#[crabgo_test]
fn case() {
    // This cannot use `requires_rustfmt` because rustfmt is not available in
    // the rust-lang/rust environment. Additionally, if running crabgo without
    // rustup (but with rustup installed), this test also fails due to HOME
    // preventing the proxy from choosing a toolchain.
    if let Err(e) = process("rustfmt").arg("-V").exec_with_output() {
        eprintln!("skipping test, rustfmt not available:\n{e:?}");
        return;
    }
    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = &project.root();

    snapbox::cmd::Command::crabgo_ui()
        .arg_line("init --lib --vcs none")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), project_root);
}
