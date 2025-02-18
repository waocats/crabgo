# crabgo-locate-project(1)

## NAME

crabgo-locate-project --- Print a JSON representation of a Crabgo.toml file's location

## SYNOPSIS

`crabgo locate-project` [_options_]

## DESCRIPTION

This command will print a JSON object to stdout with the full path to the manifest. The
manifest is found by searching upward for a file named `Crabgo.toml` starting from the current
working directory.

If the project happens to be a part of a workspace, the manifest of the project, rather than
the workspace root, is output. This can be overridden by the `--workspace` flag. The root
workspace is found by traversing further upward or by using the field `package.workspace` after
locating the manifest of a workspace member.

## OPTIONS

{{#options}}

{{#option "`--workspace`" }}
Locate the `Crabgo.toml` at the root of the workspace, as opposed to the current
workspace member.
{{/option}}

{{/options}}

### Display Options

{{#options}}

{{#option "`--message-format` _fmt_" }}
The representation in which to print the project location. Valid values:

- `json` (default): JSON object with the path under the key "root".
- `plain`: Just the path.
{{/option}}

{{> options-display }}
{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Display the path to the manifest based on the current directory:

       crabgo locate-project

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-metadata" 1}}
