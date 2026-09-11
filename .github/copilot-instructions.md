# AI assistant guidelines — ci-foundation-scratch

Canonical process, CI tiers, runner policy and release chain:

https://github.com/EdgeFirstAI/.github/blob/main/.github/copilot-instructions.md

This repository is a CI proof crate only.

## Project-specific

### Layout

- `src/lib.rs` — dummy `ping()` plus a unit test

### Tests

- Unit: `cargo nextest run --workspace --locked`
