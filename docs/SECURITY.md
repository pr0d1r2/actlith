# Security policy

## Reporting a vulnerability

Please report vulnerabilities privately, through GitHub's
[private vulnerability reporting](https://github.com/pr0d1r2/actlith/security/advisories/new),
not in a public issue. Include what you ran, what happened, and what you
expected.

You will get an acknowledgement within a week. A fix, or a reason there
will not be one, follows as soon as the issue is understood.

## Supported versions

actlith has no release yet. Fixes land on `main`; once releases exist,
the latest one is supported.

## What actlith trusts, and what it promises

actlith writes CI configuration, so what it generates runs with your
repository's permissions. Treat `.actlith` as code, the same as the
workflows it replaces:

- A `gate` row's `command` arg and an `upstream` row's `script` arg become
  commands the generated jobs run.
- A `token` arg names a secret the bump kinds use to push a branch and
  open a pull request.

In return, every workflow actlith generates is hardened (see
[REGISTRY.md](REGISTRY.md) and [KINDS.md](KINDS.md)): read-only
`contents` permission unless a job needs more, every action pinned to a
commit SHA, no persisted checkout credentials unless the job pushes, and no
`${{ }}` expression inside a `run:` script. Each kind's output passes
`actionlint` and `zizmor` with no findings in actlith's own tests.

Only `actlith pin` reaches the network, through `gh api`, and only when
you run it. In scope: a generated workflow that runs something its row
did not name, an expression that reaches a shell, a permission wider than
its job needs, an action left unpinned, or any other command that touches
the network.
