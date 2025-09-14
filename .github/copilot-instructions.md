# GitHub Copilot Instructions for astgen

## Project Overview
This project is a Rust-based CLI tool for generating ASTs (Abstract Syntax Trees) from source code files in various programming languages. It supports Rust, JavaScript, Python, Java, Go, TypeScript, and Ruby, and can process both individual files and directories. The CLI provides options for truncating output, displaying help/version info, and more.

## Coding Style
- **Language:** Rust (main), with some test and script files.
- **Warnings:** ` cargo clippy -- -D warnings` must pass without warnings.
- **Formatting:** Use `cargo fmt` for Rust code. Follow idiomatic Rust style.
- **Error Handling:** Prefer `Result` and `?` operator. Use custom error types where appropriate.
- **Testing:** Use the built-in Rust test framework. Place integration tests in `tests/` and unit tests in `src/` modules.
- **Dependencies:** Use only necessary crates. List all dependencies in `Cargo.toml`.
- **Documentation:** Use Rust doc comments (`///`) for public functions, types, and modules.
- **static linking:** Ensure the binary is statically linked for portability.

## Copilot Usage Guidelines
- **Do:**
  - Suggest idiomatic Rust code.
  - Propose improvements for error handling, performance, and code clarity.
  - Generate test cases for new features.
  - Respect the existing project structure and naming conventions.
  - Use external crates only if justified and add them to `Cargo.toml`.
  - Write concise, clear commit messages and code comments.
- **Don't:**
  - Suggest unsafe code unless explicitly requested.
  - Add unnecessary dependencies.
  - Change the public API without a clear reason.
  - Generate code that is not cross-platform (must work on macOS, Linux, Windows).

## Special Notes
- The CLI must always print valid JSON to stdout for AST output.
- All supported languages must be detected by file extension.
- When parsing directories, ignore `target/` and other build artifact folders.
- The `--truncate` option must limit the length of stdout output.
- All new features must include tests in `tests/integration_tests.rs` or new test files.

## File/Folder Conventions
- `src/` — main source code
- `tests/` — integration tests
- `IMPROVEMENTS/` — plans, enhancements, and roadmap
- `build.rs`, `make-release.sh` — build scripts

## Example Commit Message
```
feat: add support for parsing Ruby files and outputting AST as JSON
```

---

*This file is for Copilot and other AI coding assistants. Please follow these instructions to ensure consistency and quality in code contributions.*

