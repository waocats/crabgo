# crabgo(1)

## NAME

crabgo --- The Rust package manager

## SYNOPSIS

`crabgo` [_options_] _command_ [_args_]\
`crabgo` [_options_] `--version`\
`crabgo` [_options_] `--list`\
`crabgo` [_options_] `--help`\
`crabgo` [_options_] `--explain` _code_

## DESCRIPTION

This program is a package manager and build tool for the Rust language,
available at <https://rust-lang.org>.

## COMMANDS

### Build Commands

[crabgo-bench(1)](crabgo-bench.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Execute benchmarks of a package.

[crabgo-build(1)](crabgo-build.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Compile a package.

[crabgo-check(1)](crabgo-check.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Check a local package and all of its dependencies for errors.

[crabgo-clean(1)](crabgo-clean.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove artifacts that Crabgo has generated in the past.

[crabgo-doc(1)](crabgo-doc.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Build a package's documentation.

[crabgo-fetch(1)](crabgo-fetch.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Fetch dependencies of a package from the network.

[crabgo-fix(1)](crabgo-fix.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Automatically fix lint warnings reported by rustc.

[crabgo-run(1)](crabgo-run.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Run a binary or example of the local package.

[crabgo-rustc(1)](crabgo-rustc.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Compile a package, and pass extra options to the compiler.

[crabgo-rustdoc(1)](crabgo-rustdoc.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Build a package's documentation, using specified custom flags.

[crabgo-test(1)](crabgo-test.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Execute unit and integration tests of a package.

### Manifest Commands

[crabgo-generate-lockfile(1)](crabgo-generate-lockfile.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Generate `Crabgo.lock` for a project.

[crabgo-locate-project(1)](crabgo-locate-project.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Print a JSON representation of a `Crabgo.toml` file's location.

[crabgo-metadata(1)](crabgo-metadata.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Output the resolved dependencies of a package in machine-readable format.

[crabgo-pkgid(1)](crabgo-pkgid.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Print a fully qualified package specification.

[crabgo-tree(1)](crabgo-tree.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Display a tree visualization of a dependency graph.

[crabgo-update(1)](crabgo-update.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Update dependencies as recorded in the local lock file.

[crabgo-vendor(1)](crabgo-vendor.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Vendor all dependencies locally.

[crabgo-verify-project(1)](crabgo-verify-project.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Check correctness of crate manifest.

### Package Commands

[crabgo-init(1)](crabgo-init.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Create a new Crabgo package in an existing directory.

[crabgo-install(1)](crabgo-install.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Build and install a Rust binary.

[crabgo-new(1)](crabgo-new.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Create a new Crabgo package.

[crabgo-search(1)](crabgo-search.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Search packages in crates.io.

[crabgo-uninstall(1)](crabgo-uninstall.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove a Rust binary.

### Publishing Commands

[crabgo-login(1)](crabgo-login.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Save an API token from the registry locally.

[crabgo-logout(1)](crabgo-logout.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove an API token from the registry locally.

[crabgo-owner(1)](crabgo-owner.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Manage the owners of a crate on the registry.

[crabgo-package(1)](crabgo-package.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Assemble the local package into a distributable tarball.

[crabgo-publish(1)](crabgo-publish.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Upload a package to the registry.

[crabgo-yank(1)](crabgo-yank.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove a pushed crate from the index.

### General Commands

[crabgo-help(1)](crabgo-help.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Display help information about Crabgo.

[crabgo-version(1)](crabgo-version.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Show version information.

## OPTIONS

### Special Options

<dl>

<dt class="option-term" id="option-crabgo--V"><a class="option-anchor" href="#option-crabgo--V"></a><code>-V</code></dt>
<dt class="option-term" id="option-crabgo---version"><a class="option-anchor" href="#option-crabgo---version"></a><code>--version</code></dt>
<dd class="option-desc">Print version info and exit. If used with <code>--verbose</code>, prints extra
information.</dd>


<dt class="option-term" id="option-crabgo---list"><a class="option-anchor" href="#option-crabgo---list"></a><code>--list</code></dt>
<dd class="option-desc">List all installed Crabgo subcommands. If used with <code>--verbose</code>, prints extra
information.</dd>


<dt class="option-term" id="option-crabgo---explain"><a class="option-anchor" href="#option-crabgo---explain"></a><code>--explain</code> <em>code</em></dt>
<dd class="option-desc">Run <code>rustc --explain CODE</code> which will print out a detailed explanation of an
error message (for example, <code>E0004</code>).</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-crabgo--v"><a class="option-anchor" href="#option-crabgo--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo---verbose"><a class="option-anchor" href="#option-crabgo---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo--q"><a class="option-anchor" href="#option-crabgo--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo---quiet"><a class="option-anchor" href="#option-crabgo---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo---color"><a class="option-anchor" href="#option-crabgo---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



</dl>

### Manifest Options

<dl>
<dt class="option-term" id="option-crabgo---frozen"><a class="option-anchor" href="#option-crabgo---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-crabgo---locked"><a class="option-anchor" href="#option-crabgo---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Crabgo.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Crabgo will
exit with an error. The <code>--frozen</code> flag also prevents Crabgo from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Crabgo.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-crabgo---offline"><a class="option-anchor" href="#option-crabgo---offline"></a><code>--offline</code></dt>
<dd class="option-desc">Prevents Crabgo from accessing the network for any reason. Without this
flag, Crabgo will stop with an error if it needs to access the network and
the network is not available. With this flag, Crabgo will attempt to
proceed without the network if possible.</p>
<p>Beware that this may result in different dependency resolution than online
mode. Crabgo will restrict itself to crates that are downloaded locally, even
if there might be a newer version as indicated in the local copy of the index.
See the <a href="crabgo-fetch.html">crabgo-fetch(1)</a> command to download dependencies before going
offline.</p>
<p>May also be specified with the <code>net.offline</code> <a href="../reference/config.html">config value</a>.</dd>


</dl>

### Common Options

<dl>

<dt class="option-term" id="option-crabgo-+toolchain"><a class="option-anchor" href="#option-crabgo-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo---config"><a class="option-anchor" href="#option-crabgo---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo--C"><a class="option-anchor" href="#option-crabgo--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo--h"><a class="option-anchor" href="#option-crabgo--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo---help"><a class="option-anchor" href="#option-crabgo---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo--Z"><a class="option-anchor" href="#option-crabgo--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## FILES

`~/.crabgo/`\
&nbsp;&nbsp;&nbsp;&nbsp;Default location for Crabgo's "home" directory where it
stores various files. The location can be changed with the `CARGO_HOME`
environment variable.

`$CARGO_HOME/bin/`\
&nbsp;&nbsp;&nbsp;&nbsp;Binaries installed by [crabgo-install(1)](crabgo-install.html) will be located here. If using
[rustup], executables distributed with Rust are also located here.

`$CARGO_HOME/config.toml`\
&nbsp;&nbsp;&nbsp;&nbsp;The global configuration file. See [the reference](../reference/config.html)
for more information about configuration files.

`.crabgo/config.toml`\
&nbsp;&nbsp;&nbsp;&nbsp;Crabgo automatically searches for a file named `.crabgo/config.toml` in the
current directory, and all parent directories. These configuration files
will be merged with the global configuration file.

`$CARGO_HOME/credentials.toml`\
&nbsp;&nbsp;&nbsp;&nbsp;Private authentication information for logging in to a registry.

`$CARGO_HOME/registry/`\
&nbsp;&nbsp;&nbsp;&nbsp;This directory contains cached downloads of the registry index and any
downloaded dependencies.

`$CARGO_HOME/git/`\
&nbsp;&nbsp;&nbsp;&nbsp;This directory contains cached downloads of git dependencies.

Please note that the internal structure of the `$CARGO_HOME` directory is not
stable yet and may be subject to change.

[rustup]: https://rust-lang.github.io/rustup/

## EXAMPLES

1. Build a local package and all of its dependencies:

       crabgo build

2. Build a package with optimizations:

       crabgo build --release

3. Run tests for a cross-compiled target:

       crabgo test --target i686-unknown-linux-gnu

4. Create a new package that builds an executable:

       crabgo new foobar

5. Create a package in the current directory:

       mkdir foo && cd foo
       crabgo init .

6. Learn about a command's options and usage:

       crabgo help clean

## BUGS

See <https://github.com/rust-lang/crabgo/issues> for issues.

## SEE ALSO
[rustc(1)](https://doc.rust-lang.org/rustc/index.html), [rustdoc(1)](https://doc.rust-lang.org/rustdoc/index.html)
