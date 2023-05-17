# crabgo-fix(1)



## NAME

crabgo-fix --- Automatically fix lint warnings reported by rustc

## SYNOPSIS

`crabgo fix` [_options_]

## DESCRIPTION

This Crabgo subcommand will automatically take rustc's suggestions from
diagnostics like warnings and apply them to your source code. This is intended
to help automate tasks that rustc itself already knows how to tell you to fix!

Executing `crabgo fix` will under the hood execute [crabgo-check(1)](crabgo-check.html). Any warnings
applicable to your crate will be automatically fixed (if possible) and all
remaining warnings will be displayed when the check process is finished. For
example if you'd like to apply all fixes to the current package, you can run:

    crabgo fix

which behaves the same as `crabgo check --all-targets`.

`crabgo fix` is only capable of fixing code that is normally compiled with
`crabgo check`. If code is conditionally enabled with optional features, you
will need to enable those features for that code to be analyzed:

    crabgo fix --features foo

Similarly, other `cfg` expressions like platform-specific code will need to
pass `--target` to fix code for the given target.

    crabgo fix --target x86_64-pc-windows-gnu

If you encounter any problems with `crabgo fix` or otherwise have any questions
or feature requests please don't hesitate to file an issue at
<https://github.com/rust-lang/crabgo>.

### Edition migration

The `crabgo fix` subcommand can also be used to migrate a package from one
[edition] to the next. The general procedure is:

1. Run `crabgo fix --edition`. Consider also using the `--all-features` flag if
   your project has multiple features. You may also want to run `crabgo fix
   --edition` multiple times with different `--target` flags if your project
   has platform-specific code gated by `cfg` attributes.
2. Modify `Crabgo.toml` to set the [edition field] to the new edition.
3. Run your project tests to verify that everything still works. If new
   warnings are issued, you may want to consider running `crabgo fix` again
   (without the `--edition` flag) to apply any suggestions given by the
   compiler.

And hopefully that's it! Just keep in mind of the caveats mentioned above that
`crabgo fix` cannot update code for inactive features or `cfg` expressions.
Also, in some rare cases the compiler is unable to automatically migrate all
code to the new edition, and this may require manual changes after building
with the new edition.

[edition]: https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html
[edition field]: ../reference/manifest.html#the-edition-field

## OPTIONS

### Fix options

<dl>

<dt class="option-term" id="option-crabgo-fix---broken-code"><a class="option-anchor" href="#option-crabgo-fix---broken-code"></a><code>--broken-code</code></dt>
<dd class="option-desc">Fix code even if it already has compiler errors. This is useful if <code>crabgo fix</code>
fails to apply the changes. It will apply the changes and leave the broken
code in the working directory for you to inspect and manually fix.</dd>


<dt class="option-term" id="option-crabgo-fix---edition"><a class="option-anchor" href="#option-crabgo-fix---edition"></a><code>--edition</code></dt>
<dd class="option-desc">Apply changes that will update the code to the next edition. This will not
update the edition in the <code>Crabgo.toml</code> manifest, which must be updated
manually after <code>crabgo fix --edition</code> has finished.</dd>


<dt class="option-term" id="option-crabgo-fix---edition-idioms"><a class="option-anchor" href="#option-crabgo-fix---edition-idioms"></a><code>--edition-idioms</code></dt>
<dd class="option-desc">Apply suggestions that will update code to the preferred style for the current
edition.</dd>


<dt class="option-term" id="option-crabgo-fix---allow-no-vcs"><a class="option-anchor" href="#option-crabgo-fix---allow-no-vcs"></a><code>--allow-no-vcs</code></dt>
<dd class="option-desc">Fix code even if a VCS was not detected.</dd>


<dt class="option-term" id="option-crabgo-fix---allow-dirty"><a class="option-anchor" href="#option-crabgo-fix---allow-dirty"></a><code>--allow-dirty</code></dt>
<dd class="option-desc">Fix code even if the working directory has changes.</dd>


<dt class="option-term" id="option-crabgo-fix---allow-staged"><a class="option-anchor" href="#option-crabgo-fix---allow-staged"></a><code>--allow-staged</code></dt>
<dd class="option-desc">Fix code even if the working directory has staged changes.</dd>


</dl>

### Package Selection

By default, when no package selection options are given, the packages selected
depend on the selected manifest file (based on the current working directory if
`--manifest-path` is not given). If the manifest is the root of a workspace then
the workspaces default members are selected, otherwise only the package defined
by the manifest will be selected.

The default members of a workspace can be set explicitly with the
`workspace.default-members` key in the root manifest. If this is not set, a
virtual workspace will include all workspace members (equivalent to passing
`--workspace`), and a non-virtual workspace will include only the root crate itself.

<dl>

<dt class="option-term" id="option-crabgo-fix--p"><a class="option-anchor" href="#option-crabgo-fix--p"></a><code>-p</code> <em>spec</em>…</dt>
<dt class="option-term" id="option-crabgo-fix---package"><a class="option-anchor" href="#option-crabgo-fix---package"></a><code>--package</code> <em>spec</em>…</dt>
<dd class="option-desc">Fix only the specified packages. See <a href="crabgo-pkgid.html">crabgo-pkgid(1)</a> for the
SPEC format. This flag may be specified multiple times and supports common Unix
glob patterns like <code>*</code>, <code>?</code> and <code>[]</code>. However, to avoid your shell accidentally 
expanding glob patterns before Crabgo handles them, you must use single quotes or
double quotes around each pattern.</dd>


<dt class="option-term" id="option-crabgo-fix---workspace"><a class="option-anchor" href="#option-crabgo-fix---workspace"></a><code>--workspace</code></dt>
<dd class="option-desc">Fix all members in the workspace.</dd>



<dt class="option-term" id="option-crabgo-fix---all"><a class="option-anchor" href="#option-crabgo-fix---all"></a><code>--all</code></dt>
<dd class="option-desc">Deprecated alias for <code>--workspace</code>.</dd>



<dt class="option-term" id="option-crabgo-fix---exclude"><a class="option-anchor" href="#option-crabgo-fix---exclude"></a><code>--exclude</code> <em>SPEC</em>…</dt>
<dd class="option-desc">Exclude the specified packages. Must be used in conjunction with the
<code>--workspace</code> flag. This flag may be specified multiple times and supports
common Unix glob patterns like <code>*</code>, <code>?</code> and <code>[]</code>. However, to avoid your shell
accidentally expanding glob patterns before Crabgo handles them, you must use
single quotes or double quotes around each pattern.</dd>


</dl>


### Target Selection

When no target selection options are given, `crabgo fix` will fix all targets
(`--all-targets` implied). Binaries are skipped if they have
`required-features` that are missing.

Passing target selection flags will fix only the specified
targets. 

Note that `--bin`, `--example`, `--test` and `--bench` flags also 
support common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your 
shell accidentally expanding glob patterns before Crabgo handles them, you must 
use single quotes or double quotes around each glob pattern.

<dl>

<dt class="option-term" id="option-crabgo-fix---lib"><a class="option-anchor" href="#option-crabgo-fix---lib"></a><code>--lib</code></dt>
<dd class="option-desc">Fix the package’s library.</dd>


<dt class="option-term" id="option-crabgo-fix---bin"><a class="option-anchor" href="#option-crabgo-fix---bin"></a><code>--bin</code> <em>name</em>…</dt>
<dd class="option-desc">Fix the specified binary. This flag may be specified multiple times
and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-crabgo-fix---bins"><a class="option-anchor" href="#option-crabgo-fix---bins"></a><code>--bins</code></dt>
<dd class="option-desc">Fix all binary targets.</dd>



<dt class="option-term" id="option-crabgo-fix---example"><a class="option-anchor" href="#option-crabgo-fix---example"></a><code>--example</code> <em>name</em>…</dt>
<dd class="option-desc">Fix the specified example. This flag may be specified multiple times
and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-crabgo-fix---examples"><a class="option-anchor" href="#option-crabgo-fix---examples"></a><code>--examples</code></dt>
<dd class="option-desc">Fix all example targets.</dd>


<dt class="option-term" id="option-crabgo-fix---test"><a class="option-anchor" href="#option-crabgo-fix---test"></a><code>--test</code> <em>name</em>…</dt>
<dd class="option-desc">Fix the specified integration test. This flag may be specified
multiple times and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-crabgo-fix---tests"><a class="option-anchor" href="#option-crabgo-fix---tests"></a><code>--tests</code></dt>
<dd class="option-desc">Fix all targets in test mode that have the <code>test = true</code> manifest
flag set. By default this includes the library and binaries built as
unittests, and integration tests. Be aware that this will also build any
required dependencies, so the lib target may be built twice (once as a
unittest, and once as a dependency for binaries, integration tests, etc.).
Targets may be enabled or disabled by setting the <code>test</code> flag in the
manifest settings for the target.</dd>


<dt class="option-term" id="option-crabgo-fix---bench"><a class="option-anchor" href="#option-crabgo-fix---bench"></a><code>--bench</code> <em>name</em>…</dt>
<dd class="option-desc">Fix the specified benchmark. This flag may be specified multiple
times and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-crabgo-fix---benches"><a class="option-anchor" href="#option-crabgo-fix---benches"></a><code>--benches</code></dt>
<dd class="option-desc">Fix all targets in benchmark mode that have the <code>bench = true</code>
manifest flag set. By default this includes the library and binaries built
as benchmarks, and bench targets. Be aware that this will also build any
required dependencies, so the lib target may be built twice (once as a
benchmark, and once as a dependency for binaries, benchmarks, etc.).
Targets may be enabled or disabled by setting the <code>bench</code> flag in the
manifest settings for the target.</dd>


<dt class="option-term" id="option-crabgo-fix---all-targets"><a class="option-anchor" href="#option-crabgo-fix---all-targets"></a><code>--all-targets</code></dt>
<dd class="option-desc">Fix all targets. This is equivalent to specifying <code>--lib --bins --tests --benches --examples</code>.</dd>


</dl>


### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-crabgo-fix--F"><a class="option-anchor" href="#option-crabgo-fix--F"></a><code>-F</code> <em>features</em></dt>
<dt class="option-term" id="option-crabgo-fix---features"><a class="option-anchor" href="#option-crabgo-fix---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-crabgo-fix---all-features"><a class="option-anchor" href="#option-crabgo-fix---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-crabgo-fix---no-default-features"><a class="option-anchor" href="#option-crabgo-fix---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Compilation Options

<dl>

<dt class="option-term" id="option-crabgo-fix---target"><a class="option-anchor" href="#option-crabgo-fix---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Fix for the given architecture. The default is the host architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets. This flag may be specified multiple times.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Crabgo run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-crabgo-fix--r"><a class="option-anchor" href="#option-crabgo-fix--r"></a><code>-r</code></dt>
<dt class="option-term" id="option-crabgo-fix---release"><a class="option-anchor" href="#option-crabgo-fix---release"></a><code>--release</code></dt>
<dd class="option-desc">Fix optimized artifacts with the <code>release</code> profile.
See also the <code>--profile</code> option for choosing a specific profile by name.</dd>



<dt class="option-term" id="option-crabgo-fix---profile"><a class="option-anchor" href="#option-crabgo-fix---profile"></a><code>--profile</code> <em>name</em></dt>
<dd class="option-desc">Fix with the given profile.</p>
<p>As a special case, specifying the <code>test</code> profile will also enable checking in
test mode which will enable checking tests and enable the <code>test</code> cfg option.
See <a href="https://doc.rust-lang.org/rustc/tests/index.html">rustc tests</a> for more
detail.</p>
<p>See the <a href="../reference/profiles.html">the reference</a> for more details on profiles.</dd>



<dt class="option-term" id="option-crabgo-fix---ignore-rust-version"><a class="option-anchor" href="#option-crabgo-fix---ignore-rust-version"></a><code>--ignore-rust-version</code></dt>
<dd class="option-desc">Fix the target even if the selected Rust compiler is older than the
required Rust version as configured in the project’s <code>rust-version</code> field.</dd>



<dt class="option-term" id="option-crabgo-fix---timings=fmts"><a class="option-anchor" href="#option-crabgo-fix---timings=fmts"></a><code>--timings=</code><em>fmts</em></dt>
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
<dt class="option-term" id="option-crabgo-fix---target-dir"><a class="option-anchor" href="#option-crabgo-fix---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>CARGO_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-crabgo-fix--v"><a class="option-anchor" href="#option-crabgo-fix--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-crabgo-fix---verbose"><a class="option-anchor" href="#option-crabgo-fix---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for “very verbose” output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-fix--q"><a class="option-anchor" href="#option-crabgo-fix--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-crabgo-fix---quiet"><a class="option-anchor" href="#option-crabgo-fix---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">Do not print crabgo log messages.
May also be specified with the <code>term.quiet</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-crabgo-fix---color"><a class="option-anchor" href="#option-crabgo-fix---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



<dt class="option-term" id="option-crabgo-fix---message-format"><a class="option-anchor" href="#option-crabgo-fix---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
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
<dt class="option-term" id="option-crabgo-fix---manifest-path"><a class="option-anchor" href="#option-crabgo-fix---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Crabgo.toml</code> file. By default, Crabgo searches for the
<code>Crabgo.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-crabgo-fix---frozen"><a class="option-anchor" href="#option-crabgo-fix---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-crabgo-fix---locked"><a class="option-anchor" href="#option-crabgo-fix---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Crabgo.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Crabgo will
exit with an error. The <code>--frozen</code> flag also prevents Crabgo from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Crabgo.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-crabgo-fix---offline"><a class="option-anchor" href="#option-crabgo-fix---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-crabgo-fix-+toolchain"><a class="option-anchor" href="#option-crabgo-fix-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Crabgo has been installed with rustup, and the first argument to <code>crabgo</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://rust-lang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-crabgo-fix---config"><a class="option-anchor" href="#option-crabgo-fix---config"></a><code>--config</code> <em>KEY=VALUE</em> or <em>PATH</em></dt>
<dd class="option-desc">Overrides a Crabgo configuration value. The argument should be in TOML syntax of <code>KEY=VALUE</code>,
or provided as a path to an extra configuration file. This flag may be specified multiple times.
See the <a href="../reference/config.html#command-line-overrides">command-line overrides section</a> for more information.</dd>


<dt class="option-term" id="option-crabgo-fix--C"><a class="option-anchor" href="#option-crabgo-fix--C"></a><code>-C</code> <em>PATH</em></dt>
<dd class="option-desc">Changes the current working directory before executing any specified operations. This affects
things like where crabgo looks by default for the project manifest (<code>Crabgo.toml</code>), as well as
the directories searched for discovering <code>.crabgo/config.toml</code>, for example. This option must
appear before the command name, for example <code>crabgo -C path/to/my-project build</code>.</p>
<p>This option is only available on the <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">nightly
channel</a> and
requires the <code>-Z unstable-options</code> flag to enable (see
<a href="https://github.com/rust-lang/crabgo/issues/10098">#10098</a>).</dd>


<dt class="option-term" id="option-crabgo-fix--h"><a class="option-anchor" href="#option-crabgo-fix--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-crabgo-fix---help"><a class="option-anchor" href="#option-crabgo-fix---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-crabgo-fix--Z"><a class="option-anchor" href="#option-crabgo-fix--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Crabgo. Run <code>crabgo -Z help</code> for details.</dd>


</dl>


### Miscellaneous Options

<dl>
<dt class="option-term" id="option-crabgo-fix--j"><a class="option-anchor" href="#option-crabgo-fix--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-crabgo-fix---jobs"><a class="option-anchor" href="#option-crabgo-fix---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of logical CPUs. If negative, it sets the maximum number of
parallel jobs to the number of logical CPUs plus provided value.
Should not be 0.</dd>


<dt class="option-term" id="option-crabgo-fix---keep-going"><a class="option-anchor" href="#option-crabgo-fix---keep-going"></a><code>--keep-going</code></dt>
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

1. Apply compiler suggestions to the local package:

       crabgo fix

2. Update a package to prepare it for the next edition:

       crabgo fix --edition

3. Apply suggested idioms for the current edition:

       crabgo fix --edition-idioms

## SEE ALSO
[crabgo(1)](crabgo.html), [crabgo-check(1)](crabgo-check.html)
