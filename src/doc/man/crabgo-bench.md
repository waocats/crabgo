# crabgo-bench(1)
{{*set actionverb="Benchmark"}}
{{*set nouns="benchmarks"}}
{{*set multitarget=true}}

## NAME

crabgo-bench --- Execute benchmarks of a package

## SYNOPSIS

`crabgo bench` [_options_] [_benchname_] [`--` _bench-options_]

## DESCRIPTION

Compile and execute benchmarks.

The benchmark filtering argument _benchname_ and all the arguments following
the two dashes (`--`) are passed to the benchmark binaries and thus to
_libtest_ (rustc's built in unit-test and micro-benchmarking framework). If
you are passing arguments to both Crabgo and the binary, the ones after `--` go
to the binary, the ones before go to Crabgo. For details about libtest's
arguments see the output of `crabgo bench -- --help` and check out the rustc
book's chapter on how tests work at
<https://doc.rust-lang.org/rustc/tests/index.html>.

As an example, this will run only the benchmark named `foo` (and skip other
similarly named benchmarks like `foobar`):

    crabgo bench -- foo --exact

Benchmarks are built with the `--test` option to `rustc` which creates a
special executable by linking your code with libtest. The executable
automatically runs all functions annotated with the `#[bench]` attribute.
Crabgo passes the `--bench` flag to the test harness to tell it to run
only benchmarks.

The libtest harness may be disabled by setting `harness = false` in the target
manifest settings, in which case your code will need to provide its own `main`
function to handle running benchmarks.

> **Note**: The
> [`#[bench]` attribute](https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html)
> is currently unstable and only available on the
> [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html).
> There are some packages available on
> [crates.io](https://crates.io/keywords/benchmark) that may help with
> running benchmarks on the stable channel, such as
> [Criterion](https://crates.io/crates/criterion).

By default, `crabgo bench` uses the [`bench` profile], which enables
optimizations and disables debugging information. If you need to debug a
benchmark, you can use the `--profile=dev` command-line option to switch to
the dev profile. You can then run the debug-enabled benchmark within a
debugger.

[`bench` profile]: ../reference/profiles.html#bench

### Working directory of benchmarks

The working directory of every benchmark is set to the root directory of the 
package the benchmark belongs to.
Setting the working directory of benchmarks to the package's root directory 
makes it possible for benchmarks to reliably access the package's files using 
relative paths, regardless from where `crabgo bench` was executed from.

## OPTIONS

### Benchmark Options

{{> options-test }}

{{> section-package-selection }}

### Target Selection

When no target selection options are given, `crabgo bench` will build the
following targets of the selected packages:

- lib --- used to link with binaries and benchmarks
- bins (only if benchmark targets are built and required features are
  available)
- lib as a benchmark
- bins as benchmarks
- benchmark targets

The default behavior can be changed by setting the `bench` flag for the target
in the manifest settings. Setting examples to `bench = true` will build and
run the example as a benchmark, replacing the example's `main` function with
the libtest harness.

Setting targets to `bench = false` will stop them from being bencharmked by
default. Target selection options that take a target by name (such as
`--example foo`) ignore the `bench` flag and will always benchmark the given
target.

See [Configuring a target](../reference/crabgo-targets.html#configuring-a-target)
for more information on per-target settings.

{{> options-targets-bin-auto-built }}

{{> options-targets }}

{{> section-features }}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-profile }}

{{> options-ignore-rust-version }}

{{> options-timings }}

{{/options}}

### Output Options

{{#options}}
{{> options-target-dir }}
{{/options}}

### Display Options

By default the Rust test harness hides output from benchmark execution to keep
results readable. Benchmark output can be recovered (e.g., for debugging) by
passing `--nocapture` to the benchmark binaries:

    crabgo bench -- --nocapture

{{#options}}

{{> options-display }}

{{> options-message-format }}

{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}

{{> options-locked }}
{{/options}}

{{> section-options-common }}

### Miscellaneous Options

The `--jobs` argument affects the building of the benchmark executable but
does not affect how many threads are used when running the benchmarks. The
Rust test harness runs benchmarks serially in a single thread.

{{#options}}
{{> options-jobs }}
{{> options-keep-going }}
{{/options}}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Build and execute all the benchmarks of the current package:

       crabgo bench

2. Run only a specific benchmark within a specific benchmark target:

       crabgo bench --bench bench_name -- modname::some_benchmark

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-test" 1}}
