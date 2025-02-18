# Process

This chapter gives an overview of how Crabgo comes together, and how you can be
a part of that process.

See the [Working on Crabgo] chapter for an overview of the contribution
process.

Please read the guidelines below before working on an issue or new feature.

**Due to limited review capacity, the Crabgo team is not accepting new features
or major changes at this time. Please consult with the team before opening a
new PR. Only issues that have been explicitly marked as accepted will be
reviewed.**

[Working on Crabgo]: working-on-crabgo.md

## Crabgo team

Crabgo is managed by a [team] of volunteers. The Crabgo Team reviews all
changes, and sets the direction for the project.

The team meets on a weekly basis on a video chat. If you are interested in
participating, feel free to contact us on [Zulip].

If you would like more direct mentorship, you can join our
[office hours](https://github.com/rust-lang/crabgo/wiki/Office-Hours).

## Roadmap

The [Roadmap Project Board] is used for tracking major initiatives. This gives
an overview of the things the team is interested in and thinking about.

The [RFC Project Board] is used for tracking [RFCs].

[the 2020 roadmap]: https://blog.rust-lang.org/inside-rust/2020/01/10/crabgo-in-2020.html
[Roadmap Project Board]: https://github.com/rust-lang/crabgo/projects/1
[RFC Project Board]: https://github.com/rust-lang/crabgo/projects/2
[RFCs]: https://github.com/rust-lang/rfcs/

## Working on issues

Issues labeled with the [S-accepted] [label] are typically issues that the
Crabgo team wants to see addressed. If you are interested in one of those, and
it has not already been assigned to someone, leave a comment. See [Issue
assignment](#issue-assignment) below for assigning yourself.

When possible, the Crabgo team will try to also include [E-easy], [E-medium],
or [E-hard] labels to try to give an estimate of the difficulty involved with
the issue.

If there is a specific issue that you are interested in, but it is not marked
as [S-accepted], leave a comment on the issue. If a Crabgo team member has the
time to help out, they will respond to help with the next steps.

[E-easy]: https://github.com/rust-lang/crabgo/labels/E-easy
[E-medium]: https://github.com/rust-lang/crabgo/labels/E-medium
[E-hard]: https://github.com/rust-lang/crabgo/labels/E-hard
[S-accepted]: https://github.com/rust-lang/crabgo/labels/S-accepted
[label]: ../issues.md#issue-labels

## Working on small features

Small feature requests are typically managed on the [issue
tracker][issue-feature-request]. Features that the Crabgo team have approved
will have the [S-accepted] label.

If there is a feature request that you are interested in, but it is not marked
as [S-accepted], feel free to leave a comment expressing your interest. If a
Crabgo team member has the time to help out, they will respond to help with the
next steps. Keep in mind that the Crabgo team has limited time, and may not be
able to help with every feature request. Most of them require some design
work, which can be difficult. Check out the [design principles chapter] for
some guidance.

## Working on large features

Crabgo follows the Rust model of evolution. Major features usually go through
an [RFC process]. Therefore, before opening a feature request issue create a
Pre-RFC thread on the [internals][irlo] forum to get preliminary feedback.

Implementing a feature as a [custom subcommand][subcommands] is encouraged as
it helps demonstrate the demand for the functionality and is a great way to
deliver a working solution faster as it can iterate outside of Crabgo's release
cadence.

See the [unstable chapter] for how new major features are typically
implemented.

[unstable chapter]: unstable.md

## Bots and infrastructure

The Crabgo project uses several bots:

* [GitHub Actions] are used to automatically run all tests for each PR.
* [triagebot] automatically assigns reviewers for PRs, see [Assignment] for
  how to configure.
* [bors] is used to merge PRs. See [The merging process].
* [triagebot] is used for assigning issues to non-members, see [Issue
  assignment](#issue-assignment).
* [rfcbot] is used for making asynchronous decisions by team members.

[bors]: https://buildbot2.rust-lang.org/homu/
[The merging process]: working-on-crabgo.md#the-merging-process
[GitHub Actions]: https://github.com/features/actions
[triagebot]: https://github.com/rust-lang/triagebot/wiki
[rfcbot]: https://github.com/rust-lang/rfcbot-rs
[Assignment]: https://github.com/rust-lang/triagebot/wiki/Assignment

## Issue assignment

Normally, if you plan to work on an issue that has been marked with the
[S-accepted] label, it is sufficient just to leave a comment that you are
working on it. We also have a bot that allows you to formally claim an issue
by entering the text `@rustbot claim` in a comment. See the [Assignment] docs
on how this works.


[Assignment]: https://github.com/rust-lang/triagebot/wiki/Assignment
[team]: https://www.rust-lang.org/governance/teams/dev-tools#crabgo
[Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/246057-t-crabgo
[issue-feature-request]: https://github.com/rust-lang/crabgo/labels/C-feature-request
[Feature accepted]: https://github.com/rust-lang/crabgo/labels/Feature%20accepted
[design principles chapter]: ../design.md
[RFC process]: https://github.com/rust-lang/rfcs/
[irlo]: https://internals.rust-lang.org/
[subcommands]: https://doc.rust-lang.org/crabgo/reference/external-tools.html#custom-subcommands
