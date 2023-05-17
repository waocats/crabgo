# crabgo-init(1)

## NAME

crabgo-init --- Create a new Crabgo package in an existing directory

## SYNOPSIS

`crabgo init` [_options_] [_path_]

## DESCRIPTION

This command will create a new Crabgo manifest in the current directory. Give a
path as an argument to create in the given directory.

If there are typically-named Rust source files already in the directory, those
will be used. If not, then a sample `src/main.rs` file will be created, or
`src/lib.rs` if `--lib` is passed.

If the directory is not already in a VCS repository, then a new repository
is created (see `--vcs` below).

See [crabgo-new(1)](crabgo-new.html) for a similar command which will create a new package in
a new directory.

## OPTIONS

### Init Options

<dl>

<dt class="option-term" id="option-crabgo-init---bin"><a class="option-anchor" href="#option-crabgo-init---bin"></a><code>--bin</code></dt>
<dd class="option-desc">Create a package with a binary target (<code>src/main.rs</code>).
This is the default behavior.</dd>


<dt class="option-term" id="option-crabgo-init---lib"><a class="option-anchor" href="#option-crabgo-init---lib"></a><code>--lib</code></dt>
<dd class="option-desc">Create a package with a library target (<code>src/lib.rs</code>).</dd>


<dt class="option-term" id="option-crabgo-init---edition"><a class="option-anchor" href="#option-crabgo-init---edition"></a><code>--edition</code> <em>edition</em></dt>
<dd class="option-desc">Specify the Rust edition to use. Default is 2021.
Possible values: 2015, 2018, 2021</dd>


<dt class="option-term" id="option-crabgo-init---name"><a class="option-anchor" href="#option-crabgo-init---name"></a><code>--name</code> <em>name</em></dt>
<dd class="option-desc">Set the package name. Defaults to the directory name.</dd>


<dt class="option-term" id="option-crabgo-init---vcs"><a class="option-anchor" href="#option-crabgo-init---vcs"></a><code>--vcs</code> <em>vcs</em></dt>
<dd class="option-desc">Initialize a new VCS repository for the given version control system (git,
hg, pijul, or fossil) or do not initialize any version control at all
(none). If not specified, defaults to <code>git</code> or the configuration value
<code>crabgo-new.vcs</code>, or <code>none</code> if already inside a VCS repository.</dd>


<dt class="option-term" id="option-crabgo-init---registry"><a class="option-anchor" href="#option-crabgo-init---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">This sets the <code>publish</code> field in <code>Crabgo.toml</code> to the given registry name
which will restrict publishing only to that registry.</p>
<p>Registry names are defined in <a href="../reference/config.html">Crabgo config files</a>.
If not specified, the default registry defined by the <code>registry.default</code>
config key is used. If the default registry is not set and <code>--registry</code> is not
used, the <code>publish</code> field will not be set which means that publishing will not
be restricted.</dd>


</dl>


### Display Options

<dl>
<dt class="option-term" id="option-crabgo-init--v"><a class="option-anchor" href="#option-crabgo-init--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-init---verbose"><a class="option-anchor" href="#option-crabgo-init---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-init--q"><a class="option-anchor" href="#option-crabgo-init--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-init---quiet"><a class="option-anchor" href="#option-crabgo-init---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-init---color"><a class="option-anchor" href="#option-crabgo-init---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-crabgo-init-+toolchain"><a class="option-anchor" href="#option-crabgo-init-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-init---config"><a class="option-anchor" href="#option-crabgo-init---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-init--C"><a class="option-anchor" href="#option-crabgo-init--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-init--h"><a class="option-anchor" href="#option-crabgo-init--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-init---help"><a class="option-anchor" href="#option-crabgo-init---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-init--Z"><a class="option-anchor" href="#option-crabgo-init--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Create a binary Crabgo package in the current directory:

       crabgo init

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-new(1)](crabgo-new.html)
