# actlith report

## Workflows

| id | kind | systems | cache |
| --- | --- | --- | --- |
| `build` | `build` | `x86_64-linux` on `ubuntu-latest`, `aarch64-linux` on `ubuntu-24.04-arm`, `aarch64-darwin` on `macos-latest` | `pr0d1r2` |
| `ci` | `gate` | `x86_64-linux` on `ubuntu-latest`, `aarch64-linux` on `ubuntu-24.04-arm`, `aarch64-darwin` on `macos-latest` | `pr0d1r2` |
| `update-pins` | `update-pins` | `x86_64-linux` on `ubuntu-latest` | - |
| `upstream` | `upstream` | `x86_64-linux` on `ubuntu-latest` | - |

## Actions

| workflow | action | version | sha |
| --- | --- | --- | --- |
| `build` | `actions/checkout` | `v7.0.1` | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `build` | `cachix/cachix-action` | `v17` | `5f2d7c5294214f71b873db4b969586b980625e71` |
| `build` | `cachix/install-nix-action` | `v31.11.0` | `630ae543ea3a38a9a4166f03376c02c50f408342` |
| `ci` | `actions/checkout` | `v7.0.1` | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `ci` | `cachix/cachix-action` | `v17` | `5f2d7c5294214f71b873db4b969586b980625e71` |
| `ci` | `cachix/install-nix-action` | `v31.11.0` | `630ae543ea3a38a9a4166f03376c02c50f408342` |
| `update-pins` | `actions/checkout` | `v7.0.1` | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `update-pins` | `cachix/install-nix-action` | `v31.11.0` | `630ae543ea3a38a9a4166f03376c02c50f408342` |
| `upstream` | `actions/checkout` | `v7.0.1` | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `upstream` | `cachix/install-nix-action` | `v31.11.0` | `630ae543ea3a38a9a4166f03376c02c50f408342` |

## Secrets

| workflow | secret | purpose |
| --- | --- | --- |
| `build` | `CACHIX_AUTH_TOKEN` | cachix push |
| `ci` | `CACHIX_AUTH_TOKEN` | cachix push |
| `update-pins` | `GITHUB_TOKEN` | push the bump branch and open its PR |
| `upstream` | `GITHUB_TOKEN` | push the bump branch and open its PR |

## Hand-written

| path | reason |
| --- | --- |
| `.github/workflows/release.yml` | not a v1 kind yet |

## Bots

| bot | app | note |
| --- | --- | --- |
| `coderabbit` | CodeRabbit | install the app; free tier per docs/SERVICES.md |
| `greptile` | Greptile | install the app; free tier per docs/SERVICES.md |
| `socket` | Socket | install the app; free tier per docs/SERVICES.md |
