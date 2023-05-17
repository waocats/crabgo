//! Tests for workspace member discovery.

use crabgo::core::{Shell, Workspace};
use crabgo::util::config::Config;

use crabgo_test_support::install::cargo_home;
use crabgo_test_support::project;
use crabgo_test_support::registry;

/// Tests exclusion of non-directory files from workspace member discovery using glob `*`.
#[crabgo_test]
fn bad_file_member_exclusion() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [workspace]
                members = [ "crates/*" ]
            "#,
        )
        .file("crates/.DS_Store", "PLACEHOLDER")
        .file(
            "crates/bar/Crabgo.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []
            "#,
        )
        .file("crates/bar/src/main.rs", "fn main() {}")
        .build();

    // Prevent this test from accessing the network by setting up .crabgo/config.
    registry::init();
    let config = Config::new(
        Shell::from_write(Box::new(Vec::new())),
        cargo_home(),
        cargo_home(),
    );
    let ws = Workspace::new(&p.root().join("Crabgo.toml"), &config).unwrap();
    assert_eq!(ws.members().count(), 1);
    assert_eq!(ws.members().next().unwrap().name(), "bar");
}
