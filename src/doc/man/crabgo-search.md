# crabgo-search(1)

## NAME

crabgo-search --- Search packages in crates.io

## SYNOPSIS

`crabgo search` [_options_] [_query_...]

## DESCRIPTION

This performs a textual search for crates on <https://crates.io>. The matching
crates will be displayed along with their description in TOML format suitable
for copying into a `Crabgo.toml` manifest.

## OPTIONS

### Search Options

{{#options}}

{{#option "`--limit` _limit_" }}
Limit the number of results (default: 10, max: 100).
{{/option}}

{{> options-index }}

{{> options-registry }}

{{/options}}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Search for a package from crates.io:

       crabgo search serde

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-install" 1}}, {{man "crabgo-publish" 1}}
