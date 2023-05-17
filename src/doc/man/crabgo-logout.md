# crabgo-logout(1)

## NAME

crabgo-logout --- Remove an API token from the registry locally

## SYNOPSIS

`crabgo logout` [_options_]

## DESCRIPTION

This command will remove the API token from the local credential storage.
Credentials are stored in `$CARGO_HOME/credentials.toml` where `$CARGO_HOME`
defaults to `.crabgo` in your home directory.

If `--registry` is not specified, then the credentials for the default
registry will be removed (configured by
[`registry.default`](../reference/config.html#registrydefault), which defaults
to <https://crates.io/>).

This will not revoke the token on the server. If you need to revoke the token,
visit the registry website and follow its instructions (see
<https://crates.io/me> to revoke the token for <https://crates.io/>).

## OPTIONS

### Logout Options

{{#options}}
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

1. Remove the default registry token:

       crabgo logout

2. Remove the token for a specific registry:

       crabgo logout --registry my-registry

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-login" 1}}
