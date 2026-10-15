# Changelog

All notable changes to actlith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes actlith emits for the same inputs is at least a minor version
(SPEC.md V18), and its entry names the files consumers will see change.

## [Unreleased]

### Added

- `src/yaml`: typed workflow model and own YAML writer with fixed key
  order and quoting, `|` and `>-` blocks and the generated header; no
  file is emitted yet (T16).
