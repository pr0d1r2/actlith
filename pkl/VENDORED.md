# Vendored: hk configuration schema

`Config.pkl` is copied verbatim from the hk 2.8.1 release package, so
`hk.pkl` evaluates with no network access. The bytes are the ones pklith
vendors.

- source: <https://github.com/jdx/hk/releases/download/v2.8.1/hk@2.8.1.zip>
- package sha256: 037effad08b7a657c31eff76bd3b3593cb8e4cc282a4261102b2543d47ecebb5
- `Config.pkl` sha256: dd52dc82e17e753e7db7caa9c63a74e619c2a771faf90ab1bca673e3f915c72e
- license: MIT, (c) jdx and hk contributors

Do not edit it. When the hk pin moves, replace it from the matching
release in the same commit; `schema-pin` (nix T58) fails until this file
names the pinned hk version and the copy's sha256.
