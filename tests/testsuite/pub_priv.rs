//! Tests for public/private dependencies.

use crabgo_test_support::project;
use crabgo_test_support::registry::Package;

#[crabgo_test(nightly, reason = "exported_private_dependencies lint is unstable")]
fn exported_priv_warning() {
    Package::new("priv_dep", "0.1.0")
        .file("src/lib.rs", "pub struct FromPriv;")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["public-dependency"]

                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                priv_dep = "0.1.0"
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate priv_dep;
            pub fn use_priv(_: priv_dep::FromPriv) {}
        ",
        )
        .build();

    p.crabgo("check --message-format=short")
        .masquerade_as_nightly_crabgo(&["public-dependency"])
        .with_stderr_contains(
            "\
src/lib.rs:3:13: warning: type `[..]FromPriv` from private dependency 'priv_dep' in public interface
",
        )
        .run()
}

#[crabgo_test(nightly, reason = "exported_private_dependencies lint is unstable")]
fn exported_pub_dep() {
    Package::new("pub_dep", "0.1.0")
        .file("src/lib.rs", "pub struct FromPub;")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["public-dependency"]

                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                pub_dep = {version = "0.1.0", public = true}
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate pub_dep;
            pub fn use_pub(_: pub_dep::FromPub) {}
        ",
        )
        .build();

    p.crabgo("check --message-format=short")
        .masquerade_as_nightly_crabgo(&["public-dependency"])
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] pub_dep v0.1.0 ([..])
[CHECKING] pub_dep v0.1.0
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run()
}

#[crabgo_test]
pub fn requires_nightly_crabgo() {
    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["public-dependency"]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check --message-format=short")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  the crabgo feature `public-dependency` requires a nightly version of Crabgo, but this is the `stable` channel
  See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
  See https://doc.rust-lang.org/[..]crabgo/reference/unstable.html#public-dependency for more information about using this feature.
"
        )
        .run()
}

#[crabgo_test]
fn requires_feature() {
    Package::new("pub_dep", "0.1.0")
        .file("src/lib.rs", "")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                pub_dep = { version = "0.1.0", public = true }
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check --message-format=short")
        .masquerade_as_nightly_crabgo(&["public-dependency"])
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  feature `public-dependency` is required

  The package requires the Crabgo feature called `public-dependency`, \
  but that feature is not stabilized in this version of Crabgo (1.[..]).
  Consider adding `crabgo-features = [\"public-dependency\"]` to the top of Crabgo.toml \
  (above the [package] table) to tell Crabgo you are opting in to use this unstable feature.
  See https://doc.rust-lang.org/nightly/crabgo/reference/unstable.html#public-dependency \
  for more information about the status of this feature.
",
        )
        .run()
}

#[crabgo_test]
fn pub_dev_dependency() {
    Package::new("pub_dep", "0.1.0")
        .file("src/lib.rs", "pub struct FromPub;")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                crabgo-features = ["public-dependency"]

                [package]
                name = "foo"
                version = "0.0.1"

                [dev-dependencies]
                pub_dep = {version = "0.1.0", public = true}
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate pub_dep;
            pub fn use_pub(_: pub_dep::FromPub) {}
        ",
        )
        .build();

    p.crabgo("check --message-format=short")
        .masquerade_as_nightly_crabgo(&["public-dependency"])
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  'public' specifier can only be used on regular dependencies, not Development dependencies
",
        )
        .run()
}
