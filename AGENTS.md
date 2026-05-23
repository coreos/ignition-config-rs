# ignition-config-rs

Rust library providing data structures for creating [Ignition](https://coreos.github.io/ignition/) configs and serializing them to JSON. Supports Ignition spec versions 3.0 through 3.6.

## Tech Stack

- **Language**: Rust (edition 2021, MSRV 1.85.0)
- **Dependencies**: serde, serde_json, semver, thiserror
- **Build**: Cargo (single crate, no workspace)
- **Testing**: `cargo test` (built-in test framework)
- **Code Generation**: `schemafy_lib` generates `schema.rs` from `ignition.json` (behind `regenerate` feature)

## Architecture

```
src/
  lib.rs              # Config enum, version dispatch, parsing, tests
  v3_0/ .. v3_6/      # Per-spec-version modules, each containing:
    ignition.json      #   Ignition JSON Schema (source of truth)
    schema.rs          #   GENERATED — do not edit manually
    mod.rs             #   VERSION constant, Default impls
build.rs              # Regenerates schema.rs from ignition.json (feature = "regenerate")
docs/release-notes.md # Release notes (must be updated with every PR)
```

## Important Rules

- **`src/v3_*/schema.rs` files are generated** — never edit directly. Modify `ignition.json` and run `cargo build --features regenerate`
- **Every PR must update `docs/release-notes.md`** — enforced by CI (`.github/workflows/require-release-note.yml`)
- **All source files require the Apache 2.0 license header**
- Adding a new spec version requires two commits: `Copy v3_X spec from v3_Y` then `Update for 3.X.0 spec`

## Build Commands

- `cargo build --all-targets` — Build
- `cargo test --all-targets` — Run tests
- `cargo clippy --all-targets -- -D warnings` — Lint (warnings are errors)
- `cargo fmt -- --check -l` — Check formatting
- `cargo build --all-targets --features regenerate` — Build with schema regeneration

## Code Style

- Standard `rustfmt` defaults (4-space indent)
- `clippy` with `-D warnings` (all warnings are errors)
- `#[non_exhaustive]` on public enums and error types
- `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants
- Apache 2.0 license header on all `.rs` files

## Testing

- **Pre-commit**: `cargo build --all-targets && cargo test --all-targets && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check -l`
- CI also tests with MSRV, beta, and nightly toolchains
- After `regenerate` builds, CI verifies no generated files changed unexpectedly

## Commit Conventions

**Format**: `component: description` (lowercase, imperative mood)

Examples from history:
- `docs/release-notes: update for release 0.6.1`
- `Pin tempfile to maintain MSRV compatibility`
- `Update for 3.6.0 spec`
- `Copy v3_6 spec from v3_5`
- `packit: fix rust2rpm command to use local source`
- `cargo: ignition-config release 0.6.0`

**Style**: Imperative tense, lowercase start, no period. Component prefix when applicable (e.g., `docs/release-notes:`, `packit:`, `cargo:`). No conventional commit types (`feat:`, `fix:`).

## Release Process

Releases follow a checklist in `.github/ISSUE_TEMPLATE/release-checklist.md`:
1. Pre-release PR updates `docs/release-notes.md`
2. Release branch uses `cargo release` (signed commits/tags)
3. Publish to crates.io

## Resources

- [crates.io](https://crates.io/crates/ignition-config) | [docs.rs](https://docs.rs/ignition-config/latest/ignition_config/)
- [Ignition specification](https://coreos.github.io/ignition/)
- [Release checklist](.github/ISSUE_TEMPLATE/release-checklist.md)
