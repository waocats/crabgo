//! -Zadvanced-env tests

use crabgo_test_support::{paths, project, registry::Package};

#[crabgo_test]
fn source_config_env() {
    // Try to define [source] with environment variables.
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            somedep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("somedep", "1.0.0")
        .local(true)
        .file("src/lib.rs", "")
        .publish();

    let path = paths::root().join("registry");

    p.crabgo("check -Zadvanced-env")
        .masquerade_as_nightly_crabgo(&["advanced-env"])
        .env("CRABGO_SOURCE_crates-io_REPLACE_WITH", "my-local-source")
        .env("CRABGO_SOURCE_my-local-source_LOCAL_REGISTRY", path)
        .run();
}
