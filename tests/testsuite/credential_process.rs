//! Tests for credential-process.

use crabgo_test_support::registry::TestRegistry;
use crabgo_test_support::{basic_manifest, crabgo_process, paths, project, registry, Project};
use std::fs::{self, read_to_string};

fn toml_bin(proj: &Project, name: &str) -> String {
    proj.bin(name).display().to_string().replace('\\', "\\\\")
}

#[crabgo_test]
fn gated() {
    let _alternative = registry::RegistryBuilder::new()
        .alternative()
        .no_configure_token()
        .build();

    let cratesio = registry::RegistryBuilder::new()
        .no_configure_token()
        .build();

    let p = project()
        .file(
            ".crabgo/config",
            r#"
                [registry]
                credential-process = "false"
            "#,
        )
        .file("Crabgo.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/lib.rs", "")
        .build();

    p.crabgo("publish --no-verify")
        .replace_crates_io(cratesio.index_url())
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..]
[ERROR] no token found, please run `crabgo login`
or use environment variable CRABGO_REGISTRY_TOKEN
",
        )
        .run();

    p.change_file(
        ".crabgo/config",
        r#"
            [registry.alternative]
            credential-process = "false"
        "#,
    );

    p.crabgo("publish --no-verify --registry alternative")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..]
[ERROR] no token found for `alternative`, please run `crabgo login --registry alternative`
or use environment variable CRABGO_REGISTRIES_ALTERNATIVE_TOKEN
",
        )
        .run();
}

#[crabgo_test]
fn warn_both_token_and_process() {
    // Specifying both credential-process and a token in config should issue a warning.
    let _server = registry::RegistryBuilder::new()
        .http_api()
        .http_index()
        .alternative()
        .no_configure_token()
        .build();
    let p = project()
        .file(
            ".crabgo/config",
            r#"
                [registries.alternative]
                token = "alternative-sekrit"
                credential-process = "false"
            "#,
        )
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                description = "foo"
                authors = []
                license = "MIT"
                homepage = "https://example.com/"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("publish --no-verify --registry alternative -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..]
[ERROR] both `token` and `credential-process` were specified in the config for registry `alternative`.
Only one of these values may be set, remove one or the other to proceed.
",
        )
        .run();

    // Try with global credential-process, and registry-specific `token`.
    // This should silently use the config token, and not run the "false" exe.
    p.change_file(
        ".crabgo/config",
        r#"
            [registry]
            credential-process = "false"

            [registries.alternative]
            token = "alternative-sekrit"
        "#,
    );
    p.crabgo("publish --no-verify --registry alternative -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_stderr(
            "\
[UPDATING] [..]
[PACKAGING] foo v0.1.0 [..]
[PACKAGED] [..]
[UPLOADING] foo v0.1.0 [..]
[UPLOADED] foo v0.1.0 [..]
note: Waiting [..]
You may press ctrl-c [..]
[PUBLISHED] foo v0.1.0 [..]
",
        )
        .run();
}

/// Setup for a test that will issue a command that needs to fetch a token.
///
/// This does the following:
///
/// * Spawn a thread that will act as an API server.
/// * Create a simple credential-process that will generate a fake token.
/// * Create a simple `foo` project to run the test against.
/// * Configure the credential-process config.
///
/// Returns the simple `foo` project to test against and the API server handle.
fn get_token_test() -> (Project, TestRegistry) {
    // API server that checks that the token is included correctly.
    let server = registry::RegistryBuilder::new()
        .no_configure_token()
        .token(crabgo_test_support::registry::Token::Plaintext(
            "sekrit".to_string(),
        ))
        .alternative()
        .http_api()
        .build();
    // The credential process to use.
    let cred_proj = project()
        .at("cred_proj")
        .file("Crabgo.toml", &basic_manifest("test-cred", "1.0.0"))
        .file(
            "src/main.rs",
            r#"
                use std::fs::File;
                use std::io::Write;
                fn main() {
                    let mut f = File::options()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open("runs.log")
                        .unwrap();
                    write!(f, "+");
                    println!("sekrit");
                } "#,
        )
        .build();
    cred_proj.crabgo("build").run();

    let p = project()
        .file(
            ".crabgo/config",
            &format!(
                r#"
                    [registries.alternative]
                    index = "{}"
                    credential-process = ["{}"]
                "#,
                server.index_url(),
                toml_bin(&cred_proj, "test-cred")
            ),
        )
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                description = "foo"
                authors = []
                license = "MIT"
                homepage = "https://example.com/"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    (p, server)
}

#[crabgo_test]
fn publish() {
    // Checks that credential-process is used for `crabgo publish`.
    let (p, _t) = get_token_test();

    p.crabgo("publish --no-verify --registry alternative -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_stderr(
            "\
[UPDATING] [..]
[PACKAGING] foo v0.1.0 [..]
[PACKAGED] [..]
[UPLOADING] foo v0.1.0 [..]
[UPLOADED] foo v0.1.0 [..]
note: Waiting [..]
You may press ctrl-c [..]
[PUBLISHED] foo v0.1.0 [..]
",
        )
        .run();

    let calls = read_to_string(p.root().join("runs.log")).unwrap().len();
    assert_eq!(calls, 1);
}

#[crabgo_test]
fn basic_unsupported() {
    // Non-action commands don't support login/logout.
    let registry = registry::RegistryBuilder::new()
        .no_configure_token()
        .build();
    crabgo_util::paths::append(
        &paths::home().join(".crabgo/config"),
        br#"
            [registry]
            credential-process = "false"
        "#,
    )
    .unwrap();

    crabgo_process("login -Z credential-process abcdefg")
        .replace_crates_io(registry.index_url())
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] crates.io index
[ERROR] credential process `false` cannot be used to log in, \
the credential-process configuration value must pass the \
`{action}` argument in the config to support this command
",
        )
        .run();

    crabgo_process("logout -Z credential-process")
        .replace_crates_io(registry.index_url())
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_status(101)
        .with_stderr(
            "\
[ERROR] credential process `false` cannot be used to log out, \
the credential-process configuration value must pass the \
`{action}` argument in the config to support this command
",
        )
        .run();
}

#[crabgo_test]
fn login() {
    let server = registry::RegistryBuilder::new()
        .no_configure_token()
        .build();
    // The credential process to use.
    let cred_proj = project()
        .at("cred_proj")
        .file("Crabgo.toml", &basic_manifest("test-cred", "1.0.0"))
        .file(
            "src/main.rs",
                r#"
                use std::io::Read;

                fn main() {{
                    assert_eq!(std::env::var("CRABGO_REGISTRY_NAME_OPT").unwrap(), "crates-io");
                    assert_eq!(std::env::var("CRABGO_REGISTRY_INDEX_URL").unwrap(), "https://github.com/rust-lang/crates.io-index");
                    assert_eq!(std::env::args().skip(1).next().unwrap(), "store");
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();
                    assert_eq!(buffer, "abcdefg\n");
                    std::fs::write("token-store", buffer).unwrap();
                }}
            "#,
        )
        .build();
    cred_proj.crabgo("build").run();

    crabgo_util::paths::append(
        &paths::home().join(".crabgo/config"),
        format!(
            r#"
                [registry]
                credential-process = ["{}", "{{action}}"]
            "#,
            toml_bin(&cred_proj, "test-cred")
        )
        .as_bytes(),
    )
    .unwrap();

    crabgo_process("login -Z credential-process abcdefg")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .replace_crates_io(server.index_url())
        .with_stderr(
            "\
[UPDATING] [..]
[LOGIN] token for `crates.io` saved
",
        )
        .run();
    assert_eq!(
        fs::read_to_string(paths::root().join("token-store")).unwrap(),
        "abcdefg\n"
    );
}

#[crabgo_test]
fn logout() {
    let server = registry::RegistryBuilder::new()
        .no_configure_token()
        .build();
    // The credential process to use.
    let cred_proj = project()
        .at("cred_proj")
        .file("Crabgo.toml", &basic_manifest("test-cred", "1.0.0"))
        .file(
            "src/main.rs",
                r#"
                use std::io::Read;

                fn main() {{
                    assert_eq!(std::env::var("CRABGO_REGISTRY_NAME_OPT").unwrap(), "crates-io");
                    assert_eq!(std::env::var("CRABGO_REGISTRY_INDEX_URL").unwrap(), "https://github.com/rust-lang/crates.io-index");
                    assert_eq!(std::env::args().skip(1).next().unwrap(), "erase");
                    std::fs::write("token-store", "").unwrap();
                    eprintln!("token for `crates-io` has been erased!")
                }}
            "#,
        )
        .build();
    cred_proj.crabgo("build").run();

    crabgo_util::paths::append(
        &paths::home().join(".crabgo/config"),
        format!(
            r#"
                [registry]
                credential-process = ["{}", "{{action}}"]
            "#,
            toml_bin(&cred_proj, "test-cred")
        )
        .as_bytes(),
    )
    .unwrap();

    crabgo_process("logout -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .replace_crates_io(server.index_url())
        .with_stderr(
            "\
token for `crates-io` has been erased!
[LOGOUT] token for `crates-io` has been removed from local storage
[NOTE] This does not revoke the token on the registry server.
    If you need to revoke the token, visit <https://crates.io/me> \
    and follow the instructions there.
",
        )
        .run();
    assert_eq!(
        fs::read_to_string(paths::root().join("token-store")).unwrap(),
        ""
    );
}

#[crabgo_test]
fn yank() {
    let (p, _t) = get_token_test();

    p.crabgo("yank --version 0.1.0 --registry alternative -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_stderr(
            "\
[UPDATING] [..]
[YANK] foo@0.1.0
",
        )
        .run();
}

#[crabgo_test]
fn owner() {
    let (p, _t) = get_token_test();

    p.crabgo("owner --add username --registry alternative -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_stderr(
            "\
[UPDATING] [..]
[OWNER] completed!
",
        )
        .run();
}

#[crabgo_test]
fn libexec_path() {
    // crabgo: prefixed names use the sysroot
    let server = registry::RegistryBuilder::new()
        .no_configure_token()
        .build();
    crabgo_util::paths::append(
        &paths::home().join(".crabgo/config"),
        br#"
            [registry]
            credential-process = "crabgo:doesnotexist"
        "#,
    )
    .unwrap();

    crabgo_process("login -Z credential-process abcdefg")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .replace_crates_io(server.index_url())
        .with_status(101)
        .with_stderr(
            // FIXME: Update "Caused by" error message once rust/pull/87704 is merged.
            // On Windows, changing to a custom executable resolver has changed the
            // error messages.
            &format!("\
[UPDATING] [..]
[ERROR] failed to execute `[..]libexec/crabgo-credential-doesnotexist[EXE]` to store authentication token for registry `crates-io`

Caused by:
  [..]
"),
        )
        .run();
}

#[crabgo_test]
fn invalid_token_output() {
    // Error when credential process does not output the expected format for a token.
    let _server = registry::RegistryBuilder::new()
        .alternative()
        .no_configure_token()
        .build();
    let cred_proj = project()
        .at("cred_proj")
        .file("Crabgo.toml", &basic_manifest("test-cred", "1.0.0"))
        .file("src/main.rs", r#"fn main() { print!("a\nb\n"); } "#)
        .build();
    cred_proj.crabgo("build").run();

    crabgo_util::paths::append(
        &paths::home().join(".crabgo/config"),
        format!(
            r#"
                [registry]
                credential-process = ["{}"]
            "#,
            toml_bin(&cred_proj, "test-cred")
        )
        .as_bytes(),
    )
    .unwrap();

    let p = project()
        .file("Crabgo.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/lib.rs", "")
        .build();

    p.crabgo("publish --no-verify --registry alternative -Z credential-process")
        .masquerade_as_nightly_crabgo(&["credential-process"])
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..]
[ERROR] credential process `[..]test-cred[EXE]` returned more than one line of output; expected a single token
",
        )
        .run();
}
