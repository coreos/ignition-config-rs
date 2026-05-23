@AGENTS.md

## Claude Code Specific Workflows

### Testing

Before committing, always run the full validation suite:

```bash
cargo build --all-targets && cargo test --all-targets && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check -l
```

### Agent Usage

- Use subagents for broad codebase searches when simple Grep/Glob isn't enough
- Delegate independent research to subagents to keep main context clean

### Key Reminders

- Never edit `src/v3_*/schema.rs` — these are generated files
- Every PR must touch `docs/release-notes.md` (CI enforced)
- Use `component: description` commit format, not conventional commits
