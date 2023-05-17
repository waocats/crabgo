# crabgo-new(1)

## NAME

crabgo-new --- Create a new Crabgo package

## SYNOPSIS

`crabgo new` [_options_] _path_

## DESCRIPTION

This command will create a new Crabgo package in the given directory. This
includes a simple template with a `Crabgo.toml` manifest, sample source file,
and a VCS ignore file. If the directory is not already in a VCS repository,
then a new repository is created (see `--vcs` below).

See {{man "crabgo-init" 1}} for a similar command which will create a new manifest
in an existing directory.

## OPTIONS

### New Options

{{> options-new }}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Create a binary Crabgo package in the given directory:

       crabgo new foo

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-init" 1}}
