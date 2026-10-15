actlith

Tool that materializes GitHub Actions workflows from a small config.

pklith derives hk git hook guardrails from what a repository's files require.
actlith does the same for CI: a short declaration in the repository (which
systems, which gate, which caches, which schedules) becomes the full
`.github/workflows/*.yml` files, rendered deterministically, with every action
pinned and every hardening default applied.

Goals:
- one small config per repository instead of hand-copied workflow YAML
- the same workflow shape across the fleet, so a fix lands everywhere by a
  regeneration, not by editing many repositories by hand
- check that the committed workflows still match the config (drift gate)

Write in Rust.
Use atomic git commits per topic for human readability.
Keep the closest possible parallel with pklith: same command shape, same
config file style, same guardrails, same conventions. Where pklith already
decided something, reuse the decision unless this file says otherwise.

# Naming

- Project / crate name: `actlith` (act(ions) + lith, stone): fleet family
  `pklith`, `microlith`, `xenolith`.
- Binary name: `actlith`, the same as the crate, as in pklith: `cargo install
  actlith` gives the command it names, and there is no second crates.io name.
- Availability checked 2026-10-15: `actlith` free on crates.io, npm, PyPI,
  RubyGems, Homebrew core; no GitHub user/org `actlith`; no GitHub repository
  with that name.
- Rejected alternatives:
  - `actionlint`: taken by rhysd/actionlint, the workflow linter that actlith's
    own gate and pklith's catalog already run on the generated files.
  - `actionlith`: free, but one letter away from `actionlint`; easy to confuse.
  - `workflith`: free, but longer to type.
  - `ghalith`: GitHub user `ghalith` exists.
  - `runlith`: taken on npm; GitHub user `runlith` exists.
- Repository directory is still `actionlint`; rename it to `actlith` before the
  first push.

# Fleet survey (2026-10-15)

Local checkouts next to this repository, read-only:
- 170 repositories have `.github/workflows/`; 163 of them have a `ci.yml`.
- Other workflow kinds: `update-pins.yml` (11), `release.yml` (3),
  `build.yml` (3), `upstream.yml` (2), then one-offs.
- Many `ci.yml` files are byte-identical copies (groups of 28, 22, 19, 12 and
  12), yet copies of the "same" workflow drift: pklith's and congestimancer's
  `ci.yml` differ in comment, job name, `permissions` place, `timeout-minutes`
  and comment spacing after SHA pins.
- Older copies pin actions by tag (`actions/checkout@v4`,
  `cachix/install-nix-action@v27`), which zizmor flags; current ones pin by
  SHA with a version comment. Upgrading means editing every copy.
- Current reference shape (pklith `ci.yml`): push to `main`, `pull_request`,
  `workflow_dispatch`; `concurrency` per ref with cancel; `permissions:
  contents: read`; matrix `ubuntu-latest`, `ubuntu-24.04-arm`, `macos-latest`;
  checkout with `persist-credentials: false`; install-nix with the
  `pr0d1r2.cachix.org` substituter; cachix push except for fork PRs; one step
  `nix develop --command hk check --all --check`.

Conclusion: the variable part of a fleet workflow is a handful of values. The
rest is shape, and shape should come from one place.
