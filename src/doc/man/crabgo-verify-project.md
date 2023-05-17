# crabgo-verify-project(1)

## NAME

crabgo-verify-project --- Check correctness of crate manifest

## SYNOPSIS

`crabgo verify-project` [_options_]

## DESCRIPTION

This command will parse the local manifest and check its validity. It emits a
JSON object with the result. A successful validation will display:

    {"success":"true"}

An invalid workspace will display:

    {"invalid":"human-readable error message"}

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

## EXIT STATUS

* `0`: The workspace is OK.
* `1`: The workspace is invalid.

## EXAMPLES

1. Check the current workspace for errors:

       crabgo verify-project

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-package" 1}}
