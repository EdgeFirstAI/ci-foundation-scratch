# ci-foundation-scratch

Throwaway crate that proves EdgeFirstAI shared CI (`EDGEAI-1553` / T0).

Callers pin `EdgeFirstAI/.github` at `2c47ac832e402d573aba404df1013a6b2876dc46`
and pass the same value as `shared-sha`.

`tag-release.yml` and `release.yml` are also pinned there. The release job is
`dry-run: true` (no crates.io publish, no GitHub Release). Merge a `release/X.Y.Z`
PR to `main` to prove annotated tagging, then the `v*` tag starts the dry-run
release chain. The org secret `RELEASE_TAG_TOKEN` must be available to this
repository.
