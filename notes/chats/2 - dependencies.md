Micro-step 2: wire in the core crates we’ll use everywhere (CLI parsing, error handling, JSON I/O, Polars with minimal features) and confirm the project builds.

**Why this step**
- A production CLI needs a consistent error story (`anyhow`), a stable command interface (`clap`), and config/report serialization (`serde`/`serde_json`).
- Polars is feature-gated; choosing a minimal feature set now keeps compile times reasonable.

**Do this (from data-quality-gate)**

1) Add baseline dependencies
- `cargo add clap --features derive`
- `cargo add anyhow`
- `cargo add serde --features derive`
- `cargo add serde_json`

2) Add Polars (start minimal; we’ll expand later only if needed)
- `cargo add polars --features "lazy csv parquet"`

Notes:
- `lazy` is important because we’ll build most checks as lazy expressions.
- `csv` + `parquet` gives us the two most common ingestion formats.

3) Sanity build
- `cargo build`

**Checkpoint (reply with output snippets if anything fails)**
- Paste your updated Cargo.toml `[dependencies]` section (just that part).
- Confirm `cargo build` succeeded (or paste the first error block).

Next micro-step (after build is green): define the CLI shape with `clap`:
- `data-quality-gate check <INPUT> --spec <SPEC> --out <DIR> --format json|html`
…and we’ll implement a stub handler that loads a file into a Polars `LazyFrame` without doing validations yet.