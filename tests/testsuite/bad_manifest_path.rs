//! Tests for invalid --manifest-path arguments.

use crabgo_test_support::{basic_bin_manifest, main_file, project};

#[track_caller]
fn assert_not_a_crabgo_toml(command: &str, manifest_path_argument: &str) {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo(command)
        .arg("--manifest-path")
        .arg(manifest_path_argument)
        .cwd(p.root().parent().unwrap())
        .with_status(101)
        .with_stderr(
            "[ERROR] the manifest-path must be a path \
             to a Crabgo.toml file",
        )
        .run();
}

#[track_caller]
fn assert_crabgo_toml_doesnt_exist(command: &str, manifest_path_argument: &str) {
    let p = project().build();
    let expected_path = manifest_path_argument
        .split('/')
        .collect::<Vec<_>>()
        .join("[..]");

    p.crabgo(command)
        .arg("--manifest-path")
        .arg(manifest_path_argument)
        .cwd(p.root().parent().unwrap())
        .with_status(101)
        .with_stderr(format!(
            "[ERROR] manifest path `{}` does not exist",
            expected_path
        ))
        .run();
}

#[crabgo_test]
fn bench_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("bench", "foo");
}

#[crabgo_test]
fn bench_dir_plus_file() {
    assert_not_a_crabgo_toml("bench", "foo/bar");
}

#[crabgo_test]
fn bench_dir_plus_path() {
    assert_not_a_crabgo_toml("bench", "foo/bar/baz");
}

#[crabgo_test]
fn bench_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("bench", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn build_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("check", "foo");
}

#[crabgo_test]
fn build_dir_plus_file() {
    assert_not_a_crabgo_toml("bench", "foo/bar");
}

#[crabgo_test]
fn build_dir_plus_path() {
    assert_not_a_crabgo_toml("bench", "foo/bar/baz");
}

#[crabgo_test]
fn build_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("check", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn clean_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("clean", "foo");
}

#[crabgo_test]
fn clean_dir_plus_file() {
    assert_not_a_crabgo_toml("clean", "foo/bar");
}

#[crabgo_test]
fn clean_dir_plus_path() {
    assert_not_a_crabgo_toml("clean", "foo/bar/baz");
}

#[crabgo_test]
fn clean_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("clean", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn doc_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("doc", "foo");
}

#[crabgo_test]
fn doc_dir_plus_file() {
    assert_not_a_crabgo_toml("doc", "foo/bar");
}

#[crabgo_test]
fn doc_dir_plus_path() {
    assert_not_a_crabgo_toml("doc", "foo/bar/baz");
}

#[crabgo_test]
fn doc_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("doc", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn fetch_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("fetch", "foo");
}

#[crabgo_test]
fn fetch_dir_plus_file() {
    assert_not_a_crabgo_toml("fetch", "foo/bar");
}

#[crabgo_test]
fn fetch_dir_plus_path() {
    assert_not_a_crabgo_toml("fetch", "foo/bar/baz");
}

#[crabgo_test]
fn fetch_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("fetch", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn generate_lockfile_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("generate-lockfile", "foo");
}

#[crabgo_test]
fn generate_lockfile_dir_plus_file() {
    assert_not_a_crabgo_toml("generate-lockfile", "foo/bar");
}

#[crabgo_test]
fn generate_lockfile_dir_plus_path() {
    assert_not_a_crabgo_toml("generate-lockfile", "foo/bar/baz");
}

#[crabgo_test]
fn generate_lockfile_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("generate-lockfile", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn package_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("package", "foo");
}

#[crabgo_test]
fn package_dir_plus_file() {
    assert_not_a_crabgo_toml("package", "foo/bar");
}

#[crabgo_test]
fn package_dir_plus_path() {
    assert_not_a_crabgo_toml("package", "foo/bar/baz");
}

#[crabgo_test]
fn package_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("package", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn pkgid_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("pkgid", "foo");
}

#[crabgo_test]
fn pkgid_dir_plus_file() {
    assert_not_a_crabgo_toml("pkgid", "foo/bar");
}

#[crabgo_test]
fn pkgid_dir_plus_path() {
    assert_not_a_crabgo_toml("pkgid", "foo/bar/baz");
}

#[crabgo_test]
fn pkgid_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("pkgid", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn publish_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("publish", "foo");
}

#[crabgo_test]
fn publish_dir_plus_file() {
    assert_not_a_crabgo_toml("publish", "foo/bar");
}

#[crabgo_test]
fn publish_dir_plus_path() {
    assert_not_a_crabgo_toml("publish", "foo/bar/baz");
}

#[crabgo_test]
fn publish_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("publish", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn read_manifest_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("read-manifest", "foo");
}

#[crabgo_test]
fn read_manifest_dir_plus_file() {
    assert_not_a_crabgo_toml("read-manifest", "foo/bar");
}

#[crabgo_test]
fn read_manifest_dir_plus_path() {
    assert_not_a_crabgo_toml("read-manifest", "foo/bar/baz");
}

#[crabgo_test]
fn read_manifest_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("read-manifest", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn run_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("run", "foo");
}

#[crabgo_test]
fn run_dir_plus_file() {
    assert_not_a_crabgo_toml("run", "foo/bar");
}

#[crabgo_test]
fn run_dir_plus_path() {
    assert_not_a_crabgo_toml("run", "foo/bar/baz");
}

#[crabgo_test]
fn run_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("run", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn rustc_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("rustc", "foo");
}

#[crabgo_test]
fn rustc_dir_plus_file() {
    assert_not_a_crabgo_toml("rustc", "foo/bar");
}

#[crabgo_test]
fn rustc_dir_plus_path() {
    assert_not_a_crabgo_toml("rustc", "foo/bar/baz");
}

#[crabgo_test]
fn rustc_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("rustc", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn test_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("test", "foo");
}

#[crabgo_test]
fn test_dir_plus_file() {
    assert_not_a_crabgo_toml("test", "foo/bar");
}

#[crabgo_test]
fn test_dir_plus_path() {
    assert_not_a_crabgo_toml("test", "foo/bar/baz");
}

#[crabgo_test]
fn test_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("test", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn update_dir_containing_crabgo_toml() {
    assert_not_a_crabgo_toml("update", "foo");
}

#[crabgo_test]
fn update_dir_plus_file() {
    assert_not_a_crabgo_toml("update", "foo/bar");
}

#[crabgo_test]
fn update_dir_plus_path() {
    assert_not_a_crabgo_toml("update", "foo/bar/baz");
}

#[crabgo_test]
fn update_dir_to_nonexistent_crabgo_toml() {
    assert_crabgo_toml_doesnt_exist("update", "foo/bar/baz/Crabgo.toml");
}

#[crabgo_test]
fn verify_project_dir_containing_crabgo_toml() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("verify-project --manifest-path foo")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"the manifest-path must be a path to a Crabgo.toml file\"}\
             ",
        )
        .run();
}

#[crabgo_test]
fn verify_project_dir_plus_file() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("verify-project --manifest-path foo/bar")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"the manifest-path must be a path to a Crabgo.toml file\"}\
             ",
        )
        .run();
}

#[crabgo_test]
fn verify_project_dir_plus_path() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("verify-project --manifest-path foo/bar/baz")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"the manifest-path must be a path to a Crabgo.toml file\"}\
             ",
        )
        .run();
}

#[crabgo_test]
fn verify_project_dir_to_nonexistent_crabgo_toml() {
    let p = project().build();
    p.crabgo("verify-project --manifest-path foo/bar/baz/Crabgo.toml")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"manifest path `foo[..]bar[..]baz[..]Crabgo.toml` does not exist\"}\
             ",
        )
        .run();
}
