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
