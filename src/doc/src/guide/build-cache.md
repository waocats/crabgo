## Build cache

Crabgo stores the output of a build into the "target" directory. By default,
this is the directory named `target` in the root of your
[*workspace*][def-workspace]. To change the location, you can set the
`CRABGO_TARGET_DIR` [environment variable], the [`build.target-dir`] config
value, or the `--target-dir` command-line flag.

The directory layout depends on whether or not you are using the `--target`
flag to build for a specific platform. If `--target` is not specified, Crabgo
runs in a mode where it builds for the host architecture. The output goes into
the root of the target directory, with each [profile] stored in a separate
subdirectory:

Directory | Description
----------|------------
<code style="white-space: nowrap">target/debug/</code> | Contains output for the `dev` profile.
<code style="white-space: nowrap">target/release/</code> | Contains output for the `release` profile (with the `--release` option).
<code style="white-space: nowrap">target/foo/</code> | Contains build output for the `foo` profile (with the `--profile=foo` option).

For historical reasons, the `dev` and `test` profiles are stored in the
`debug` directory, and the `release` and `bench` profiles are stored in the
`release` directory. User-defined profiles are stored in a directory with the
same name as the profile.

When building for another target with `--target`, the output is placed in a
directory with the name of the [target]:

Directory | Example
----------|--------
<code style="white-space: nowrap">target/&lt;triple&gt;/debug/</code> | <code style="white-space: nowrap">target/thumbv7em-none-eabihf/debug/</code>
<code style="white-space: nowrap">target/&lt;triple&gt;/release/</code> | <code style="white-space: nowrap">target/thumbv7em-none-eabihf/release/</code>

> **Note**: When not using `--target`, this has a consequence that Crabgo will
> share your dependencies with build scripts and proc macros. [`RUSTFLAGS`]
> will be shared with every `rustc` invocation. With the `--target` flag,
> build scripts and proc macros are built separately (for the host
> architecture), and do not share `RUSTFLAGS`.

Within the profile directory (such as `debug` or `release`), artifacts are
placed into the following directories:

Directory | Description
----------|------------
<code style="white-space: nowrap">target/debug/</code> | Contains the output of the package being built (the [binary executables] and [library targets]).
<code style="white-space: nowrap">target/debug/examples/</code> | Contains [example targets].

Some commands place their output in dedicated directories in the top level of
the `target` directory:

Directory | Description
----------|------------
<code style="white-space: nowrap">target/doc/</code> | Contains rustdoc documentation ([`crabgo doc`]).
<code style="white-space: nowrap">target/package/</code> | Contains the output of the [`crabgo package`] and [`crabgo publish`] commands.

Crabgo also creates several other directories and files needed for the build
process. Their layout is considered internal to Crabgo, and is subject to
change. Some of these directories are:

Directory | Description
----------|------------
<code style="white-space: nowrap">target/debug/deps/</code> | Dependencies and other artifacts.
<code style="white-space: nowrap">target/debug/incremental/</code> | `rustc` [incremental output], a cache used to speed up subsequent builds.
<code style="white-space: nowrap">target/debug/build/</code> | Output from [build scripts].

### Dep-info files

Next to each compiled artifact is a file called a "dep info" file with a `.d`
suffix. This file is a Makefile-like syntax that indicates all of the file
dependencies required to rebuild the artifact. These are intended to be used
with external build systems so that they can detect if Crabgo needs to be
re-executed. The paths in the file are absolute by default. See the
[`build.dep-info-basedir`] config option to use relative paths.

```Makefile
# Example dep-info file found in target/debug/foo.d
/path/to/myproj/target/debug/foo: /path/to/myproj/src/lib.rs /path/to/myproj/src/main.rs
```

### Shared cache

A third party tool, [sccache], can be used to share built dependencies across
different workspaces.

To setup `sccache`, install it with `crabgo install sccache` and set
`RUSTC_WRAPPER` environmental variable to `sccache` before invoking Crabgo. If
you use bash, it makes sense to add `export RUSTC_WRAPPER=sccache` to
`.bashrc`. Alternatively, you can set [`build.rustc-wrapper`] in the [Crabgo
configuration][config]. Refer to sccache documentation for more details.

[`RUSTFLAGS`]: ../reference/config.md#buildrustflags
[`build.dep-info-basedir`]: ../reference/config.md#builddep-info-basedir
[`build.rustc-wrapper`]: ../reference/config.md#buildrustc-wrapper
[`build.target-dir`]: ../reference/config.md#buildtarget-dir
[`crabgo doc`]: ../commands/crabgo-doc.md
[`crabgo package`]: ../commands/crabgo-package.md
[`crabgo publish`]: ../commands/crabgo-publish.md
[build scripts]: ../reference/build-scripts.md
[config]: ../reference/config.md
[def-workspace]:  ../appendix/glossary.md#workspace  '"workspace" (glossary entry)'
[target]: ../appendix/glossary.md#target '"target" (glossary entry)'
[environment variable]: ../reference/environment-variables.md
[incremental output]: ../reference/profiles.md#incremental
[sccache]: https://github.com/mozilla/sccache
[profile]: ../reference/profiles.md
[binary executables]: ../reference/crabgo-targets.md#binaries
[library targets]: ../reference/crabgo-targets.md#library
[example targets]: ../reference/crabgo-targets.md#examples
