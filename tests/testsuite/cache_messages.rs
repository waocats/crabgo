//! Tests for caching compiler diagnostics.

use super::messages::raw_rustc_output;
use crabgo_test_support::tools;
use crabgo_test_support::{basic_manifest, is_coarse_mtime, project, registry::Package, sleep_ms};

fn as_str(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("valid utf-8")
}

#[crabgo_test]
fn simple() {
    // A simple example that generates two warnings (unused functions).
    let p = project()
        .file(
            "src/lib.rs",
            "
            fn a() {}
            fn b() {}
            ",
        )
        .build();

    // Capture what rustc actually emits. This is done to avoid relying on the
    // exact message formatting in rustc.
    let rustc_output = raw_rustc_output(&p, "src/lib.rs", &[]);

    // -q so the output is the same as rustc (no "Compiling" or "Finished").
    let crabgo_output1 = p
        .crabgo("check -q --color=never")
        .exec_with_output()
        .expect("crabgo to run");
    assert_eq!(rustc_output, as_str(&crabgo_output1.stderr));
    assert!(crabgo_output1.stdout.is_empty());
    // Check that the cached version is exactly the same.
    let crabgo_output2 = p
        .crabgo("check -q")
        .exec_with_output()
        .expect("crabgo to run");
    assert_eq!(rustc_output, as_str(&crabgo_output2.stderr));
    assert!(crabgo_output2.stdout.is_empty());
}

// same as `simple`, except everything is using the short format
#[crabgo_test]
fn simple_short() {
    let p = project()
        .file(
            "src/lib.rs",
            "
                fn a() {}
                fn b() {}
            ",
        )
        .build();

    let rustc_output = raw_rustc_output(&p, "src/lib.rs", &["--error-format=short"]);

    let crabgo_output1 = p
        .crabgo("check -q --color=never --message-format=short")
        .exec_with_output()
        .expect("crabgo to run");
    assert_eq!(rustc_output, as_str(&crabgo_output1.stderr));
    // assert!(crabgo_output1.stdout.is_empty());
    let crabgo_output2 = p
        .crabgo("check -q --message-format=short")
        .exec_with_output()
        .expect("crabgo to run");
    println!("{}", String::from_utf8_lossy(&crabgo_output2.stdout));
    assert_eq!(rustc_output, as_str(&crabgo_output2.stderr));
    assert!(crabgo_output2.stdout.is_empty());
}

#[crabgo_test]
fn color() {
    // Check enabling/disabling color.
    let p = project().file("src/lib.rs", "fn a() {}").build();

    // Hack for issue in fwdansi 1.1. It is squashing multiple resets
    // into a single reset.
    // https://github.com/kennytm/fwdansi/issues/2
    fn normalize(s: &str) -> String {
        #[cfg(windows)]
        return s.replace("\x1b[0m\x1b[0m", "\x1b[0m");
        #[cfg(not(windows))]
        return s.to_string();
    }

    let compare = |a, b| {
        assert_eq!(normalize(a), normalize(b));
    };

    // Capture the original color output.
    let rustc_color = raw_rustc_output(&p, "src/lib.rs", &["--color=always"]);
    assert!(rustc_color.contains("\x1b["));

    // Capture the original non-color output.
    let rustc_nocolor = raw_rustc_output(&p, "src/lib.rs", &[]);
    assert!(!rustc_nocolor.contains("\x1b["));

    // First pass, non-cached, with color, should be the same.
    let crabgo_output1 = p
        .crabgo("check -q --color=always")
        .exec_with_output()
        .expect("crabgo to run");
    compare(&rustc_color, as_str(&crabgo_output1.stderr));

    // Replay cached, with color.
    let crabgo_output2 = p
        .crabgo("check -q --color=always")
        .exec_with_output()
        .expect("crabgo to run");
    compare(&rustc_color, as_str(&crabgo_output2.stderr));

    // Replay cached, no color.
    let crabgo_output_nocolor = p
        .crabgo("check -q --color=never")
        .exec_with_output()
        .expect("crabgo to run");
    compare(&rustc_nocolor, as_str(&crabgo_output_nocolor.stderr));
}

#[crabgo_test]
fn cached_as_json() {
    // Check that cached JSON output is the same.
    let p = project().file("src/lib.rs", "fn a() {}").build();

    // Grab the non-cached output, feature disabled.
    // NOTE: When stabilizing, this will need to be redone.
    let crabgo_output = p
        .crabgo("check --message-format=json")
        .exec_with_output()
        .expect("crabgo to run");
    assert!(crabgo_output.status.success());
    let orig_crabgo_out = as_str(&crabgo_output.stdout);
    assert!(orig_crabgo_out.contains("compiler-message"));
    p.crabgo("clean").run();

    // Check JSON output, not fresh.
    let crabgo_output1 = p
        .crabgo("check --message-format=json")
        .exec_with_output()
        .expect("crabgo to run");
    assert_eq!(as_str(&crabgo_output1.stdout), orig_crabgo_out);

    // Check JSON output, fresh.
    let crabgo_output2 = p
        .crabgo("check --message-format=json")
        .exec_with_output()
        .expect("crabgo to run");
    // The only difference should be this field.
    let fix_fresh = as_str(&crabgo_output2.stdout).replace("\"fresh\":true", "\"fresh\":false");
    assert_eq!(fix_fresh, orig_crabgo_out);
}

#[crabgo_test]
fn clears_cache_after_fix() {
    // Make sure the cache is invalidated when there is no output.
    let p = project().file("src/lib.rs", "fn asdf() {}").build();
    // Fill the cache.
    p.crabgo("check").with_stderr_contains("[..]asdf[..]").run();
    let cpath = p
        .glob("target/debug/.fingerprint/foo-*/output-*")
        .next()
        .unwrap()
        .unwrap();
    assert!(std::fs::read_to_string(cpath).unwrap().contains("asdf"));

    // Fix it.
    if is_coarse_mtime() {
        sleep_ms(1000);
    }
    p.change_file("src/lib.rs", "");

    p.crabgo("check")
        .with_stdout("")
        .with_stderr(
            "\
[CHECKING] foo [..]
[FINISHED] [..]
",
        )
        .run();
    assert_eq!(
        p.glob("target/debug/.fingerprint/foo-*/output-*").count(),
        0
    );

    // And again, check the cache is correct.
    p.crabgo("check")
        .with_stdout("")
        .with_stderr(
            "\
[FINISHED] [..]
",
        )
        .run();
}

#[crabgo_test]
fn rustdoc() {
    // Create a warning in rustdoc.
    let p = project()
        .file(
            "src/lib.rs",
            "
            #![warn(missing_docs)]
            pub fn f() {}
            ",
        )
        .build();

    let rustdoc_output = p
        .crabgo("doc -q --color=always")
        .exec_with_output()
        .expect("rustdoc to run");
    assert!(rustdoc_output.status.success());
    let rustdoc_stderr = as_str(&rustdoc_output.stderr);
    assert!(rustdoc_stderr.contains("missing"));
    assert!(rustdoc_stderr.contains("\x1b["));
    assert_eq!(
        p.glob("target/debug/.fingerprint/foo-*/output-*").count(),
        1
    );

    // Check the cached output.
    let rustdoc_output = p
        .crabgo("doc -q --color=always")
        .exec_with_output()
        .expect("rustdoc to run");
    assert_eq!(as_str(&rustdoc_output.stderr), rustdoc_stderr);
}

#[crabgo_test]
fn fix() {
    // Make sure `fix` is not broken by caching.
    let p = project().file("src/lib.rs", "pub fn try() {}").build();

    p.crabgo("fix --edition --allow-no-vcs").run();

    assert_eq!(p.read_file("src/lib.rs"), "pub fn r#try() {}");
}

#[crabgo_test]
fn very_verbose() {
    // Handle cap-lints in dependencies.
    Package::new("bar", "1.0.0")
        .file("src/lib.rs", "fn not_used() {}")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            bar = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.crabgo("check -vv")
        .with_stderr_contains("[..]not_used[..]")
        .run();

    p.crabgo("check").with_stderr("[FINISHED] [..]").run();

    p.crabgo("check -vv")
        .with_stderr_contains("[..]not_used[..]")
        .run();
}

#[crabgo_test]
fn doesnt_create_extra_files() {
    // Ensure it doesn't create `output` files when not needed.
    Package::new("dep", "1.0.0")
        .file("src/lib.rs", "fn unused() {}")
        .publish();

    let p = project()
        .file(
            "Crabgo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                dep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .build();

    p.crabgo("check").run();

    assert_eq!(
        p.glob("target/debug/.fingerprint/foo-*/output-*").count(),
        0
    );
    assert_eq!(
        p.glob("target/debug/.fingerprint/dep-*/output-*").count(),
        0
    );
    if is_coarse_mtime() {
        sleep_ms(1000);
    }
    p.change_file("src/lib.rs", "fn unused() {}");
    p.crabgo("check").run();
    assert_eq!(
        p.glob("target/debug/.fingerprint/foo-*/output-*").count(),
        1
    );
}

#[crabgo_test]
fn replay_non_json() {
    // Handles non-json output.
    let rustc = project()
        .at("rustc")
        .file("Crabgo.toml", &basic_manifest("rustc_alt", "1.0.0"))
        .file(
            "src/main.rs",
            r#"
            fn main() {
                eprintln!("line 1");
                eprintln!("line 2");
                let r = std::process::Command::new("rustc")
                    .args(std::env::args_os().skip(1))
                    .status();
                std::process::exit(r.unwrap().code().unwrap_or(2));
            }
            "#,
        )
        .build();
    rustc.crabgo("build").run();
    let p = project().file("src/lib.rs", "").build();
    p.crabgo("check")
        .env("RUSTC", rustc.bin("rustc_alt"))
        .with_stderr(
            "\
[CHECKING] foo [..]
line 1
line 2
[FINISHED] dev [..]
",
        )
        .run();

    p.crabgo("check")
        .env("RUSTC", rustc.bin("rustc_alt"))
        .with_stderr(
            "\
line 1
line 2
[FINISHED] dev [..]
",
        )
        .run();
}

#[crabgo_test]
fn caching_large_output() {
    // Handles large number of messages.
    // This is an arbitrary amount that is greater than the 100 used in
    // job_queue. This is here to check for deadlocks or any other problems.
    const COUNT: usize = 250;
    let rustc = project()
        .at("rustc")
        .file("Crabgo.toml", &basic_manifest("rustc_alt", "1.0.0"))
        .file(
            "src/main.rs",
            &format!(
                r#"
                fn main() {{
                    for i in 0..{} {{
                        eprintln!("{{{{\"message\": \"test message {{}}\", \"level\": \"warning\", \
                            \"spans\": [], \"children\": [], \"rendered\": \"test message {{}}\"}}}}",
                            i, i);
                    }}
                    let r = std::process::Command::new("rustc")
                        .args(std::env::args_os().skip(1))
                        .status();
                    std::process::exit(r.unwrap().code().unwrap_or(2));
                }}
                "#,
                COUNT
            ),
        )
        .build();

    let mut expected = String::new();
    for i in 0..COUNT {
        expected.push_str(&format!("test message {}\n", i));
    }

    rustc.crabgo("build").run();
    let p = project().file("src/lib.rs", "").build();
    p.crabgo("check")
        .env("RUSTC", rustc.bin("rustc_alt"))
        .with_stderr(&format!(
            "\
[CHECKING] foo [..]
{}warning: `foo` (lib) generated 250 warnings
[FINISHED] dev [..]
",
            expected
        ))
        .run();

    p.crabgo("check")
        .env("RUSTC", rustc.bin("rustc_alt"))
        .with_stderr(&format!(
            "\
{}warning: `foo` (lib) generated 250 warnings
[FINISHED] dev [..]
",
            expected
        ))
        .run();
}

#[crabgo_test]
fn rustc_workspace_wrapper() {
    let p = project()
        .file(
            "src/lib.rs",
            "pub fn f() { assert!(true); }\n\
             fn unused_func() {}",
        )
        .build();

    p.crabgo("check -v")
        .env("RUSTC_WORKSPACE_WRAPPER", tools::echo_wrapper())
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name foo src/lib.rs [..]")
        .run();

    // Check without a wrapper should rebuild
    p.crabgo("check -v")
        .with_stderr_contains(
            "\
[CHECKING] foo [..]
[RUNNING] `rustc[..]
[WARNING] [..]unused_func[..]
",
        )
        .with_stdout_does_not_contain("WRAPPER CALLED: rustc --crate-name foo src/lib.rs [..]")
        .run();

    // Again, reading from the cache.
    p.crabgo("check -v")
        .env("RUSTC_WORKSPACE_WRAPPER", tools::echo_wrapper())
        .with_stderr_contains("[FRESH] foo [..]")
        .with_stdout_does_not_contain("WRAPPER CALLED: rustc --crate-name foo src/lib.rs [..]")
        .run();

    // And `check` should also be fresh, reading from cache.
    p.crabgo("check -v")
        .with_stderr_contains("[FRESH] foo [..]")
        .with_stderr_contains("[WARNING] [..]unused_func[..]")
        .with_stdout_does_not_contain("WRAPPER CALLED: rustc --crate-name foo src/lib.rs [..]")
        .run();
}

#[crabgo_test]
fn wacky_hashless_fingerprint() {
    // On Windows, executables don't have hashes. This checks for a bad
    // assumption that caused bad caching.
    let p = project()
        .file("src/bin/a.rs", "fn main() { let unused = 1; }")
        .file("src/bin/b.rs", "fn main() {}")
        .build();
    p.crabgo("check --bin b")
        .with_stderr_does_not_contain("[..]unused[..]")
        .run();
    p.crabgo("check --bin a")
        .with_stderr_contains("[..]unused[..]")
        .run();
    // This should not pick up the cache from `a`.
    p.crabgo("check --bin b")
        .with_stderr_does_not_contain("[..]unused[..]")
        .run();
}
