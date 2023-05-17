//! Tests for the `crabgo logout` command.

use super::login::check_token;
use crabgo_test_support::paths::{self, CrabgoPathExt};
use crabgo_test_support::registry::TestRegistry;
use crabgo_test_support::{crabgo_process, registry};

fn simple_logout_test(registry: &TestRegistry, reg: Option<&str>, flag: &str, note: &str) {
    let msg = reg.unwrap_or("crates-io");
    check_token(Some(registry.token()), reg);
    let mut crabgo = crabgo_process(&format!("logout {}", flag));
    if reg.is_none() {
        crabgo.replace_crates_io(registry.index_url());
    }
    crabgo
        .with_stderr(&format!(
            "\
[LOGOUT] token for `{msg}` has been removed from local storage
[NOTE] This does not revoke the token on the registry server.\n    \
If you need to revoke the token, visit {note} and follow the instructions there.
"
        ))
        .run();
    check_token(None, reg);

    let mut crabgo = crabgo_process(&format!("logout {}", flag));
    if reg.is_none() {
        crabgo.replace_crates_io(registry.index_url());
    }
    crabgo
        .with_stderr(&format!("[LOGOUT] not currently logged in to `{msg}`"))
        .run();
    check_token(None, reg);
}

#[crabgo_test]
fn default_registry_unconfigured() {
    let registry = registry::init();
    simple_logout_test(&registry, None, "", "<https://crates.io/me>");
}

#[crabgo_test]
fn other_registry() {
    let registry = registry::alt_init();
    simple_logout_test(
        &registry,
        Some("alternative"),
        "--registry alternative",
        "the `alternative` website",
    );
    // It should not touch crates.io.
    check_token(Some("sekrit"), None);
}

#[crabgo_test]
fn default_registry_configured() {
    // When registry.default is set, logout should use that one when
    // --registry is not used.
    let cargo_home = paths::home().join(".crabgo");
    cargo_home.mkdir_p();
    crabgo_util::paths::write(
        &cargo_home.join("config.toml"),
        r#"
            [registry]
            default = "dummy-registry"

            [registries.dummy-registry]
            index = "https://127.0.0.1/index"
        "#,
    )
    .unwrap();
    crabgo_util::paths::write(
        &cargo_home.join("credentials.toml"),
        r#"
        [registry]
        token = "crates-io-token"

        [registries.dummy-registry]
        token = "dummy-token"
        "#,
    )
    .unwrap();
    check_token(Some("dummy-token"), Some("dummy-registry"));
    check_token(Some("crates-io-token"), None);

    crabgo_process("logout -Zunstable-options")
        .masquerade_as_nightly_crabgo(&["crabgo-logout"])
        .with_stderr(
            "\
[LOGOUT] token for `dummy-registry` has been removed from local storage
[NOTE] This does not revoke the token on the registry server.
    If you need to revoke the token, visit the `dummy-registry` website \
    and follow the instructions there.
",
        )
        .run();
    check_token(None, Some("dummy-registry"));
    check_token(Some("crates-io-token"), None);

    crabgo_process("logout -Zunstable-options")
        .masquerade_as_nightly_crabgo(&["crabgo-logout"])
        .with_stderr("[LOGOUT] not currently logged in to `dummy-registry`")
        .run();
}
