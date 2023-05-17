//! General error tests that don't belong anywhere else.

use crabgo_test_support::crabgo_process;

#[crabgo_test]
fn internal_error() {
    crabgo_process("init")
        .env("__CRABGO_TEST_INTERNAL_ERROR", "1")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] internal error test
[NOTE] this is an unexpected crabgo internal error
[NOTE] we would appreciate a bug report: https://github.com/rust-lang/crabgo/issues/
[NOTE] crabgo [..]
",
        )
        .run();
}
