# crabgo-generate-lockfile(1)

## NAME

crabgo-generate-lockfile --- Generate the lockfile for a package

## SYNOPSIS

`crabgo generate-lockfile` [_options_]

## DESCRIPTION

This command will create the `Crabgo.lock` lockfile for the current package or
workspace. If the lockfile already exists, it will be rebuilt with the latest
available version of every package.

See also {{man "crabgo-update" 1}} which is also capable of creating a `Crabgo.lock`
lockfile and has more options for controlling update behavior.

## OPTIONS

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

1. Create or update the lockfile for the current package or workspace:

       crabgo generate-lockfile

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-update" 1}}
