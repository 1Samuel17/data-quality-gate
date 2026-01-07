Great—Project 1 it is. Micro-step 1 is: scaffold a clean Rust CLI repo + folders (including `notes/` for our transcripts), and pin the toolchain so builds are reproducible.

**Micro-step 1 — Scaffold the project**
- **What/why**: We want a standard Rust binary crate that can grow into a production CLI (clear structure, reproducible builds, easy CI).
- **Do this (in your terminal, from Rust)**:

1) Create the repo + Cargo project
- Pick a name (I’ll use `dq-gate` unless you prefer another).
- Run:
  - `mkdir dq-gate && cd dq-gate`
  - `git init`
  - `cargo init --bin`

2) Add the “project hygiene” folders we’ll use from day 1
- `mkdir -p notes/chats data/inbox data/outbox`

3) Pin a Rust toolchain (recommended for teams/CI)
- Create `rust-toolchain.toml` with:
  - channel = `"stable"`
  - profile = `"default"`
  - components = `["rustfmt", "clippy"]`

4) First commit
- `git add -A`
- `git commit -m "chore: scaffold dq-gate CLI project"`

**Checkpoint (reply with this so I can guide the next micro-step precisely)**
- The project name you chose (`dq-gate` or other)
- Your `rustc --version`
- Confirm you now have: `Cargo.toml`, `src/main.rs`, and folders `notes/`, `data/inbox/`, `data/outbox/`

Next micro-step after that: add minimal dependencies (`clap`, `anyhow`, `polars`) and define the first CLI command shape (`dq-gate check <input> --spec <spec>`).