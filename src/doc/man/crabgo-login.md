# crabgo-login(1)

## NAME

crabgo-login --- Save an API token from the registry locally

## SYNOPSIS

`crabgo login` [_options_] [_token_]

## DESCRIPTION

This command will save the API token to disk so that commands that require
authentication, such as {{man "crabgo-publish" 1}}, will be automatically
authenticated. The token is saved in `$CRABGO_HOME/credentials.toml`. `CRABGO_HOME`
defaults to `.crabgo` in your home directory.

If the _token_ argument is not specified, it will be read from stdin.

The API token for crates.io may be retrieved from <https://crates.io/me>.

Take care to keep the token secret, it should not be shared with anyone else.

## OPTIONS

### Login Options

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

1. Save the API token to disk:

       crabgo login

## SEE ALSO
{{man "crabgo" 1}}, {{man "crabgo-logout" 1}}, {{man "crabgo-publish" 1}}
