# crabgo-run(1)


## NAME

crabgo-run --- Run the current package

## SYNOPSIS

`crabgo run` [_options_] [`--` _args_]

## DESCRIPTION

Run a binary or example of the local package.

All the arguments following the two dashes (`--`) are passed to the binary to
run. If you're passing arguments to both Crabgo and the binary, the ones after
`--` go to the binary, the ones before go to Crabgo.

Unlike [crabgo-test(1)](crabgo-test.html) and [crabgo-bench(1)](crabgo-bench.html), `crabgo run` sets the 
working directory of the binary executed to the current working directory, same 
as if it was executed in the shell directly.

## OPTIONS

### Package Selection

By default, the package in the current working directory is selected. The `-p`
flag can be used to choose a different package in a workspace.

<dl>

<dt class="option-term" id="option-crabgo-run--p"><a class="option-anchor" href="#option-crabgo-run--p"></a><code>-p</code> <em>spec</em></dt>
<dt class="option-term" id="option-crabgo-run---package"><a class="option-anchor" href="#option-crabgo-run---package"></a><code>--package</code> <em>spec</em></dt>
<dd class="option-desc">The package to run. See <a href="crabgo-pkgid.html">crabgo-pkgid(1)</a> for the SPEC
format.</dd>


</dl>


### Target Selection

When no target selection options are given, `crabgo run` will run the binary
target. If there are multiple binary targets, you must pass a target flag to
choose one. Or, the `default-run` field may be specified in the `[package]`
section of `Crabgo.toml` to choose the name of the binary to run by default.

<dl>

<dt class="option-term" id="option-crabgo-run---bin"><a class="option-anchor" href="#option-crabgo-run---bin"></a><code>--bin</code> <em>name</em></dt>
<dd class="option-desc">Run the specified binary.</dd>


<dt class="option-term" id="option-crabgo-run---example"><a class="option-anchor" href="#option-crabgo-run---example"></a><code>--example</code> <em>name</em></dt>
<dd class="option-desc">Run the specified example.</dd>


</dl>

### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-crabgo-run--F"><a class="option-anchor" href="#option-crabgo-run--F"></a><code>-F</code> <em>features</em></dt>
<dt class="option-term" id="option-crabgo-run---features"><a class="option-anchor" href="#option-crabgo-run---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-crabgo-run---all-features"><a class="option-anchor" href="#option-crabgo-run---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-crabgo-run---no-default-features"><a class="option-anchor" href="#option-crabgo-run---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Compilation Options

<dl>

<dt class="option-term" id="option-crabgo-run---target"><a class="option-anchor" href="#option-crabgo-run---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Run for the given architecture. The default is the host architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Crabgo run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-crabgo-run--r"><a class="option-anchor" href="#option-crabgo-run--r"></a><code>-r</code></dt>
<dt class="option-term" id="option-crabgo-run---release"><a class="option-anchor" href="#option-crabgo-run---release"></a><code>--release</code></dt>
<dd class="option-desc">Run optimized artifacts with the <code>release</code> profile.
See also the <code>--profile</code> option for choosing a specific profile by name.</dd>



<dt class="option-term" id="option-crabgo-run---profile"><a class="option-anchor" href="#option-crabgo-run---profile"></a><code>--profile</code> <em>name</em></dt>
<dd class="option-desc">Run with the given profile.
See the <a href="../reference/profiles.html">the reference</a> for more details on profiles.</dd>



<dt class="option-term" id="option-crabgo-run---ignore-rust-version"><a class="option-anchor" href="#option-crabgo-run---ignore-rust-version"></a><code>--ignore-rust-version</code></dt>
<dd class="option-desc">Run the target even if the selected Rust compiler is older than the
required Rust version as configured in the project’s <code>rust-version</code> field.</dd>



<dt class="option-term" id="option-crabgo-run---timings=fmts"><a class="option-anchor" href="#option-crabgo-run---timings=fmts"></a><code>--timings=</code><em>fmts</em></dt>
<dd class="option-desc">Output information how long each compilation takes, and track concurrency
information over time. Accepts an optional comma-separated list of output
formats; <code>--timings</code> without an argument will default to <code>--timings=html</code>.
Specifying an output format (rather than the default) is unstable and requires
<code>-Zunstable-options</code>. Valid output formats:</p>
<ul>
<li><code>html</code> (unstable, requires <code>-Zunstable-options</code>): Write a human-readable file <code>crabgo-timing.html</code> to the
<code>target/crabgo-timings</code> directory with a report of the compilation. Also write
a report to the same directory with a timestamp in the filename if you want
to look at older runs. HTML output is suitable for human consumption only,
and does not provide machine-readable timing data.</li>
<li><code>json</code> (unstable, requires <code>-Zunstable-options</code>): Emit machine-readable JSON
information about timing information.</li>
</ul></dd>




</dl>

### Output Options

<dl>
<dt class="option-term" id="option-crabgo-run---target-dir"><a class="option-anchor" href="#option-crabgo-run---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>CRABGO_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-crabgo-run--v"><a class="option-anchor" href="#option-crabgo-run--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-run---verbose"><a class="option-anchor" href="#option-crabgo-run---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-run--q"><a class="option-anchor" href="#option-crabgo-run--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-run---quiet"><a class="option-anchor" href="#option-crabgo-run---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-run---color"><a class="option-anchor" href="#option-crabgo-run---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



<dt class="option-term" id="option-crabgo-run---message-format"><a class="option-anchor" href="#option-crabgo-run---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
<dd class="option-desc">The output format for diagnostic messages. Can be specified multiple times
and consists of comma-separated values. Valid values:</p>
<ul>
<li><code>human</code> (default): Display in a human-readable text format. Conflicts with
<code>short</code> and <code>json</code>.</li>
<li><code>short</code>: Emit shorter, human-readable text messages. Conflicts with <code>human</code>
and <code>json</code>.</li>
<li><code>json</code>: Emit JSON messages to stdout. See
<a href="../reference/external-tools.html#json-messages">the reference</a>
for more details. Conflicts with <code>human</code> and <code>short</code>.</li>
<li><code>json-diagnostic-short</code>: Ensure the <code>rendered</code> field of JSON messages contains
the “short” rendering from rustc. Cannot be used with <code>human</code> or <code>short</code>.</li>
<li><code>json-diagnostic-rendered-ansi</code>: Ensure the <code>rendered</code> field of JSON messages
contains embedded ANSI color codes for respecting rustc’s default color
scheme. Cannot be used with <code>human</code> or <code>short</code>.</li>
<li><code>json-render-diagnostics</code>: Instruct Crabgo to not include rustc diagnostics
in JSON messages printed, but instead Crabgo itself should render the
JSON diagnostics coming from rustc. Crabgo’s own JSON diagnostics and others
coming from rustc are still emitted. Cannot be used with <code>human</code> or <code>short</code>.</li>
</ul></dd>



</dl>

### Manifest Options

<dl>

<dt class="option-term" id="option-crabgo-run---manifest-path"><a class="option-anchor" href="#option-crabgo-run---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Crabgo.toml</code> file. By default, Crabgo searches for the
<code>Crabgo.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-crabgo-run---frozen"><a class="option-anchor" href="#option-crabgo-run---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-crabgo-run---locked"><a class="option-anchor" href="#option-crabgo-run---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Crabgo.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Crabgo will
exit with an error. The <code>--frozen</code> flag also prevents Crabgo from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Crabgo.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-crabgo-run---offline"><a class="option-anchor" href="#option-crabgo-run---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-crabgo-run-+toolchain"><a class="option-anchor" href="#option-crabgo-run-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-run---config"><a class="option-anchor" href="#option-crabgo-run---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-run--C"><a class="option-anchor" href="#option-crabgo-run--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-run--h"><a class="option-anchor" href="#option-crabgo-run--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-run---help"><a class="option-anchor" href="#option-crabgo-run---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-run--Z"><a class="option-anchor" href="#option-crabgo-run--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


### Miscellaneous Options

<dl>
<dt class="option-term" id="option-crabgo-run--j"><a class="option-anchor" href="#option-crabgo-run--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-crabgo-run---jobs"><a class="option-anchor" href="#option-crabgo-run---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of logical CPUs. If negative, it sets the maximum number of
parallel jobs to the number of logical CPUs plus provided value.
Should not be 0.</dd>


<dt class="option-term" id="option-crabgo-run---keep-going"><a class="option-anchor" href="#option-crabgo-run---keep-going"></a><code>--keep-going</code></dt>
<dd class="option-desc">Build as many crates in the dependency graph as possible, rather than aborting
the build on the first one that fails to build. Unstable, requires
<code>-Zunstable-options</code>.</dd>


</dl>

## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Crabgo reads.


## EXIT STATUS

* `0`: Crabgo succeeded.
* `101`: Crabgo failed to complete.


## EXAMPLES

1. Build the local package and run its main target (assuming only one binary):

       crabgo run

2. Run an example with extra arguments:

       crabgo run --example exname -- --exoption exarg1 exarg2

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-build(1)](crabgo-build.html)
