# crabgo-logout(1)

## NAME

crabgo-logout --- Remove an API token from the registry locally

## SYNOPSIS

`crabgo logout` [_options_]

## DESCRIPTION

This command will remove the API token from the local credential storage.
Credentials are stored in `$CARGO_HOME/credentials.toml` where `$CARGO_HOME`
defaults to `.crabgo` in your home directory.

If `--registry` is not specified, then the credentials for the default
registry will be removed (configured by
[`registry.default`](../reference/config.html#registrydefault), which defaults
to <https://crates.io/>).

This will not revoke the token on the server. If you need to revoke the token,
visit the registry website and follow its instructions (see
<https://crates.io/me> to revoke the token for <https://crates.io/>).

## OPTIONS

### Logout Options

<dl>
<dt class="option-term" id="option-crabgo-logout---registry"><a class="option-anchor" href="#option-crabgo-logout---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">Name of the registry to use. Registry names are defined in <a href="../reference/config.html">Crabgo config
files</a>. If not specified, the default registry is used,
which is defined by the <code>registry.default</code> config key which defaults to
<code>crates-io</code>.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-crabgo-logout--v"><a class="option-anchor" href="#option-crabgo-logout--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-logout---verbose"><a class="option-anchor" href="#option-crabgo-logout---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-logout--q"><a class="option-anchor" href="#option-crabgo-logout--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-logout---quiet"><a class="option-anchor" href="#option-crabgo-logout---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-logout---color"><a class="option-anchor" href="#option-crabgo-logout---color"></a><code>--color</code> <em>when</em></dt>
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

### Common Options

<dl>

<dt class="option-term" id="option-crabgo-logout-+toolchain"><a class="option-anchor" href="#option-crabgo-logout-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-logout---config"><a class="option-anchor" href="#option-crabgo-logout---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-logout--C"><a class="option-anchor" href="#option-crabgo-logout--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-logout--h"><a class="option-anchor" href="#option-crabgo-logout--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-logout---help"><a class="option-anchor" href="#option-crabgo-logout---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-logout--Z"><a class="option-anchor" href="#option-crabgo-logout--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Remove the default registry token:

       crabgo logout

2. Remove the token for a specific registry:

       crabgo logout --registry my-registry

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-login(1)](crabgo-login.html)
