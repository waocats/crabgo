# crabgo-pkgid(1)

## NAME

crabgo-pkgid --- Print a fully qualified package specification

## SYNOPSIS

`crabgo pkgid` [_options_] [_spec_]

## DESCRIPTION

Given a _spec_ argument, print out the fully qualified package ID specifier
for a package or dependency in the current workspace. This command will
generate an error if _spec_ is ambiguous as to which package it refers to in
the dependency graph. If no _spec_ is given, then the specifier for the local
package is printed.

This command requires that a lockfile is available and dependencies have been
fetched.

A package specifier consists of a name, version, and source URL. You are
allowed to use partial specifiers to succinctly match a specific package as
long as it matches only one package. The format of a _spec_ can be one of the
following:

SPEC Structure             | Example SPEC
---------------------------|--------------
_name_                     | `bitflags`
_name_`@`_version_         | `bitflags@1.0.4`
_url_                      | `https://github.com/rust-lang/crabgo`
_url_`#`_version_          | `https://github.com/rust-lang/crabgo#0.33.0`
_url_`#`_name_             | `https://github.com/rust-lang/crates.io-index#bitflags`
_url_`#`_name_`@`_version_ | `https://github.com/rust-lang/crabgo#crates-io@0.21.0`

## OPTIONS

### Package Selection

<dl>

<dt class="option-term" id="option-crabgo-pkgid--p"><a class="option-anchor" href="#option-crabgo-pkgid--p"></a><code>-p</code> <em>spec</em></dt>
<dt class="option-term" id="option-crabgo-pkgid---package"><a class="option-anchor" href="#option-crabgo-pkgid---package"></a><code>--package</code> <em>spec</em></dt>
<dd class="option-desc">Get the package ID for the given package instead of the current package.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-crabgo-pkgid--v"><a class="option-anchor" href="#option-crabgo-pkgid--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-pkgid---verbose"><a class="option-anchor" href="#option-crabgo-pkgid---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-pkgid--q"><a class="option-anchor" href="#option-crabgo-pkgid--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-pkgid---quiet"><a class="option-anchor" href="#option-crabgo-pkgid---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-pkgid---color"><a class="option-anchor" href="#option-crabgo-pkgid---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-crabgo-pkgid---manifest-path"><a class="option-anchor" href="#option-crabgo-pkgid---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Crabgo.toml</code> file. By default, Crabgo searches for the
<code>Crabgo.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-crabgo-pkgid---frozen"><a class="option-anchor" href="#option-crabgo-pkgid---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-crabgo-pkgid---locked"><a class="option-anchor" href="#option-crabgo-pkgid---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Crabgo.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Crabgo will
exit with an error. The <code>--frozen</code> flag also prevents Crabgo from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Crabgo.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-crabgo-pkgid---offline"><a class="option-anchor" href="#option-crabgo-pkgid---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-crabgo-pkgid-+toolchain"><a class="option-anchor" href="#option-crabgo-pkgid-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-pkgid---config"><a class="option-anchor" href="#option-crabgo-pkgid---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-pkgid--C"><a class="option-anchor" href="#option-crabgo-pkgid--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-pkgid--h"><a class="option-anchor" href="#option-crabgo-pkgid--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-pkgid---help"><a class="option-anchor" href="#option-crabgo-pkgid---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-pkgid--Z"><a class="option-anchor" href="#option-crabgo-pkgid--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Retrieve package specification for `foo` package:

       crabgo pkgid foo

2. Retrieve package specification for version 1.0.0 of `foo`:

       crabgo pkgid foo@1.0.0

3. Retrieve package specification for `foo` from crates.io:

       crabgo pkgid https://github.com/rust-lang/crates.io-index#foo

4. Retrieve package specification for `foo` from a local package:

       crabgo pkgid file:///path/to/local/package#foo

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-generate-lockfile(1)](crabgo-generate-lockfile.html), [crabgo-metadata(1)](crabgo-metadata.html)
