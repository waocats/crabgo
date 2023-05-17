# crabgo-locate-project(1)

## NAME

crabgo-locate-project --- Print a JSON representation of a Crabgo.toml file's location

## SYNOPSIS

`crabgo locate-project` [_options_]

## DESCRIPTION

This command will print a JSON object to stdout with the full path to the manifest. The
manifest is found by searching upward for a file named `Crabgo.toml` starting from the current
working directory.

If the project happens to be a part of a workspace, the manifest of the project, rather than
the workspace root, is output. This can be overridden by the `--workspace` flag. The root
workspace is found by traversing further upward or by using the field `package.workspace` after
locating the manifest of a workspace member.

## OPTIONS

<dl>

<dt class="option-term" id="option-crabgo-locate-project---workspace"><a class="option-anchor" href="#option-crabgo-locate-project---workspace"></a><code>--workspace</code></dt>
<dd class="option-desc">Locate the <code>Crabgo.toml</code> at the root of the workspace, as opposed to the current
workspace member.</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-crabgo-locate-project---message-format"><a class="option-anchor" href="#option-crabgo-locate-project---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
<dd class="option-desc">The representation in which to print the project location. Valid values:</p>
<ul>
<li><code>json</code> (default): JSON object with the path under the key “root”.</li>
<li><code>plain</code>: Just the path.</li>
</ul></dd>


<dt class="option-term" id="option-crabgo-locate-project--v"><a class="option-anchor" href="#option-crabgo-locate-project--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-locate-project---verbose"><a class="option-anchor" href="#option-crabgo-locate-project---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-locate-project--q"><a class="option-anchor" href="#option-crabgo-locate-project--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-locate-project---quiet"><a class="option-anchor" href="#option-crabgo-locate-project---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-locate-project---color"><a class="option-anchor" href="#option-crabgo-locate-project---color"></a><code>--color</code> <em>when</em></dt>
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
<dt class="option-term" id="option-crabgo-locate-project---manifest-path"><a class="option-anchor" href="#option-crabgo-locate-project---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Crabgo.toml</code> file. By default, Crabgo searches for the
<code>Crabgo.toml</code> file in the current directory or any parent directory.</dd>


</dl>

### Common Options

<dl>

<dt class="option-term" id="option-crabgo-locate-project-+toolchain"><a class="option-anchor" href="#option-crabgo-locate-project-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-locate-project---config"><a class="option-anchor" href="#option-crabgo-locate-project---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-locate-project--C"><a class="option-anchor" href="#option-crabgo-locate-project--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-locate-project--h"><a class="option-anchor" href="#option-crabgo-locate-project--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-locate-project---help"><a class="option-anchor" href="#option-crabgo-locate-project---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-locate-project--Z"><a class="option-anchor" href="#option-crabgo-locate-project--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Display the path to the manifest based on the current directory:

       crabgo locate-project

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-metadata(1)](crabgo-metadata.html)
