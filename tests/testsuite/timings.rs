//! Tests for --timings.

use crabgo_test_support::project;
use crabgo_test_support::registry::Package;

#[crabgo_test]
fn timings_works() {
    Package::new("dep", "0.1.0").publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            dep = "0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .file("tests/t1.rs", "")
        .file("examples/ex1.rs", "fn main() {}")
        .build();

    p.crabgo("build --all-targets --timings")
        .with_stderr_unordered(
            "\
[UPDATING] [..]
[DOWNLOADING] crates ...
[DOWNLOADED] dep v0.1.0 [..]
[COMPILING] dep v0.1.0
[COMPILING] foo v0.1.0 [..]
[FINISHED] [..]
      Timing report saved to [..]/foo/target/crabgo-timings/crabgo-timing-[..].html
",
        )
        .run();

    p.crabgo("clean").run();

    p.crabgo("test --timings").run();

    p.crabgo("clean").run();

    p.crabgo("check --timings").run();

    p.crabgo("clean").run();

    p.crabgo("doc --timings").run();
}
