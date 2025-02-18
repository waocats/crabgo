# crabgo-vendor(1)

## NAME

crabgo-vendor --- Vendor all dependencies locally

## SYNOPSIS

`crabgo vendor` [_options_] [_path_]

## DESCRIPTION

This crabgo subcommand will vendor all crates.io and git dependencies for a
project into the specified directory at `<path>`. After this command completes
the vendor directory specified by `<path>` will contain all remote sources from
dependencies specified. Additional manifests beyond the default one can be
specified with the `-s` option.

The `crabgo vendor` command will also print out the configuration necessary
to use the vendored sources, which you will need to add to `.crabgo/config.toml`.

## OPTIONS

### Vendor Options

{{#options}}

{{#option "`-s` _manifest_" "`--sync` _manifest_" }}
Specify an extra `Crabgo.toml` manifest to workspaces which should also be
vendored and synced to the output. May be specified multiple times.
{{/option}}

{{#option "`--no-delete`" }}
Don't delete the "vendor" directory when vendoring, but rather keep all
existing contents of the vendor directory
{{/option}}

{{#option "`--respect-source-config`" }}
Instead of ignoring `[source]` configuration by default in `.crabgo/config.toml`
read it and use it when downloading crates from crates.io, for example
{{/option}}

{{#option "`--versioned-dirs`" }}
Normally versions are only added to disambiguate multiple versions of the
same package. This option causes all directories in the "vendor" directory
to be versioned, which makes it easier to track the history of vendored
packages over time, and can help with the performance of re-vendoring when
only a subset of the packages have changed.
{{/option}}

{{/options}}

### Manifest Options

{{#options}}

{{> options-manifest-path }}

{{> options-locked }}

{{/options}}

### Display Options

{{#options}}

{{> options-display }}

{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Vendor all dependencies into a local "vendor" folder

       crabgo vendor

2. Vendor all dependencies into a local "third-party/vendor" folder

       crabgo vendor third-party/vendor

3. Vendor the current workspace as well as another to "vendor"

       crabgo vendor -s ../path/to/Crabgo.toml

## SEE ALSO
{{man "crabgo" 1}}

