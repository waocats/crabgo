# crabgo-search(1)

## NAME

crabgo-search --- Search packages in crates.io

## SYNOPSIS

`crabgo search` [_options_] [_query_...]

## DESCRIPTION

This performs a textual search for crates on <https://crates.io>. The matching
crates will be displayed along with their description in TOML format suitable
for copying into a `Crabgo.toml` manifest.

## OPTIONS

### Search Options

<dl>

<dt class="option-term" id="option-crabgo-search---limit"><a class="option-anchor" href="#option-crabgo-search---limit"></a><code>--limit</code> <em>limit</em></dt>
<dd class="option-desc">Limit the number of results (default: 10, max: 100).</dd>


<dt class="option-term" id="option-crabgo-search---index"><a class="option-anchor" href="#option-crabgo-search---index"></a><code>--index</code> <em>index</em></dt>
<dd class="option-desc">The URL of the registry index to use.</dd>



<dt class="option-term" id="option-crabgo-search---registry"><a class="option-anchor" href="#option-crabgo-search---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">Name of the registry to use. Registry names are defined in <a href="../reference/config.html">Crabgo config
files</a>. If not specified, the default registry is used,
which is defined by the <code>registry.default</code> config key which defaults to
<code>crates-io</code>.</dd>



</dl>

### Display Options

<dl>
<dt class="option-term" id="option-crabgo-search--v"><a class="option-anchor" href="#option-crabgo-search--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-search---verbose"><a class="option-anchor" href="#option-crabgo-search---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-search--q"><a class="option-anchor" href="#option-crabgo-search--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-search---quiet"><a class="option-anchor" href="#option-crabgo-search---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-search---color"><a class="option-anchor" href="#option-crabgo-search---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-crabgo-search-+toolchain"><a class="option-anchor" href="#option-crabgo-search-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-search---config"><a class="option-anchor" href="#option-crabgo-search---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-search--C"><a class="option-anchor" href="#option-crabgo-search--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-search--h"><a class="option-anchor" href="#option-crabgo-search--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-search---help"><a class="option-anchor" href="#option-crabgo-search---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-search--Z"><a class="option-anchor" href="#option-crabgo-search--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Search for a package from crates.io:

       crabgo search serde

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-install(1)](crabgo-install.html), [crabgo-publish(1)](crabgo-publish.html)
