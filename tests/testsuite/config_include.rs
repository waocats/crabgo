//! Tests for `include` config field.

use super::config::{assert_error, write_config, write_config_at, ConfigBuilder};
use crabgo_test_support::{no_such_file_err_msg, project};

#[crabgo_test]
fn gated() {
    // Requires -Z flag.
    write_config("include='other'");
    write_config_at(
        ".crabgo/other",
        "
        othervalue = 1
        ",
    );
    let config = ConfigBuilder::new().build();
    assert_eq!(config.get::<Option<i32>>("othervalue").unwrap(), None);
    let config = ConfigBuilder::new().unstable_flag("config-include").build();
    assert_eq!(config.get::<i32>("othervalue").unwrap(), 1);
}

#[crabgo_test]
fn simple() {
    // Simple test.
    write_config_at(
        ".crabgo/config",
        "
        include = 'other'
        key1 = 1
        key2 = 2
        ",
    );
    write_config_at(
        ".crabgo/other",
        "
        key2 = 3
        key3 = 4
        ",
    );
    let config = ConfigBuilder::new().unstable_flag("config-include").build();
    assert_eq!(config.get::<i32>("key1").unwrap(), 1);
    assert_eq!(config.get::<i32>("key2").unwrap(), 2);
    assert_eq!(config.get::<i32>("key3").unwrap(), 4);
}

#[crabgo_test]
fn works_with_cli() {
    write_config_at(
        ".crabgo/config.toml",
        "
        include = 'other.toml'
        [build]
        rustflags = ['-W', 'unused']
        ",
    );
    write_config_at(
        ".crabgo/other.toml",
        "
        [build]
        rustflags = ['-W', 'unsafe-code']
        ",
    );
    let p = project().file("src/lib.rs", "").build();
    p.crabgo("check -v")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 [..]
[RUNNING] `rustc [..]-W unused`
[FINISHED] [..]
",
        )
        .run();
    p.crabgo("check -v -Z config-include")
        .masquerade_as_nightly_crabgo(&["config-include"])
        .with_stderr(
            "\
[DIRTY] foo v0.0.1 ([..]): the rustflags changed
[CHECKING] foo v0.0.1 [..]
[RUNNING] `rustc [..]-W unsafe-code -W unused`
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn left_to_right() {
    // How it merges multiple includes.
    write_config_at(
        ".crabgo/config",
        "
        include = ['one', 'two']
        primary = 1
        ",
    );
    write_config_at(
        ".crabgo/one",
        "
        one = 1
        primary = 2
        ",
    );
    write_config_at(
        ".crabgo/two",
        "
        two = 2
        primary = 3
        ",
    );
    let config = ConfigBuilder::new().unstable_flag("config-include").build();
    assert_eq!(config.get::<i32>("primary").unwrap(), 1);
    assert_eq!(config.get::<i32>("one").unwrap(), 1);
    assert_eq!(config.get::<i32>("two").unwrap(), 2);
}

#[crabgo_test]
fn missing_file() {
    // Error when there's a missing file.
    write_config("include='missing'");
    let config = ConfigBuilder::new()
        .unstable_flag("config-include")
        .build_err();
    assert_error(
        config.unwrap_err(),
        &format!(
            "\
could not load Crabgo configuration

Caused by:
  failed to load config include `missing` from `[..]/.crabgo/config`

Caused by:
  failed to read configuration file `[..]/.crabgo/missing`

Caused by:
  {}",
            no_such_file_err_msg()
        ),
    );
}

#[crabgo_test]
fn cycle() {
    // Detects a cycle.
    write_config_at(".crabgo/config", "include='one'");
    write_config_at(".crabgo/one", "include='two'");
    write_config_at(".crabgo/two", "include='config'");
    let config = ConfigBuilder::new()
        .unstable_flag("config-include")
        .build_err();
    assert_error(
        config.unwrap_err(),
        "\
could not load Crabgo configuration

Caused by:
  failed to load config include `one` from `[..]/.crabgo/config`

Caused by:
  failed to load config include `two` from `[..]/.crabgo/one`

Caused by:
  failed to load config include `config` from `[..]/.crabgo/two`

Caused by:
  config `include` cycle detected with path `[..]/.crabgo/config`",
    );
}

#[crabgo_test]
fn cli_include() {
    // Using --config with include.
    // CLI takes priority over files.
    write_config_at(
        ".crabgo/config",
        "
        foo = 1
        bar = 2
        ",
    );
    write_config_at(".crabgo/config-foo", "foo = 2");
    let config = ConfigBuilder::new()
        .unstable_flag("config-include")
        .config_arg("include='.crabgo/config-foo'")
        .build();
    assert_eq!(config.get::<i32>("foo").unwrap(), 2);
    assert_eq!(config.get::<i32>("bar").unwrap(), 2);
}

#[crabgo_test]
fn bad_format() {
    // Not a valid format.
    write_config("include = 1");
    let config = ConfigBuilder::new()
        .unstable_flag("config-include")
        .build_err();
    assert_error(
        config.unwrap_err(),
        "\
could not load Crabgo configuration

Caused by:
  `include` expected a string or list, but found integer in `[..]/.crabgo/config`",
    );
}

#[crabgo_test]
fn cli_include_failed() {
    // Error message when CLI include fails to load.
    let config = ConfigBuilder::new()
        .unstable_flag("config-include")
        .config_arg("include='foobar'")
        .build_err();
    assert_error(
        config.unwrap_err(),
        &format!(
            "\
failed to load --config include

Caused by:
  failed to load config include `foobar` from `--config cli option`

Caused by:
  failed to read configuration file `[..]/foobar`

Caused by:
  {}",
            no_such_file_err_msg()
        ),
    );
}

#[crabgo_test]
fn cli_merge_failed() {
    // Error message when CLI include merge fails.
    write_config("foo = ['a']");
    write_config_at(
        ".crabgo/other",
        "
        foo = 'b'
        ",
    );
    let config = ConfigBuilder::new()
        .unstable_flag("config-include")
        .config_arg("include='.crabgo/other'")
        .build_err();
    // Maybe this error message should mention it was from an include file?
    assert_error(
        config.unwrap_err(),
        "\
failed to merge --config key `foo` into `[..]/.crabgo/config`

Caused by:
  failed to merge config value from `[..]/.crabgo/other` into `[..]/.crabgo/config`: \
  expected array, but found string",
    );
}

#[crabgo_test]
fn cli_include_take_priority_over_env() {
    write_config_at(".crabgo/include.toml", "k='include'");

    // k=env
    let config = ConfigBuilder::new().env("CRABGO_K", "env").build();
    assert_eq!(config.get::<String>("k").unwrap(), "env");

    // k=env
    // --config 'include=".crabgo/include.toml"'
    let config = ConfigBuilder::new()
        .env("CRABGO_K", "env")
        .unstable_flag("config-include")
        .config_arg("include='.crabgo/include.toml'")
        .build();
    assert_eq!(config.get::<String>("k").unwrap(), "include");

    // k=env
    // --config '.crabgo/foo.toml'
    write_config_at(".crabgo/foo.toml", "include='include.toml'");
    let config = ConfigBuilder::new()
        .env("CRABGO_K", "env")
        .unstable_flag("config-include")
        .config_arg(".crabgo/foo.toml")
        .build();
    assert_eq!(config.get::<String>("k").unwrap(), "include");
}
