# voxeltron-plugin-api

Stable public plugin API for Voxeltron.

## Status

This repository is part of the April 17, 2026 Voxeltron org-restructure plan.
The canonical plugin API still lives in the main public repo during the transition:

- upstream path: `crates/voxeltron-plugin-api/`
- upstream repo: `deep-thinking-llc/voxeltron`

This repo exists so the public Apache-2.0 plugin surface has a dedicated home once the extraction is completed.

## Planned Contents

- stable trait definitions for runtimes, middleware, audit sinks, registry policy, auth providers, and config loading
- semver-checked releases for downstream plugin authors
- public release notes and migration guidance for API consumers

## License

Apache-2.0. See `LICENSE`.
