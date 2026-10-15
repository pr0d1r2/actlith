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
