# crabgo-fetch(1)
{{*set actionverb="Fetch"}}
{{*set target-default-to-all-arch=true}}
{{*set multitarget=true}}

## NAME

crabgo-fetch --- Fetch dependencies of a package from the network

## SYNOPSIS

`crabgo fetch` [_options_]

## DESCRIPTION

If a `Crabgo.lock` file is available, this command will ensure that all of the
git dependencies and/or registry dependencies are downloaded and locally
available. Subsequent Crabgo commands will be able to run offline after a `crabgo
fetch` unless the lock file changes.

If the lock file is not available, then this command will generate the lock
file before fetching the dependencies.

If `--target` is not specified, then all target dependencies are fetched.

See also the [crabgo-prefetch](https://crates.io/crates/crabgo-prefetch)
plugin which adds a command to download popular crates. This may be useful if
you plan to use Crabgo without a network with the `--offline` flag.

## OPTIONS

### Fetch options

{{#options}}
{{> options-target-triple }}
{{/options}}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}

{{> options-locked }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Fetch all dependencies:

       crabgo fetch

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-update" 1}}, {{man "crabgo-generate-lockfile" 1}}
