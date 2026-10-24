# CI in an existing repository

A worked example of materializing a repository's guardrails in CI with
actlith, in a repository that already has workflows it must not rewrite.
The example is
[age-plugin-se PR #9](https://github.com/pr0d1r2/age-plugin-se/pull/9),
a fork of an upstream age plugin written in Swift, merged on 2026-10-18 as
`e9e7a34` (its last commit `844d789`), with `gate (macos-latest)` green.
Its local gate was built earlier with pklith (see pklith's
[worked example](https://github.com/pr0d1r2/pklith/blob/main/docs/EXISTING-REPO.md));
this PR runs that same gate on GitHub Actions.

[INTEGRATION.md](INTEGRATION.md) is the general path; this page is about
what a fork with a pinned Xcode adds to it.

## One gate row, upstream workflows left alone

`.actlith` holds one row:

```text
## workflows
id|kind|systems|cache|args
ci|gate|aarch64-darwin|pr0d1r2|xcode=26.6, timeout=60, command=HOST_DEVELOPER_DIR="$DEVELOPER_DIR" nix develop --command hk check --all --check
```

- `aarch64-darwin` only, which actlith maps to `macos-latest`: the Swift
  toolchain and XCTest come from Xcode, which nix does not supply.
  Because the row names its system, it does not pick up the
  `x86_64-darwin` job (on `macos-15-intel`) that `-` now brings.
- `xcode=26.6` pins that Xcode instead of the image's default
  ([KINDS.md](KINDS.md), src/gate V52). The job sets `DEVELOPER_DIR`, and
  its first step fails naming the path if the image does not have it.
- The command is the hooks' own gate, `hk check --all --check`, so CI and
  the commit hooks run one definition. The `HOST_DEVELOPER_DIR` prefix is
  a workaround, explained [below](#known-gotcha-nix-develop-drops-the-xcode-pin).

`actlith gen` renders `.github/workflows/ci.yml`, with the job `gate`, so
the check is called `gate (macos-latest)`.

The upstream workflows (`build.yml`, `build-main.yml`, `release.yml`) and
its composite action stay byte for byte as upstream wrote them. They are
listed under `## hand-written` with a reason each, so `actlith check`
accounts for every file in `.github/workflows/` without actlith touching
them. Rewriting them would make every upstream merge a conflict.

## The gate holds the workflow, one check per commit

Each check joins the gate in its own commit, whose body records the
planted violation it refused:

| check | what it runs | planted violation |
| --- | --- | --- |
| `actlith-check` | `actlith gen --check && actlith check` | `ci.yml` hand-edited to `timeout-minutes: 90` (stale); an undeclared `stray.yml` |
| `actionlint` | actionlint on the fork's workflow | `matrix.oss` for `matrix.os` |
| `zizmor` | zizmor, offline | `run: echo "${{ github.event.pull_request.title }}"` (template injection) |
| `yamllint` | yamllint with `.yamllint.yaml` as actlith's | a second `jobs:` key |

Before this PR the repository exempted `.github/**` whole, because it was
all upstream's. The actionlint commit narrowed that to the upstream files
only, in `.pklith` and in `hk.pkl`, so the fork's own `ci.yml` is read by
every check while upstream's files are not.

The yamllint config extends the default rules the way actlith's own does:
`on:` is a key, not a boolean (truthy `check-keys` off), and a SHA-pinned
action with its version comment runs past 80 columns (line length 120).

## Known gotcha: nix develop drops the Xcode pin

With the default command, the pin does not reach the build. The job env
sets `DEVELOPER_DIR` to the pinned Xcode, and the first step checks it.
But on macOS `nix develop` sets `DEVELOPER_DIR` to the stdenv's apple-sdk
before the dev shell's hook runs. A hook that resets it to the host Xcode
then finds a `/nix/store` path, falls back to `xcode-select -p`, and gets
the image's default Xcode. The job passes on an Xcode nobody chose, and
nothing says so.

The workaround has two halves:

1. The row's command hands the pin over under a name the stdenv does not
   touch: `HOST_DEVELOPER_DIR="$DEVELOPER_DIR" nix develop --command ...`.
   The shell expands it when the step runs, so no `${{ }}` reaches `run:`.
2. The dev shell's hook uses `HOST_DEVELOPER_DIR` first and falls back to
   `xcode-select -p` only when it is unset:

   ```sh
   case "${DEVELOPER_DIR:-}" in
   "" | /nix/store/*)
     DEVELOPER_DIR=${HOST_DEVELOPER_DIR:-$(/usr/bin/env -u DEVELOPER_DIR /usr/bin/xcode-select -p)}
     ;;
   esac
   export DEVELOPER_DIR
   ```

To prove the hook reads the handed-over Xcode, the PR planted a path that
does not exist: `HOST_DEVELOPER_DIR=/nonexistent nix develop -c true`
exits 1 because the toolchain guard cannot read a Swift version.

actlith records this as `src/gate` B11. Making the pin reach the dev
shell without a consumer workaround is `src/gate` T71, still open.

## Steps only the owner can take

actlith does not change repository settings. On this fork they were:

1. **Enable Actions.** A fork starts with Actions off, and only the
   Actions tab turns it on; there is no API for it. Once on, the upstream
   workflows run too: `build.yml` on every push and pull request, and
   `build-main.yml` and `release.yml` on `main` and on releases. Check
   what they need (a packaging secret, GitHub Pages) or accept their
   failures. Here upstream's Alpine packaging job reads an `ALPINE_KEY`
   secret the fork lacked. A CI-only key fills it:
   `release/alpine-keygen.sh` makes a throwaway key in memory, writes no
   file, and sets the secret with `gh secret set` (it does nothing if one
   is set; `--rotate` replaces it).
2. **Optional: `CACHIX_AUTH_TOKEN`**, so the job pushes to the cache.
   Without it the job only reads.
3. **Require the check** after a green run. When `main` has no required
   status checks yet, the per-check endpoint returns 404, so the first
   one needs a full branch-protection `PUT` that restates every current
   setting and adds `"checks": [{"context": "gate (macos-latest)"}]`
   under `required_status_checks`. The PR body has the exact payload.

## Pinning actlith until a tag has the arg

`xcode=` landed in actlith PR #19, which no tag carries yet, so the fork
adds actlith as a flake input on `ref=main`:

```nix
actlith.url = "git+https://github.com/pr0d1r2/actlith?ref=main&shallow=1";
```

Its fleet tools follow the fork's own inputs, and its dev-shell-only tools
follow `""`, so the lock keeps one nixpkgs. Move the input to a tag once
one carries `xcode=`.
