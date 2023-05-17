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
    let (git_dep, git_repo) = crabgo_test_support::git::new_repo("git-package", |project| {
        project
            .file(
                "Crabgo.toml",
                &crabgo_test_support::basic_manifest("git-package", "0.3.0+git-package"),
            )
            .file("src/lib.rs", "")
    });
    let find_head = || (git_repo.head().unwrap().peel_to_commit().unwrap());
    let head = find_head().id().to_string();
    let git_url = git_dep.url().to_string();

    snapbox::cmd::Command::crabgo_ui()
        .arg("add")
        .args(["git-package", "--git", &git_url, "--rev", &head])
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
