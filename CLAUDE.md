# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`astgen` is a Rust CLI that parses source files with pre-built [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammars and emits each file's AST as JSON (one JSON document per file, single-line by default). It walks files and directories in parallel and supports ~24 languages.

## Commands

```bash
cargo build                      # debug build
cargo build --release            # release build (binary at target/release/astgen)
cargo test                       # run all tests (unit tests live inline in each src/*.rs)
cargo test test_parse_file_rust_success   # run a single test by name
cargo test parsing::             # run all tests in a module
cargo clippy -- -D warnings      # lint — CI fails on any warning
cargo fmt --all -- --check       # formatting check (CI); use `cargo fmt` to fix
```

CI (`.github/workflows/ci.yaml`) runs `test`, `clippy -- -D warnings`, `fmt --check`, and a release build on every push/PR. Run all four locally before pushing — clippy-as-error is the most common CI break (see commit `c9d2379`).

## Releases

Releases are driven by `make-release.sh -b {patch|minor|major}`, which bumps the version and triggers `.github/workflows/release.yml` (built on `cargo dist`). The release workflow produces binaries and updates the `grahambrooks/homebrew-astgen` tap. Do not hand-edit release artifacts.

## Architecture

The pipeline is **CLI args → config → encodings (language registry) → walk → parse → JSON → output**. Data flows one direction; modules don't call back upward.

- **`main.rs`** — entry point. Parses `Args`, loads config, builds the global Rayon thread pool, constructs the encodings registry once, then dispatches each input path to `walk::process_single_file` or `walk::process_directory`. Exits non-zero if any file errored. The binary `VERSION` is `CARGO_PKG_VERSION` + git short SHA (the SHA is injected at build time — see below).

- **`languages.rs`** — the source of truth for supported languages. Each tree-sitter `Language` is held in a `OnceLock` singleton (lazily initialized, shared across threads). `create_encodings()` registers every language's extension regex → grammar mapping; `supported_languages()` / `print_supported_languages()` back `--list-languages`. **To add a language:** add the `tree-sitter-*` crate to `Cargo.toml`, then add a `OnceLock`, a `supported_languages()` entry, and a `create_encodings()` registration here.

- **`encoding.rs` / `encodings.rs`** — `Encoding` pairs a regex (matched against file extension, falling back to the full filename for extensionless files like `Dockerfile`) with a grammar and display name. `Encodings::match_file` returns the **first** matching encoding, so registration order in `create_encodings()` is significant for ambiguous extensions.

- **`walk.rs`** — file discovery and orchestration. Uses the `ignore` crate's `WalkBuilder` (respects `.gitignore` and a custom `.astgenignore`), applies include/exclude glob filters, then parses the collected files in parallel via `rayon::par_iter`. Also owns the `indicatif` progress bar and output writing (stdout, or append to `--output` file). Note: `glob_match` here is a hand-rolled single-`*` matcher, not full glob semantics.

- **`parsing.rs`** — `parse_file_safe_with_size_limit` enforces the max-size limit, reads the file (with a clear error for non-UTF-8 input), runs the tree-sitter parser, and wraps the result as `{ version, filename, language, ast }`. Optional `--truncate` cuts the JSON string at a `}` boundary.

- **`json.rs`** — `node_to_json` recursively converts a tree-sitter `Node` into the serializable `JsonNode { kind, start_byte, end_byte, children, text }`. `text` is only populated on **leaf** nodes (nodes with children carry no text, to avoid duplicating source).

- **`config.rs`** — optional TOML config (`--config <path>`, else `.astgenrc` in cwd then `$HOME`). Currently only `performance.max_threads` is wired into runtime behavior; other sections deserialize but are largely unused.

- **`cli_types.rs`** — `clap` `Args` struct and `OutputFormat` enum (`json` / `pretty-json` / `yaml`); `format_output` does the final serialization per format.

- **`error.rs`** — `AstgenError` enum and the crate-wide `Result<T>` alias.

### Build-time code generation (`build.rs`)

`build.rs` runs before compilation and generates two things into `OUT_DIR`:
- `version.txt` — the git short SHA, `include_str!`'d into `main.rs` for the version string.
- `versions_gen.rs` — parses `Cargo.toml`, extracts every `tree-sitter-*` dependency version, and emits `*_VERSION` constants plus the `TREE_SITTER_PARSERS` table. `src/versions.rs` re-exports these via `include!`.

Because of this, **tree-sitter parser versions shown to users come straight from `Cargo.toml`** — bumping a grammar dependency automatically updates `--list-languages` output. The build re-runs when `Cargo.toml` changes. If you reference a `TREE_SITTER_*_VERSION` constant, it must correspond to a dependency named in `Cargo.toml`.

## Conventions

- Unit tests are colocated in each module under `#[cfg(test)] mod tests`; there is no separate `tests/` integration directory. Tests use `tempfile` for filesystem fixtures.
- Errors propagate as `Result<T>` (= `Result<T, AstgenError>`); avoid `unwrap`/`expect` in non-test code paths (the `parsing.rs` tests note this as existing tech debt to not replicate).
