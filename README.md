# Abstract Tree Generator (astgen)

**astgen** is a simple yet powerful tool designed to generate an abstract syntax tree (AST) from a given source code file. It utilizes pre-built [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammars to parse source code and generate an AST in JSON format, outputting each AST as a single line of JSON per parsed file.

## Features

- Supports multiple programming languages using Tree-sitter grammars.
- Outputs AST in JSON format for easy integration and processing.
- Lightweight and easy to use.

## Installation

### Project Build and Release Status

![CI](https://github.com/grahambrooks/astgen/actions/workflows/ci.yaml/badge.svg)
![Release](https://github.com/grahambrooks/astgen/actions/workflows/release.yml/badge.svg)

### How to Install

Instructions for installing **astgen** can be found under the [Releases](https://github.com/grahambrooks/astgen/releases) section on GitHub.

## Usage

### Basic Usage

Parse a single file:
```bash
astgen src/main.rs
```

Parse multiple files or directories:
```bash
astgen src/ tests/ examples/main.py
```

### Advanced Options

```bash
# Pretty JSON output with progress bar
astgen --format pretty-json --progress src/

# Filter files and save to file
astgen --include "*.rs" --exclude "*test*" --output results.json src/

# Parallel processing with custom thread count
astgen --parallel 8 --max-file-size 50 src/

# Dry run to see what would be processed
astgen --dry-run --verbose src/
```

See [USAGE.md](USAGE.md) for comprehensive usage examples.

## Creating a Release

Releases are cut by pushing a CalVer tag (`vYYYY.M.N`, e.g. `v2026.9.0`):

```bash
git tag v2026.9.0 && git push origin v2026.9.0
```

The `release` workflow (`.github/workflows/release.yml`, release-kit v2, configured by `.release.env`)
stamps the tag's version into `Cargo.toml`, builds `astgen-<tag>-<target>.tar.gz` for macOS and Linux
(x86_64 and aarch64), publishes them with `SHA256SUMS` on the GitHub release, and merges a PR that updates
the in-repo Homebrew formula `Formula/astgen.rb`. Install with:

```bash
brew tap grahambrooks/astgen https://github.com/grahambrooks/astgen
brew install grahambrooks/astgen/astgen
```

or run it without installing via `bx grahambrooks/astgen`.

## Contributing

We welcome contributions from the community. Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a Pull Request.

## License

**astgen** is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any questions or feedback, feel free to open an issue on the [GitHub repo](https://github.com/grahambrooks/astgen/issues).