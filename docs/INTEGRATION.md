# Integration

From hand-copied workflows to generated ones, without a red build in
between. Each step leaves the repository working.

## What you need

- `actlith` on your `PATH` (see [Install](../README.md#install)).
- `git`: actlith reads the list of tracked files from it.
- For the `gate` kind: a nix flake whose dev shell provides
  [hk](https://hk.jdx.dev) and an `hk.pkl`, because the generated job runs
  `nix develop --command hk check --all --check`. A different command goes
  in the row's `command` arg. [pklith](https://github.com/pr0d1r2/pklith)
  builds such a gate from your files.
- Optional: a [cachix](https://www.cachix.org) cache, and its auth token
  as the `CACHIX_AUTH_TOKEN` repository secret.

## A repository with no workflows yet

```sh
actlith detect        # print proposed rows; nothing is written
actlith seed --init   # write .actlith from them
actlith gen           # write .github/workflows/*.yml and scripts/actlith/*.sh
actlith check         # exit 0: every workflow file is accounted for
```

`detect` proposes a `gate` row when it finds `flake.nix`. When
`flake.lock` has a `nixpkgs-lock` input it also proposes `update-pins`,
commented out: that workflow pushes branches and opens pull requests on a
schedule, so it stays off until you uncomment it:

```text
# off by default: uncomment to get daily nixpkgs-lock bump PRs
# update-pins|update-pins|-|-|-
```

Edit `.actlith` before `gen` if you want different systems, a cache or
other args; [REGISTRY.md](REGISTRY.md) describes every column. Review the
generated files, commit them together with `.actlith`, and push.

The scripts under `scripts/actlith/` are generated too (the bump kinds run
them), with the same header. Commit them, and do not edit them: they live
as real files so shellcheck and your other linters see them.

## A repository that already has workflows

Roll out one repository at a time, starting with pklith, then
congestimancer, then the remaining byte-identical `ci.yml` groups. For each
repository, get user approval before opening or pushing a PR. The mechanical
sequence is:

1. Run `actlith detect` for repository-level `ci`/`gate` rows. For other
   existing workflows, derive the row from the workflow's path, kind and
   arguments.
2. Add or edit that row in `.actlith`.
3. Remove or rename replaced workflows, or list retained files under
   `## hand-written` with a reason for each file (`path|reason`).
4. Run `actlith gen`, then `actlith check` and review the complete PR diff.
5. Only after the diff is approved, open the repository PR.

Do not use `actlith import` as a fleet migration tool. It accepts only an
exact render (after comment and whitespace normalisation); the fleet sweep
found 3 matches out of 214 workflow files, with the misses differing in
shape. Use `## hand-written` for workflows that are intentionally retained.

For a single existing workflow, the exact-match diagnostic remains useful:

1. Ask whether a kind renders it already:

   ```sh
   actlith import .github/workflows/ci.yml
   ```

   It prints the row when a kind's render matches the file after comments
   and the generated header are set aside, or exits 1 naming the first key
   that differs.
2. If no kind matches, choose: add a row and accept the generated shape
   (diff the result before you commit it), or keep the file and list it
   under `## hand-written` with a reason.
3. Run `actlith gen` and `actlith check`. When both pass, commit.

Exact matches are rare in practice, because hand-copied workflows differ
in small ways. Expect to review a diff rather than to import silently.
Job names are stable across actlith versions, because branch protection
refers to them: after moving `ci.yml` to the `gate` kind, check that your
required status checks still match (`gate (ubuntu-latest)` and so on).

A fork that keeps upstream's workflows untouched, gates its own with
actionlint, zizmor and yamllint, and pins an Xcode on macOS is walked
through in [EXISTING-REPO.md](EXISTING-REPO.md), including a known gotcha
with `xcode=` and `nix develop`.

## Keeping it true

Add both checks to the gate that runs on every commit, so drift fails
where it starts:

```sh
actlith gen --check   # a generated file was edited by hand, or is stale
actlith check         # a workflow file nobody declared
```

With pklith, they are two `.pklith` rows; this repository's own
[`.pklith`](../.pklith) has them as `actlith-gen-check` and
`actlith-check`.

When you upgrade actlith, run `actlith gen` and review the diff. A change
to the bytes actlith writes for the same `.actlith` is at least a minor
version, and [CHANGELOG.md](../CHANGELOG.md) names the files that change.

## Pinning actions

The built-in pins move with actlith releases. To move them yourself:

```sh
actlith pin --dry-run   # print fresh `## actions` rows
actlith pin             # write them into .actlith
actlith gen
```

`pin` is the only command that reaches the network, through `gh api`.

## What actlith will not do to your repository

- Write anywhere but `.github/workflows/`, `scripts/actlith/`, the bot
  files your `## bots` rows name, and `.actlith` (only `seed --init`, and
  `pin` for its `## actions` section).
- Overwrite an existing `.actlith` (`seed --init` exits 1 instead).
- Rewrite a file whose bytes already match.
- Touch a workflow listed under `## hand-written`.
- Install GitHub Apps, change repository settings or read your secrets:
  those stay your actions. `actlith report` lists what your rows assume.
- Reach the network from any command but `pin`.

## Known gaps

- Four kinds in v1. A workflow shape gets a kind once two repositories
  need it; until then, list it under `## hand-written`.
- A pull request opened by `update-pins` or `upstream` with the default
  `GITHUB_TOKEN` does not start other workflows, so the gate does not run
  on it until someone pushes or re-runs it. The `token` arg names a secret
  holding a GitHub App or fine-grained token that lifts this.
- `x86_64-darwin` runs on `macos-15-intel`, which GitHub says is its last
  Intel image, available until August 2027 (actions/runner-images#13045).
  After that, a `## runners` row naming your own Intel Mac keeps the
  system in CI ([REGISTRY.md](REGISTRY.md#-runners)); otherwise drop it
  from the row's systems.
- Since pipelith 0.2, `-` in `systems` means all four systems, so a `gate`
  or `build` row written with `-` gains an Intel macOS job when you take
  this actlith. List the systems you want if you do not want that job.
