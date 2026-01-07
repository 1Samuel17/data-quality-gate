Cool—pinning Polars to `0.51.0` is a good call for now. We can come back to `0.52.0` once we see the exact error and decide whether it’s a feature change, MSRV bump, or a dependency conflict.

**Micro-step 3 — Define the CLI contract (no Polars work yet)**
Goal: make `data-quality-gate check ...` parse correctly and route into a handler function.

1) Shape the CLI types in main.rs
- **What**: Add a `Cli` struct (top-level) and a `Commands` enum (subcommands) using `clap` derive.
- **Why**: This becomes your stable public interface; everything else is internal implementation.

Design (implement this structure yourself):
- `data-quality-gate check <INPUT>`
- Flags:
  - `--spec <SPEC>` (required) — path to a rules/spec file (we’ll define JSON in the next step)
  - `--out <OUT_DIR>` (optional, default outbox)
  - `--format <json|html>` (optional, default `json`)
  - `--fail-on-warn` (optional bool, default false)
  - `--sample <N>` (optional; later used to bound work on huge files)

2) Add a stub handler
- **What**: When `check` runs, just print (or log) the parsed args and exit `Ok(())`.
- **Why**: Confirms the CLI works before we add file I/O and Polars.

3) Quick verification commands
- `cargo run -- check data/inbox/example.csv --spec specs/basic.json`
  - This should parse arguments and run your stub (it will fail if the file doesn’t exist only if you check existence—don’t yet).
- Also verify help output:
  - `cargo run -- --help`
  - `cargo run -- check --help`

**Checkpoint**
- Paste the output of `cargo run -- --help` and `cargo run -- check --help`.
- Also: paste the exact error you got on Polars `0.52.0` (first 30–50 lines is enough). That will let me tell you whether we should try upgrading again soon or keep `0.51.0` pinned.

Next micro-step after CLI is solid: create a tiny `specs/basic.json` + parse it with `serde` (still no Polars), then we’ll add “load input into LazyFrame” for CSV/Parquet.