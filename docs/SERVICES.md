# Services

Catalogue of third-party services actlith can wire into public repositories:
review bots, security scanners, dependency and release tooling, caches and
runners. Read it with `docs/KINDS.md`. Prices and free-tier terms were checked
on each vendor's own page on 2026-10-15; "unverified" marks what could not be
confirmed there.

The rule (`src/bots` SPEC, cost rule): use a free tier wherever one exists.
Paid plans are recorded as potential, at their smallest price, and used only
when the user opts in. Installing a GitHub App, applying to an open-source
programme or paying is always the user's action; actlith only writes files.

## Adopt (free for public repositories)

| Service | What it does | Free terms | actlith writes | Source |
| --- | --- | --- | --- | --- |
| CodeRabbit | AI PR review | "free reviews forever for public repositories" | `.coderabbit.yaml` stub with `remote_config` to a central repo | <https://www.coderabbit.ai/pricing> |
| Socket | dependency supply-chain alerts | "free to use for open-source"; free plan 1,000 scans/month | `socket.yml` (optional) | <https://socket.dev/pricing> |
| Greptile | AI PR review | Starter free: 1 developer, 50 credits/month; non-commercial OSS free by application | `greptile.json` | <https://www.greptile.com/pricing> |
| Sourcery | AI PR review | Open Source plan "Free forever", public repos | nothing (app-only) | <https://www.sourcery.ai/pricing> |
| cubic | AI PR review | "free for public repositories", unlimited | nothing (app-only) | <https://www.cubic.dev/pricing-plans> |
| Korbit | AI PR review | "Korbit Max is free for all open source repositories" | nothing (app-only) | <https://www.korbit.ai/> |
| CodeQL | static analysis, Rust and GitHub Actions | free on all public repositories | `codeql.yml` (languages `rust`, `actions`) or default setup | <https://github.com/security/plans> |
| Secret scanning, push protection | server-side secret detection | free on public repos; repo push protection off by default | a settings/API step, no file | <https://docs.github.com/en/code-security/secret-scanning/introduction/about-push-protection> |
| Dependabot | version and security update PRs | free; `cargo` (version + security), `nix` (version only), `github-actions` | `.github/dependabot.yml` | <https://docs.github.com/en/code-security/dependabot/ecosystems-supported-by-dependabot/supported-ecosystems-and-repositories> |
| Renovate | update PRs, incl. `flake.lock` (nix manager, opt-in beta) | app "free to install for both public and private repositories" | `renovate.json` (alternative to Dependabot; pick one) | <https://docs.renovatebot.com/modules/manager/nix/> |
| OpenSSF Scorecard | repo security posture, badge | "free for all public repositories" | `scorecard.yml` | <https://github.com/ossf/scorecard-action> |
| Harden-Runner | CI egress monitoring | "free for public repositories on GitHub-hosted runners" | first step of every generated job, `egress-policy: audit` | <https://github.com/step-security/harden-runner> |
| Artifact attestations | build provenance (SLSA L2/L3) | public repos use the Sigstore public-good instance | `actions/attest` step in release jobs | <https://docs.github.com/en/actions/concepts/security/artifact-attestations> |
| crates.io trusted publishing | publish without a token (OIDC) | free; first publish manual | publish job with `id-token: write` | <https://crates.io/docs/trusted-publishing> |
| release-plz | release PRs, changelog, publish | free open source | `release-plz.yml` workflow | <https://release-plz.dev/docs/github/quickstart> |
| GitHub merge queue | serialised merges | free with GitHub | `merge_group:` trigger on CI workflows | <https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue> |
| Mergify | merge queue and rules as code | "Free for Open Source projects", public repos only | `.mergify.yml` (alternative to native merge queue) | <https://github.com/marketplace/mergify> |
| Cachix | Nix binary cache | "5 GB limit for open source projects" | push/pull steps (already used) | <https://www.cachix.org/pricing> |
| cache-nix-action | Nix store in the GitHub Actions cache | free, MIT | cache step in Nix jobs | <https://github.com/nix-community/cache-nix-action> |
| flake-checker-action | stale or forked nixpkgs input | free, MIT | flake-health step, `send-statistics: false` | <https://github.com/DeterminateSystems/flake-checker-action> |
| Codecov | coverage reports | free for public repos (tokenless for public, unprotected branch) | coverage upload + `codecov.yml` | <https://docs.codecov.com/docs/codecov-tokens> |
| CodSpeed | continuous benchmarks | "Unlimited for open-source projects" | bench workflow | <https://codspeed.io/pricing> |
| Bencher | continuous benchmarks | "Free forever for open source", public only | bench workflow (alternative to CodSpeed) | <https://bencher.dev/pricing/> |
| Semantic PR title | conventional title check | free action | `pr-title.yml` | <https://github.com/amannn/action-semantic-pull-request> |
| actions/stale | stale issue and PR sweep | free action | `stale.yml` | <https://github.com/actions/stale> |
| lychee-action | link check on a schedule | free action | `links.yml` | <https://github.com/lycheeverse/lychee-action> |

## Potential (paid, smallest plan, only on opt-in)

| Service | Smallest paid | When it would help |
| --- | --- | --- |
| Cursor Bugbot | Cursor Premium $30/month | strong review findings; no free tier |
| CodeRabbit Essentials | $24/user/month annual | if free public reviews run short |
| Greptile Pro | $30/seat/month, $1 per extra credit | if Starter credits run short |
| Socket Team | $25/developer/month, minimum 5 | private repos or more scans |
| Sourcery Pro | $12/developer/month annual | private repos |
| Korbit Pro | $12/user/month annual | private repos |
| cubic Team | $30/developer/month annual | private repos |
| FlakeHub Cache | $7/month individual | private Nix cache |
| Garnix | $25/month individual (free 1,500 min/month) | Nix-native CI beyond the free minutes |
| Blacksmith, Ubicloud runners | usage-billed after free minutes | faster runners |

## Skip

- Entire: no published pricing; its gate failed on hk #1705.
- Copilot code review: not in Copilot Free; free Pro only for maintainers GitHub selects.
- Gemini Code Assist on GitHub: consumer app discontinued, enterprise only.
- Snyk: Rust only via SBOM. Aikido (10 repos), FOSSA (5 projects): caps too low.
- GitGuardian, OSV-Scanner, Trivy, Mend Bolt: overlap ripsecrets or cargo deny in the hk gate.
- SLSA generator: unmaintained; use artifact attestations.
- BuildJet: shut down 2026-03-31; never emit `buildjet-*` labels.
- magic-nix-cache: throttled; use cache-nix-action or Cachix.
