# Release process

Crabgo is released with `rustc` using a ["train model"][choochoo]. After a
change lands in Crabgo's master branch, it will be synced with the
[rust-lang/rust] repository by a Crabgo team member, which happens about once a
week. If there are complications, it can take longer. After it is synced and
merged, the changes will appear in the next nightly release, which is usually
published around 00:30 UTC.

After changes are in the nightly release, they will make their way to the
stable release anywhere from 6 to 12 weeks later, depending on when during the
cycle it landed.

The current release schedule is posted on the [Rust Forge]. See the [release
process] for more details on how Rust's releases are created. Rust releases
are managed by the [Release team].

[Rust Forge]: https://forge.rust-lang.org/

## Build process

The build process for Crabgo is handled as part of building Rust. Every PR on
the [rust-lang/rust] repository creates a full collection of release artifacts
for every platform. The code for this is in the [`dist` bootstrap module].
Every night at 00:00 UTC, the artifacts from the most recently merged PR are
promoted to the nightly release channel. A similar process happens for beta
and stable releases.

[`dist` bootstrap module]: https://github.com/rust-lang/rust/blob/master/src/bootstrap/dist.rs

## Submodule updates

Crabgo is tracked in the [rust-lang/rust] repository using a [git submodule].
It is updated manually about once a week by a Crabgo team member.
However, anyone is welcome to update it as needed.

[@ehuss] has a tool called [subup](https://github.com/ehuss/subup) to automate the process of updating the submodule, updating the lockfile, running tests, and creating a PR.
Running the tests ahead-of-time helps avoid long cycle times waiting for bors if there are any errors.
Subup will also provide a message to include in the PR with a list of all PRs it covers.
Posting this in the PR message also helps create reference links on each Crabgo PR to the submodule update PR to help track when it gets merged.

The following is an example of the command to run in a local clone of rust-lang/rust to run a certain set of tests of things that are likely to get broken by a Crabgo update:

```bash
subup --up-branch update-crabgo \
    --commit-message "Update crabgo" \
    --test="src/tools/linkchecker tidy \
        src/tools/crabgo \
        src/tools/rustfmt" \
    src/tools/crabgo
```

If doing a [beta backport](#beta-backports), the command is similar, but needs to point to the correct branches:

```bash
subup --up-branch update-beta-crabgo \
    --rust-branch beta \
    --set-config rust.channel=beta \
    --commit-message "[beta] Update crabgo" \
    --test="src/tools/linkchecker tidy \
        src/tools/crabgo \
        src/tools/rustfmt" \
    rust-1.66.0:src/tools/crabgo
```

[@ehuss]: https://github.com/ehuss/
[git submodule]: https://git-scm.com/book/en/v2/Git-Tools-Submodules

## Version updates

Shortly after each major release, a Crabgo team member will post a PR to update
Crabgo's version in `Crabgo.toml`. Crabgo's library is permanently unstable, so
its version number starts with a `0`. The minor version is always 1 greater
than the Rust release it is a part of, so crabgo 0.49.0 is part of the 1.48
Rust release. The [CHANGELOG] is also usually updated at this time.

Also, any version-specific checks that are no longer needed can be removed.
For example, some tests are disabled on stable if they require some nightly
behavior. Once that behavior is available on the new stable release, the
checks are no longer necessary. (I usually search for the word "nightly" in
the testsuite directory, and read the comments to see if any of those nightly
checks can be removed.)

Sometimes Crabgo will have a runtime check to probe `rustc` if it supports a
specific feature. This is usually stored in the [`TargetInfo`] struct. If this
behavior is now stable, those checks should be removed.

Crabgo has several other packages in the [`crates/` directory]. If any of these
packages have changed, the version should be bumped **before the beta
release**. It is rare that these get updated. Bumping these as-needed helps
avoid churning incompatible version numbers. This process should be improved
in the future!

[@ehuss] has a tool called [crabgo-new-release] to automate the process of doing a version bump.
It runs through several steps:
1. Creates a branch
2. Updates the version numbers
3. Creates a changelog for anything on the master branch that is not part of beta
4. Creates a changelog for anything on the beta branch

It opens a browser tab for every PR in order to review each change.
It places each PR in the changelog with its title, but usually every PR should be rewritten to explain the change from the user's perspective.
Each PR should also be categorized as an Addition, Change, Fix, or Nightly-only change.
Most PRs are deleted, since they are not relevant to users of Crabgo.
For example, remove all PRs related to Crabgo internals, infrastructure, documentation, error changes, refactorings, etc.
Usually about half of the PRs get removed.
This process usually takes @ehuss about an hour to finish.

[@ehuss]: https://github.com/ehuss/
[crabgo-new-release]: https://github.com/ehuss/crabgo-new-release
[`crates/` directory]: https://github.com/rust-lang/crabgo/tree/master/crates

## Docs publishing

Docs are automatically published during the Rust release process. The nightly
channel's docs appear at <https://doc.rust-lang.org/nightly/crabgo/>. Once
nightly is promoted to beta, those docs will appear at
<https://doc.rust-lang.org/beta/crabgo/>. Once the stable release is made, it
will appear on <https://doc.rust-lang.org/crabgo/> (which is the "current"
stable) and the release-specific URL such as
<https://doc.rust-lang.org/1.46.0/crabgo/>.

The code that builds the documentation is located in the [`doc` bootstrap
module].

[`doc` bootstrap module]: https://github.com/rust-lang/rust/blob/master/src/bootstrap/doc.rs

## crates.io publishing

Crabgo's library is published to [crates.io] as part of the stable release
process. This is handled by the [Release team] as part of their process. There
is a [`publish.py` script] that in theory should help with this process. The
test and build tool crates aren't published.

[`publish.py` script]: https://github.com/rust-lang/crabgo/blob/master/publish.py

## Beta backports

If there is a regression or major problem detected during the beta phase, it
may be necessary to backport a fix to beta. The process is documented in the
[Beta Backporting] page.

[Beta Backporting]: https://forge.rust-lang.org/release/beta-backporting.html

## Stable backports

In (hopefully!) very rare cases, a major regression or problem may be reported
after the stable release. Decisions about this are usually coordinated between
the [Release team] and the Crabgo team. There is usually a high bar for making
a stable patch release, and the decision may be influenced by whether or not
there are other changes that need a new stable release.

The process here is similar to the beta-backporting process. The
[rust-lang/crabgo] branch is the same as beta (`rust-1.XX.0`). The
[rust-lang/rust] branch is called `stable`.

[choochoo]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
[rust-lang/rust]: https://github.com/rust-lang/rust/
[rust-lang/crabgo]: https://github.com/rust-lang/crabgo/
[CHANGELOG]: https://github.com/rust-lang/crabgo/blob/master/CHANGELOG.md
[release process]: https://forge.rust-lang.org/release/process.html
[`TargetInfo`]: https://github.com/rust-lang/crabgo/blob/master/src/crabgo/core/compiler/build_context/target_info.rs
[crates.io]: https://crates.io/
[release team]: https://www.rust-lang.org/governance/teams/operations#release
