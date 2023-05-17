## Running a Registry

A minimal registry can be implemented by having a git repository that contains
an index, and a server that contains the compressed `.crate` files created by
[`crabgo package`]. Users won't be able to use Crabgo to publish to it, but this
may be sufficient for closed environments. The index format is described in
[Registry Index].

A full-featured registry that supports publishing will additionally need to
have a web API service that conforms to the API used by Crabgo. The web API is
described in [Registry Web API].

Commercial and community projects are available for building and running a
registry. See <https://github.com/rust-lang/crabgo/wiki/Third-party-registries>
for a list of what is available.

[Registry Web API]: registry-web-api.md
[Registry Index]: registry-index.md
[`crabgo publish`]: ../commands/crabgo-publish.md
[`crabgo package`]: ../commands/crabgo-package.md
