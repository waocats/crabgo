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

See {{man "crabgo-new" 1}} for a similar command which will create a new package in
a new directory.

## OPTIONS

### Init Options

{{> options-new }}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Create a binary Crabgo package in the current directory:

       crabgo init

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-new" 1}}
