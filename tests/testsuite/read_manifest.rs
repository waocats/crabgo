//! Tests for the `crabgo read-manifest` command.

use crabgo_test_support::{basic_bin_manifest, main_file, project};

fn manifest_output(readme_value: &str) -> String {
    format!(
        r#"
{{
    "authors": [
        "wycats@example.com"
    ],
    "categories": [],
    "default_run": null,
    "name":"foo",
    "readme": {},
    "homepage": null,
    "documentation": null,
    "repository": null,
    "rust_version": null,
    "version":"0.5.0",
    "id":"foo[..]0.5.0[..](path+file://[..]/foo)",
    "keywords": [],
    "license": null,
    "license_file": null,
    "links": null,
    "description": null,
    "edition": "2015",
    "source":null,
    "dependencies":[],
    "targets":[{{
        "kind":["bin"],
        "crate_types":["bin"],
        "doc": true,
        "doctest": false,
        "test": true,
        "edition": "2015",
        "name":"foo",
        "src_path":"[..]/foo/src/foo.rs"
    }}],
    "features":{{}},
    "manifest_path":"[..]Crabgo.toml",
    "metadata": null,
    "publish": null
}}"#,
        readme_value
    )
}

fn manifest_output_no_readme() -> String {
    manifest_output("null")
}

pub fn basic_bin_manifest_with_readme(name: &str, readme_filename: &str) -> String {
    format!(
        r#"
            [package]

            name = "{}"
            version = "0.5.0"
            authors = ["wycats@example.com"]
            readme = {}

            [[bin]]

            name = "{}"
        "#,
        name, readme_filename, name
    )
}

#[crabgo_test]
fn crabgo_read_manifest_path_to_crabgo_toml_relative() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest --manifest-path foo/Crabgo.toml")
        .cwd(p.root().parent().unwrap())
        .with_json(&manifest_output_no_readme())
        .run();
}

#[crabgo_test]
fn crabgo_read_manifest_path_to_crabgo_toml_absolute() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest --manifest-path")
        .arg(p.root().join("Crabgo.toml"))
        .cwd(p.root().parent().unwrap())
        .with_json(&manifest_output_no_readme())
        .run();
}

#[crabgo_test]
fn crabgo_read_manifest_path_to_crabgo_toml_parent_relative() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest --manifest-path foo")
        .cwd(p.root().parent().unwrap())
        .with_status(101)
        .with_stderr(
            "[ERROR] the manifest-path must be \
             a path to a Crabgo.toml file",
        )
        .run();
}

#[crabgo_test]
fn crabgo_read_manifest_path_to_crabgo_toml_parent_absolute() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest --manifest-path")
        .arg(p.root())
        .cwd(p.root().parent().unwrap())
        .with_status(101)
        .with_stderr(
            "[ERROR] the manifest-path must be \
             a path to a Crabgo.toml file",
        )
        .run();
}

#[crabgo_test]
fn crabgo_read_manifest_cwd() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest")
        .with_json(&manifest_output_no_readme())
        .run();
}

#[crabgo_test]
fn crabgo_read_manifest_with_specified_readme() {
    let p = project()
        .file(
            "Crabgo.toml",
            &basic_bin_manifest_with_readme("foo", r#""SomeReadme.txt""#),
        )
        .file("SomeReadme.txt", "Sample Project")
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest")
        .with_json(&manifest_output(&format!(r#""{}""#, "SomeReadme.txt")))
        .run();
}

#[crabgo_test]
fn crabgo_read_manifest_default_readme() {
    let readme_filenames = ["README.md", "README.txt", "README"];

    for readme in readme_filenames.iter() {
        let p = project()
            .file("Crabgo.toml", &basic_bin_manifest("foo"))
            .file(readme, "Sample project")
            .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
            .build();

        p.crabgo("read-manifest")
            .with_json(&manifest_output(&format!(r#""{}""#, readme)))
            .run();
    }
}

#[crabgo_test]
fn crabgo_read_manifest_suppress_default_readme() {
    let p = project()
        .file(
            "Crabgo.toml",
            &basic_bin_manifest_with_readme("foo", "false"),
        )
        .file("README.txt", "Sample project")
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest")
        .with_json(&manifest_output_no_readme())
        .run();
}

// If a file named README.md exists, and `readme = true`, the value `README.md` should be defaulted in.
#[crabgo_test]
fn crabgo_read_manifest_defaults_readme_if_true() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest_with_readme("foo", "true"))
        .file("README.md", "Sample project")
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.crabgo("read-manifest")
        .with_json(&manifest_output(r#""README.md""#))
        .run();
}
