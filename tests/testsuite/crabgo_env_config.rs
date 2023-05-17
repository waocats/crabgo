//! Tests for `[env]` config.

use crabgo_test_support::basic_manifest;
use crabgo_test_support::{basic_bin_manifest, project};

#[crabgo_test]
fn env_basic() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        fn main() {
            println!( "compile-time:{}", env!("ENV_TEST_1233") );
            println!( "run-time:{}", env::var("ENV_TEST_1233").unwrap());
        }
        "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                ENV_TEST_1233 = "Hello"
            "#,
        )
        .build();

    p.crabgo("run")
        .with_stdout_contains("compile-time:Hello")
        .with_stdout_contains("run-time:Hello")
        .run();
}

#[crabgo_test]
fn env_invalid() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        fn main() {
        }
        "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                ENV_TEST_BOOL = false
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr_contains("[..]could not load config key `env.ENV_TEST_BOOL`")
        .run();
}

#[crabgo_test]
fn env_no_crabgo_home() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        fn main() {
        }
        "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                CARGO_HOME = "/"
            "#,
        )
        .build();

    p.crabgo("check")
        .with_status(101)
        .with_stderr_contains("[..]setting the `CARGO_HOME` environment variable is not supported in the `[env]` configuration table")
        .run();
}

#[crabgo_test]
fn env_force() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        fn main() {
            println!( "ENV_TEST_FORCED:{}", env!("ENV_TEST_FORCED") );
            println!( "ENV_TEST_UNFORCED:{}", env!("ENV_TEST_UNFORCED") );
            println!( "ENV_TEST_UNFORCED_DEFAULT:{}", env!("ENV_TEST_UNFORCED_DEFAULT") );
        }
        "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                ENV_TEST_UNFORCED_DEFAULT = "from-config"
                ENV_TEST_UNFORCED = { value = "from-config", force = false }
                ENV_TEST_FORCED = { value = "from-config", force = true }
            "#,
        )
        .build();

    p.crabgo("run")
        .env("ENV_TEST_FORCED", "from-env")
        .env("ENV_TEST_UNFORCED", "from-env")
        .env("ENV_TEST_UNFORCED_DEFAULT", "from-env")
        .with_stdout_contains("ENV_TEST_FORCED:from-config")
        .with_stdout_contains("ENV_TEST_UNFORCED:from-env")
        .with_stdout_contains("ENV_TEST_UNFORCED_DEFAULT:from-env")
        .run();
}

#[crabgo_test]
fn env_relative() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo2"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        use std::path::Path;
        fn main() {
            println!( "ENV_TEST_REGULAR:{}", env!("ENV_TEST_REGULAR") );
            println!( "ENV_TEST_REGULAR_DEFAULT:{}", env!("ENV_TEST_REGULAR_DEFAULT") );
            println!( "ENV_TEST_RELATIVE:{}", env!("ENV_TEST_RELATIVE") );

            assert!( Path::new(env!("ENV_TEST_RELATIVE")).is_absolute() );
            assert!( !Path::new(env!("ENV_TEST_REGULAR")).is_absolute() );
            assert!( !Path::new(env!("ENV_TEST_REGULAR_DEFAULT")).is_absolute() );
        }
        "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                ENV_TEST_REGULAR = { value = "Crabgo.toml", relative = false }
                ENV_TEST_REGULAR_DEFAULT = "Crabgo.toml"
                ENV_TEST_RELATIVE = { value = "Crabgo.toml", relative = true }
            "#,
        )
        .build();

    p.crabgo("run").run();
}

#[crabgo_test]
fn env_no_override() {
    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("unchanged"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        fn main() {
            println!( "CARGO_PKG_NAME:{}", env!("CARGO_PKG_NAME") );
        }
        "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                CARGO_PKG_NAME = { value = "from-config", force = true }
            "#,
        )
        .build();

    p.crabgo("run")
        .with_stdout_contains("CARGO_PKG_NAME:unchanged")
        .run();
}

#[crabgo_test]
fn env_applied_to_target_info_discovery_rustc() {
    let wrapper = project()
        .at("wrapper")
        .file("Crabgo.toml", &basic_manifest("wrapper", "1.0.0"))
        .file(
            "src/main.rs",
            r#"
            fn main() {
                let mut args = std::env::args().skip(1);
                let env_test = std::env::var("ENV_TEST").unwrap();
                eprintln!("WRAPPER ENV_TEST:{env_test}");
                let status = std::process::Command::new(&args.next().unwrap())
                    .args(args).status().unwrap();
                std::process::exit(status.code().unwrap_or(1));
            }
            "#,
        )
        .build();
    wrapper.crabgo("build").run();
    let wrapper = &wrapper.bin("wrapper");

    let p = project()
        .file("Crabgo.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
            fn main() {
                eprintln!( "MAIN ENV_TEST:{}", std::env!("ENV_TEST") );
            }
            "#,
        )
        .file(
            ".crabgo/config",
            r#"
                [env]
                ENV_TEST = "from-config"
            "#,
        )
        .build();

    p.crabgo("run")
        .env("RUSTC_WORKSPACE_WRAPPER", wrapper)
        .with_stderr_contains("WRAPPER ENV_TEST:from-config")
        .with_stderr_contains("MAIN ENV_TEST:from-config")
        .run();

    // Ensure wrapper also maintains the same overridden priority for envs.
    p.crabgo("clean").run();
    p.crabgo("run")
        .env("ENV_TEST", "from-env")
        .env("RUSTC_WORKSPACE_WRAPPER", wrapper)
        .with_stderr_contains("WRAPPER ENV_TEST:from-env")
        .with_stderr_contains("MAIN ENV_TEST:from-env")
        .run();
}
