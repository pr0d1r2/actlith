# SPEC

## §G GOAL

`actlith` (cmd `actlith`) — GitHub Actions workflow MATERIALIZER: from small `.actlith` (1 row = 1 workflow: id, kind, systems, cache, args) RENDER full `.github/workflows/*.yml` + seeded scripts; ∀ action SHA-pinned; hardening ⊥ optional; CHECK committed files == render (drift) & ∀ workflow file declared (totality). purely mechanical: ⊥ LLM, ⊥ inference, ⊥ network (sole exception `actlith pin`, opt-in).

PAIR: pklith decides WHAT is checked (hk gate); actlith decides WHERE & WHEN CI runs it. actlith ⊥ adds lint steps.

WHY: fleet = 163 hand-copied `ci.yml`, copies drift (R1). variable part = few values ∴ shape from 1 place.

NAME: act(ions) + lith — fleet family `pklith`, `microlith`, `xenolith` (V22).

FIRST BUILT: this repo's own guardrails (§T T1–T12), 1 check per commit, hooks live from hk commit — before any product code.

SOURCES: `SPEC-PROMPT.md` (decisions D1–D23) & `docs/KINDS.md` (kind catalogue). this spec wins where they differ.
