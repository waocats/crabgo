# Debugging

## Logging

Crabgo uses the [`env_logger`] crate to display debug log messages. The
`CRABGO_LOG` environment variable can be set to enable debug logging, with a
value such as `trace`, `debug`, or `warn`. It also supports filtering for
specific modules. Feel free to use the standard [`log`] macros to help with
diagnosing problems.

```sh
# Outputs all logs with levels debug and higher
CRABGO_LOG=debug crabgo generate-lockfile

# Don't forget that you can filter by module as well
CRABGO_LOG=crabgo::core::resolver=trace crabgo generate-lockfile

# This will print lots of info about the download process. `trace` prints even more.
CRABGO_HTTP_DEBUG=true CRABGO_LOG=crabgo::ops::registry=debug crabgo fetch

# This is an important command for diagnosing fingerprint issues.
CRABGO_LOG=crabgo::core::compiler::fingerprint=trace crabgo build
```

[`env_logger`]: https://docs.rs/env_logger
[`log`]: https://docs.rs/log
