# ci-foundation-scratch

Throwaway crate that proves EdgeFirstAI shared CI (`EDGEAI-1553` / T0).

Callers pin `EdgeFirstAI/.github` at `f7c16cba` until the checkout-fix PR
merges; then pin the merged SHA (`v1.0.0` workflow tree was `ca1dd55` but
cannot fetch composites).
