# crabgo-uninstall(1)

## NAME

crabgo-uninstall --- Remove a Rust binary

## SYNOPSIS

`crabgo uninstall` [_options_] [_spec_...]

## DESCRIPTION

This command removes a package installed with [crabgo-install(1)](crabgo-install.html). The _spec_
argument is a package ID specification of the package to remove (see
[crabgo-pkgid(1)](crabgo-pkgid.html)).

By default all binaries are removed for a crate but the `--bin` and
`--example` flags can be used to only remove particular binaries.

The installation root is determined, in order of precedence:

- `--root` option
- `CRABGO_INSTALL_ROOT` environment variable
- `install.root` Crabgo [config value](../reference/config.html)
- `CRABGO_HOME` environment variable
- `$HOME/.crabgo`


## OPTIONS

### Install Options

<dl>

<dt class="option-term" id="option-crabgo-uninstall--p"><a class="option-anchor" href="#option-crabgo-uninstall--p"></a><code>-p</code></dt>
<dt class="option-term" id="option-crabgo-uninstall---package"><a class="option-anchor" href="#option-crabgo-uninstall---package"></a><code>--package</code> <em>spec</em>…</dt>
<dd class="option-desc">Package to uninstall.</dd>


<dt class="option-term" id="option-crabgo-uninstall---bin"><a class="option-anchor" href="#option-crabgo-uninstall---bin"></a><code>--bin</code> <em>name</em>…</dt>
<dd class="option-desc">Only uninstall the binary <em>name</em>.</dd>


<dt class="option-term" id="option-crabgo-uninstall---root"><a class="option-anchor" href="#option-crabgo-uninstall---root"></a><code>--root</code> <em>dir</em></dt>
<dd class="option-desc">Directory to uninstall packages from.</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-crabgo-uninstall--v"><a class="option-anchor" href="#option-crabgo-uninstall--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-uninstall---verbose"><a class="option-anchor" href="#option-crabgo-uninstall---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-uninstall--q"><a class="option-anchor" href="#option-crabgo-uninstall--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-uninstall---quiet"><a class="option-anchor" href="#option-crabgo-uninstall---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-uninstall---color"><a class="option-anchor" href="#option-crabgo-uninstall---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-crabgo-uninstall-+toolchain"><a class="option-anchor" href="#option-crabgo-uninstall-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-uninstall---config"><a class="option-anchor" href="#option-crabgo-uninstall---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-uninstall--C"><a class="option-anchor" href="#option-crabgo-uninstall--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-uninstall--h"><a class="option-anchor" href="#option-crabgo-uninstall--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-uninstall---help"><a class="option-anchor" href="#option-crabgo-uninstall---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-uninstall--Z"><a class="option-anchor" href="#option-crabgo-uninstall--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Uninstall a previously installed package.

       crabgo uninstall ripgrep

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-install(1)](crabgo-install.html)
