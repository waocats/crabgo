//! Tests for corrupt git repos.

use crabgo_test_support::paths;
use crabgo_test_support::{basic_manifest, git, project};
use crabgo_util::paths as crabgopaths;
use std::fs;
use std::path::{Path, PathBuf};

#[crabgo_test]
fn deleting_database_files() {
    let project = project();
    let git_project = git::new("bar", |project| {
        project
            .file("Crabgo.toml", &basic_manifest("bar", "0.5.0"))
            .file("src/lib.rs", "")
    });

    let project = project
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.5.0"
                    authors = []

                    [dependencies]
                    bar = {{ git = '{}' }}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    project.crabgo("check").run();

    let mut files = Vec::new();
    find_files(&paths::home().join(".crabgo/git/db"), &mut files);
    assert!(!files.is_empty());

    let log = "crabgo::sources::git=trace";
    for file in files {
        if !file.exists() {
            continue;
        }
        println!("deleting {}", file.display());
        crabgopaths::remove_file(&file).unwrap();
        project.crabgo("check -v").env("CRABGO_LOG", log).run();

        if !file.exists() {
            continue;
        }
        println!("truncating {}", file.display());
        make_writable(&file);
        fs::OpenOptions::new()
            .write(true)
            .open(&file)
            .unwrap()
            .set_len(2)
            .unwrap();
        project.crabgo("check -v").env("CRABGO_LOG", log).run();
    }
}

#[crabgo_test]
fn deleting_checkout_files() {
    let project = project();
    let git_project = git::new("bar", |project| {
        project
            .file("Crabgo.toml", &basic_manifest("bar", "0.5.0"))
            .file("src/lib.rs", "")
    });

    let project = project
        .file(
            "Crabgo.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.5.0"
                    authors = []

                    [dependencies]
                    bar = {{ git = '{}' }}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    project.crabgo("check").run();

    let dir = paths::home()
        .join(".crabgo/git/checkouts")
        // get the first entry in the checkouts dir for the package's location
        .read_dir()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        // get the first child of that checkout dir for our checkout
        .read_dir()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        // and throw on .git to corrupt things
        .join(".git");
    let mut files = Vec::new();
    find_files(&dir, &mut files);
    assert!(!files.is_empty());

    let log = "crabgo::sources::git=trace";
    for file in files {
        if !file.exists() {
            continue;
        }
        println!("deleting {}", file.display());
        crabgopaths::remove_file(&file).unwrap();
        project.crabgo("check -v").env("CRABGO_LOG", log).run();

        if !file.exists() {
            continue;
        }
        println!("truncating {}", file.display());
        make_writable(&file);
        fs::OpenOptions::new()
            .write(true)
            .open(&file)
            .unwrap()
            .set_len(2)
            .unwrap();
        project.crabgo("check -v").env("CRABGO_LOG", log).run();
    }
}

fn make_writable(path: &Path) {
    let mut p = path.metadata().unwrap().permissions();
    p.set_readonly(false);
    fs::set_permissions(path, p).unwrap();
}

fn find_files(path: &Path, dst: &mut Vec<PathBuf>) {
    for e in path.read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            find_files(&path, dst);
        } else {
            dst.push(path);
        }
    }
}
