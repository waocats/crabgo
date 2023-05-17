# crabgo-uninstall(1)

## NAME

crabgo-uninstall --- Remove a Rust binary

## SYNOPSIS

`crabgo uninstall` [_options_] [_spec_...]

## DESCRIPTION

This command removes a package installed with {{man "crabgo-install" 1}}. The _spec_
argument is a package ID specification of the package to remove (see
{{man "crabgo-pkgid" 1}}).

By default all binaries are removed for a crate but the `--bin` and
`--example` flags can be used to only remove particular binaries.

{{> description-install-root }}

## OPTIONS

### Install Options

{{#options}}

{{#option "`-p`" "`--package` _spec_..." }}
Package to uninstall.
{{/option}}

{{#option "`--bin` _name_..." }}
Only uninstall the binary _name_.
{{/option}}

{{#option "`--root` _dir_" }}
Directory to uninstall packages from.
{{/option}}

{{/options}}

### Display Options

{{#options}}

{{> options-display }}

{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Uninstall a previously installed package.

       crabgo uninstall ripgrep

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-install" 1}}
