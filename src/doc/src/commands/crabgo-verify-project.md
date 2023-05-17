# crabgo-verify-project(1)

## NAME

crabgo-verify-project --- Check correctness of crate manifest

## SYNOPSIS

`crabgo verify-project` [_options_]

## DESCRIPTION

This command will parse the local manifest and check its validity. It emits a
JSON object with the result. A successful validation will display:

    {"success":"true"}

An invalid workspace will display:

    {"invalid":"human-readable error message"}

## OPTIONS

### Display Options

<dl>

<dt class="option-term" id="option-crabgo-verify-project--v"><a class="option-anchor" href="#option-crabgo-verify-project--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-verify-project---verbose"><a class="option-anchor" href="#option-crabgo-verify-project---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-verify-project--q"><a class="option-anchor" href="#option-crabgo-verify-project--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-verify-project---quiet"><a class="option-anchor" href="#option-crabgo-verify-project---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-verify-project---color"><a class="option-anchor" href="#option-crabgo-verify-project---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-crabgo-verify-project---manifest-path"><a class="option-anchor" href="#option-crabgo-verify-project---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Crabgo.toml</code> file. By default, Crabgo searches for the
<code>Crabgo.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-crabgo-verify-project---frozen"><a class="option-anchor" href="#option-crabgo-verify-project---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-crabgo-verify-project---locked"><a class="option-anchor" href="#option-crabgo-verify-project---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Crabgo.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Crabgo will
exit with an error. The <code>--frozen</code> flag also prevents Crabgo from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Crabgo.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-crabgo-verify-project---offline"><a class="option-anchor" href="#option-crabgo-verify-project---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-crabgo-verify-project-+toolchain"><a class="option-anchor" href="#option-crabgo-verify-project-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-verify-project---config"><a class="option-anchor" href="#option-crabgo-verify-project---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-verify-project--C"><a class="option-anchor" href="#option-crabgo-verify-project--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-verify-project--h"><a class="option-anchor" href="#option-crabgo-verify-project--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-verify-project---help"><a class="option-anchor" href="#option-crabgo-verify-project---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-verify-project--Z"><a class="option-anchor" href="#option-crabgo-verify-project--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: The workspace is OK.
* `1`: The workspace is invalid.

## EXAMPLES

1. Check the current workspace for errors:

       crabgo verify-project

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-package(1)](crabgo-package.html)
