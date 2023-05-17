# crabgo-owner(1)

## NAME

crabgo-owner --- Manage the owners of a crate on the registry

## SYNOPSIS

`crabgo owner` [_options_] `--add` _login_ [_crate_]\
`crabgo owner` [_options_] `--remove` _login_ [_crate_]\
`crabgo owner` [_options_] `--list` [_crate_]

## DESCRIPTION

This command will modify the owners for a crate on the registry. Owners of a
crate can upload new versions and yank old versions. Non-team owners can also
modify the set of owners, so take care!

This command requires you to be authenticated with either the `--token` option
or using [crabgo-login(1)](crabgo-login.html).

If the crate name is not specified, it will use the package name from the
current directory.

See [the reference](../reference/publishing.html#crabgo-owner) for more
information about owners and publishing.

## OPTIONS

### Owner Options

<dl>

<dt class="option-term" id="option-crabgo-owner--a"><a class="option-anchor" href="#option-crabgo-owner--a"></a><code>-a</code></dt>
<dt class="option-term" id="option-crabgo-owner---add"><a class="option-anchor" href="#option-crabgo-owner---add"></a><code>--add</code> <em>login</em>…</dt>
<dd class="option-desc">Invite the given user or team as an owner.</dd>


<dt class="option-term" id="option-crabgo-owner--r"><a class="option-anchor" href="#option-crabgo-owner--r"></a><code>-r</code></dt>
<dt class="option-term" id="option-crabgo-owner---remove"><a class="option-anchor" href="#option-crabgo-owner---remove"></a><code>--remove</code> <em>login</em>…</dt>
<dd class="option-desc">Remove the given user or team as an owner.</dd>


<dt class="option-term" id="option-crabgo-owner--l"><a class="option-anchor" href="#option-crabgo-owner--l"></a><code>-l</code></dt>
<dt class="option-term" id="option-crabgo-owner---list"><a class="option-anchor" href="#option-crabgo-owner---list"></a><code>--list</code></dt>
<dd class="option-desc">List owners of a crate.</dd>


<dt class="option-term" id="option-crabgo-owner---token"><a class="option-anchor" href="#option-crabgo-owner---token"></a><code>--token</code> <em>token</em></dt>
<dd class="option-desc">API token to use when authenticating. This overrides the token stored in
the credentials file (which is created by <a href="crabgo-login.html">crabgo-login(1)</a>).</p>
<p><a href="../reference/config.html">Crabgo config</a> environment variables can be
used to override the tokens stored in the credentials file. The token for
crates.io may be specified with the <code>CRABGO_REGISTRY_TOKEN</code> environment
variable. Tokens for other registries may be specified with environment
variables of the form <code>CRABGO_REGISTRIES_NAME_TOKEN</code> where <code>NAME</code> is the name
of the registry in all capital letters.</dd>



<dt class="option-term" id="option-crabgo-owner---index"><a class="option-anchor" href="#option-crabgo-owner---index"></a><code>--index</code> <em>index</em></dt>
<dd class="option-desc">The URL of the registry index to use.</dd>



<dt class="option-term" id="option-crabgo-owner---registry"><a class="option-anchor" href="#option-crabgo-owner---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">Name of the registry to use. Registry names are defined in <a href="../reference/config.html">Crabgo config
files</a>. If not specified, the default registry is used,
which is defined by the <code>registry.default</code> config key which defaults to
<code>crates-io</code>.</dd>



</dl>

### Display Options

<dl>
<dt class="option-term" id="option-crabgo-owner--v"><a class="option-anchor" href="#option-crabgo-owner--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-owner---verbose"><a class="option-anchor" href="#option-crabgo-owner---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-owner--q"><a class="option-anchor" href="#option-crabgo-owner--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-owner---quiet"><a class="option-anchor" href="#option-crabgo-owner---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-owner---color"><a class="option-anchor" href="#option-crabgo-owner---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-crabgo-owner-+toolchain"><a class="option-anchor" href="#option-crabgo-owner-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-owner---config"><a class="option-anchor" href="#option-crabgo-owner---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-owner--C"><a class="option-anchor" href="#option-crabgo-owner--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-owner--h"><a class="option-anchor" href="#option-crabgo-owner--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-owner---help"><a class="option-anchor" href="#option-crabgo-owner---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-owner--Z"><a class="option-anchor" href="#option-crabgo-owner--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. List owners of a package:

       crabgo owner --list foo

2. Invite an owner to a package:

       crabgo owner --add username foo

3. Remove an owner from a package:

       crabgo owner --remove username foo

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-login(1)](crabgo-login.html), [crabgo-publish(1)](crabgo-publish.html)
