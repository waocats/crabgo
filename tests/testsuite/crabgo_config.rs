//! Tests for the `crabgo config` command.

use super::config::write_config_at;
use crabgo_test_support::paths;
use std::fs;
use std::path::PathBuf;

fn crabgo_process(s: &str) -> crabgo_test_support::Execs {
    let mut p = crabgo_test_support::crabgo_process(s);
    // Clear out some of the environment added by the default crabgo_process so
    // the tests don't need to deal with it.
    p.env_remove("CARGO_PROFILE_DEV_SPLIT_DEBUGINFO")
        .env_remove("CARGO_PROFILE_TEST_SPLIT_DEBUGINFO")
        .env_remove("CARGO_PROFILE_RELEASE_SPLIT_DEBUGINFO")
        .env_remove("CARGO_PROFILE_BENCH_SPLIT_DEBUGINFO")
        .env_remove("CARGO_INCREMENTAL");
    p
}

#[crabgo_test]
fn gated() {
    crabgo_process("config get")
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_status(101)
        .with_stderr("\
error: the `crabgo config` command is unstable, pass `-Z unstable-options` to enable it
See https://github.com/rust-lang/crabgo/issues/9301 for more information about the `crabgo config` command.
")
        .run();
}

fn common_setup() -> PathBuf {
    write_config_at(
        paths::home().join(".crabgo/config.toml"),
        "
        [alias]
        foo = \"abc --xyz\"
        [build]
        jobs = 99
        rustflags = [\"--flag-global\"]
        [profile.dev]
        opt-level = 3
        [profile.dev.package.foo]
        opt-level = 1
        [target.'cfg(target_os = \"linux\")']
        runner = \"runme\"

        # How unknown keys are handled.
        [extra-table]
        somekey = \"somevalue\"
        ",
    );
    let sub_folder = paths::root().join("foo/.crabgo");
    write_config_at(
        sub_folder.join("config.toml"),
        "
        [alias]
        sub-example = [\"sub\", \"example\"]
        [build]
        rustflags = [\"--flag-directory\"]
        ",
    );
    sub_folder
}

#[crabgo_test]
fn get_toml() {
    // Notes:
    // - The "extra-table" is shown without a warning. I'm not sure how that
    //   should be handled, since displaying warnings could cause problems
    //   with ingesting the output.
    // - Environment variables aren't loaded. :(
    let sub_folder = common_setup();
    crabgo_process("config get -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_ALIAS_BAR", "cat dog")
        .env("CARGO_BUILD_JOBS", "100")
        // The weird forward slash in the linux line is due to testsuite normalization.
        .with_stdout(
            "\
alias.foo = \"abc --xyz\"
alias.sub-example = [\"sub\", \"example\"]
build.jobs = 99
build.rustflags = [\"--flag-directory\", \"--flag-global\"]
extra-table.somekey = \"somevalue\"
profile.dev.opt-level = 3
profile.dev.package.foo.opt-level = 1
target.\"cfg(target_os = \\\"linux\\\")\".runner = \"runme\"
# The following environment variables may affect the loaded values.
# CARGO_ALIAS_BAR=[..]cat dog[..]
# CARGO_BUILD_JOBS=100
# CARGO_HOME=[ROOT]/home/.crabgo
",
        )
        .with_stderr("")
        .run();

    // Env keys work if they are specific.
    crabgo_process("config get build.jobs -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_BUILD_JOBS", "100")
        .with_stdout("build.jobs = 100")
        .with_stderr("")
        .run();

    // Array value.
    crabgo_process("config get build.rustflags -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_stdout("build.rustflags = [\"--flag-directory\", \"--flag-global\"]")
        .with_stderr("")
        .run();

    // Sub-table
    crabgo_process("config get profile -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_stdout(
            "\
profile.dev.opt-level = 3
profile.dev.package.foo.opt-level = 1
",
        )
        .with_stderr("")
        .run();

    // Specific profile entry.
    crabgo_process("config get profile.dev.opt-level -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_stdout("profile.dev.opt-level = 3")
        .with_stderr("")
        .run();

    // A key that isn't set.
    crabgo_process("config get build.rustc -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_status(101)
        .with_stdout("")
        .with_stderr("error: config value `build.rustc` is not set")
        .run();

    // A key that is not part of Crabgo's config schema.
    crabgo_process("config get not.set -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_status(101)
        .with_stdout("")
        .with_stderr("error: config value `not.set` is not set")
        .run();
}

#[crabgo_test]
fn get_json() {
    // Notes:
    // - This does not show env vars at all. :(
    let all_json = r#"
            {
              "alias": {
                "foo": "abc --xyz",
                "sub-example": [
                  "sub",
                  "example"
                ]
              },
              "build": {
                "jobs": 99,
                "rustflags": [
                  "--flag-directory",
                  "--flag-global"
                ]
              },
              "extra-table": {
                "somekey": "somevalue"
              },
              "profile": {
                "dev": {
                  "opt-level": 3,
                  "package": {
                    "foo": {
                      "opt-level": 1
                    }
                  }
                }
              },
              "target": {
                "cfg(target_os = \"linux\")": {
                  "runner": "runme"
                }
              }
            }
            "#;
    let sub_folder = common_setup();
    crabgo_process("config get --format=json -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_ALIAS_BAR", "cat dog")
        .env("CARGO_BUILD_JOBS", "100")
        .with_json(all_json)
        .with_stderr(
            "\
note: The following environment variables may affect the loaded values.
CARGO_ALIAS_BAR=[..]cat dog[..]
CARGO_BUILD_JOBS=100
CARGO_HOME=[ROOT]/home/.crabgo
",
        )
        .run();

    // json-value is the same for the entire root table
    crabgo_process("config get --format=json-value -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_json(all_json)
        .with_stderr(
            "\
note: The following environment variables may affect the loaded values.
CARGO_HOME=[ROOT]/home/.crabgo
",
        )
        .run();

    crabgo_process("config get --format=json build.jobs -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_json(
            r#"
            {"build": {"jobs": 99}}
            "#,
        )
        .with_stderr("")
        .run();

    crabgo_process("config get --format=json-value build.jobs -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_stdout("99")
        .with_stderr("")
        .run();
}

#[crabgo_test]
fn show_origin_toml() {
    let sub_folder = common_setup();
    crabgo_process("config get --show-origin -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_stdout(
            "\
alias.foo = \"abc --xyz\" # [ROOT]/home/.crabgo/config.toml
alias.sub-example = [
    \"sub\", # [ROOT]/foo/.crabgo/config.toml
    \"example\", # [ROOT]/foo/.crabgo/config.toml
]
build.jobs = 99 # [ROOT]/home/.crabgo/config.toml
build.rustflags = [
    \"--flag-directory\", # [ROOT]/foo/.crabgo/config.toml
    \"--flag-global\", # [ROOT]/home/.crabgo/config.toml
]
extra-table.somekey = \"somevalue\" # [ROOT]/home/.crabgo/config.toml
profile.dev.opt-level = 3 # [ROOT]/home/.crabgo/config.toml
profile.dev.package.foo.opt-level = 1 # [ROOT]/home/.crabgo/config.toml
target.\"cfg(target_os = \\\"linux\\\")\".runner = \"runme\" # [ROOT]/home/.crabgo/config.toml
# The following environment variables may affect the loaded values.
# CARGO_HOME=[ROOT]/home/.crabgo
",
        )
        .with_stderr("")
        .run();

    crabgo_process("config get --show-origin build.rustflags -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_BUILD_RUSTFLAGS", "env1 env2")
        .with_stdout(
            "\
build.rustflags = [
    \"--flag-directory\", # [ROOT]/foo/.crabgo/config.toml
    \"--flag-global\", # [ROOT]/home/.crabgo/config.toml
    \"env1\", # environment variable `CARGO_BUILD_RUSTFLAGS`
    \"env2\", # environment variable `CARGO_BUILD_RUSTFLAGS`
]
",
        )
        .with_stderr("")
        .run();
}

#[crabgo_test]
fn show_origin_toml_cli() {
    let sub_folder = common_setup();
    crabgo_process("config get --show-origin build.jobs -Zunstable-options --config build.jobs=123")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_BUILD_JOBS", "1")
        .with_stdout("build.jobs = 123 # --config cli option")
        .with_stderr("")
        .run();

    crabgo_process("config get --show-origin build.rustflags -Zunstable-options --config")
        .arg("build.rustflags=[\"cli1\",\"cli2\"]")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_BUILD_RUSTFLAGS", "env1 env2")
        .with_stdout(
            "\
build.rustflags = [
    \"--flag-directory\", # [ROOT]/foo/.crabgo/config.toml
    \"--flag-global\", # [ROOT]/home/.crabgo/config.toml
    \"cli1\", # --config cli option
    \"cli2\", # --config cli option
    \"env1\", # environment variable `CARGO_BUILD_RUSTFLAGS`
    \"env2\", # environment variable `CARGO_BUILD_RUSTFLAGS`
]
",
        )
        .with_stderr("")
        .run();
}

#[crabgo_test]
fn show_origin_json() {
    let sub_folder = common_setup();
    crabgo_process("config get --show-origin --format=json -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_status(101)
        .with_stderr("error: the `json` format does not support --show-origin, try the `toml` format instead")
        .run();
}

#[crabgo_test]
fn unmerged_toml() {
    let sub_folder = common_setup();
    crabgo_process("config get --merged=no -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_ALIAS_BAR", "cat dog")
        .env("CARGO_BUILD_JOBS", "100")
        .with_stdout(
            "\
# Environment variables
# CARGO=[..]
# CARGO_ALIAS_BAR=[..]cat dog[..]
# CARGO_BUILD_JOBS=100
# CARGO_HOME=[ROOT]/home/.crabgo

# [ROOT]/foo/.crabgo/config.toml
alias.sub-example = [\"sub\", \"example\"]
build.rustflags = [\"--flag-directory\"]

# [ROOT]/home/.crabgo/config.toml
alias.foo = \"abc --xyz\"
build.jobs = 99
build.rustflags = [\"--flag-global\"]
extra-table.somekey = \"somevalue\"
profile.dev.opt-level = 3
profile.dev.package.foo.opt-level = 1
target.\"cfg(target_os = \\\"linux\\\")\".runner = \"runme\"

",
        )
        .with_stderr("")
        .run();

    crabgo_process("config get --merged=no build.rustflags -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_BUILD_RUSTFLAGS", "env1 env2")
        .with_stdout(
            "\
# Environment variables
# CARGO_BUILD_RUSTFLAGS=[..]env1 env2[..]

# [ROOT]/foo/.crabgo/config.toml
build.rustflags = [\"--flag-directory\"]

# [ROOT]/home/.crabgo/config.toml
build.rustflags = [\"--flag-global\"]

",
        )
        .with_stderr("")
        .run();

    crabgo_process("config get --merged=no does.not.exist -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_stderr("")
        .with_stderr("")
        .run();

    crabgo_process("config get --merged=no build.rustflags.extra -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_status(101)
        .with_stderr(
            "error: expected table for configuration key `build.rustflags`, \
             but found array in [ROOT]/foo/.crabgo/config.toml",
        )
        .run();
}

#[crabgo_test]
fn unmerged_toml_cli() {
    let sub_folder = common_setup();
    crabgo_process("config get --merged=no build.rustflags -Zunstable-options --config")
        .arg("build.rustflags=[\"cli1\",\"cli2\"]")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .env("CARGO_BUILD_RUSTFLAGS", "env1 env2")
        .with_stdout(
            "\
# --config cli option
build.rustflags = [\"cli1\", \"cli2\"]

# Environment variables
# CARGO_BUILD_RUSTFLAGS=[..]env1 env2[..]

# [ROOT]/foo/.crabgo/config.toml
build.rustflags = [\"--flag-directory\"]

# [ROOT]/home/.crabgo/config.toml
build.rustflags = [\"--flag-global\"]

",
        )
        .with_stderr("")
        .run();
}

#[crabgo_test]
fn unmerged_json() {
    let sub_folder = common_setup();
    crabgo_process("config get --merged=no --format=json -Zunstable-options")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config"])
        .with_status(101)
        .with_stderr(
            "error: the `json` format does not support --merged=no, try the `toml` format instead",
        )
        .run();
}

#[crabgo_test]
fn includes() {
    let sub_folder = common_setup();
    fs::write(
        sub_folder.join("config.toml"),
        "
        include = 'other.toml'
        [build]
        rustflags = [\"--flag-directory\"]
        ",
    )
    .unwrap();
    fs::write(
        sub_folder.join("other.toml"),
        "
        [build]
        rustflags = [\"--flag-other\"]
        ",
    )
    .unwrap();

    crabgo_process("config get build.rustflags -Zunstable-options -Zconfig-include")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config", "config-include"])
        .with_stdout(r#"build.rustflags = ["--flag-other", "--flag-directory", "--flag-global"]"#)
        .with_stderr("")
        .run();

    crabgo_process("config get build.rustflags --show-origin -Zunstable-options -Zconfig-include")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config", "config-include"])
        .with_stdout(
            "\
build.rustflags = [
    \"--flag-other\", # [ROOT]/foo/.crabgo/other.toml
    \"--flag-directory\", # [ROOT]/foo/.crabgo/config.toml
    \"--flag-global\", # [ROOT]/home/.crabgo/config.toml
]
",
        )
        .with_stderr("")
        .run();

    crabgo_process("config get --merged=no -Zunstable-options -Zconfig-include")
        .cwd(&sub_folder.parent().unwrap())
        .masquerade_as_nightly_crabgo(&["crabgo-config", "config-include"])
        .with_stdout(
            "\
# Environment variables
# CARGO=[..]
# CARGO_HOME=[ROOT]/home/.crabgo

# [ROOT]/foo/.crabgo/other.toml
build.rustflags = [\"--flag-other\"]

# [ROOT]/foo/.crabgo/config.toml
build.rustflags = [\"--flag-directory\"]
include = \"other.toml\"

# [ROOT]/home/.crabgo/config.toml
alias.foo = \"abc --xyz\"
build.jobs = 99
build.rustflags = [\"--flag-global\"]
extra-table.somekey = \"somevalue\"
profile.dev.opt-level = 3
profile.dev.package.foo.opt-level = 1
target.\"cfg(target_os = \\\"linux\\\")\".runner = \"runme\"

",
        )
        .with_stderr("")
        .run();
}
