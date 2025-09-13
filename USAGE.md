# astgen Usage Guide

## Basic Usage

Parse a single file:
```bash
astgen src/main.rs
```

Parse multiple files:
```bash
astgen src/main.rs src/lib.rs
```

Parse an entire directory:
```bash
astgen src/
```

## Output Formats

JSON (default):
```bash
astgen --format json src/main.rs
```

Pretty JSON:
```bash
astgen --format pretty-json src/main.rs
```

YAML:
```bash
astgen --format yaml src/main.rs
```

## Filtering Files

Include only specific patterns:
```bash
astgen --include "*.rs" --include "*.py" src/
```

Exclude specific patterns:
```bash
astgen --exclude "test*" --exclude "*.tmp" src/
```

## Output Options

Save to file:
```bash
astgen --output results.json src/
```

Truncate long output:
```bash
astgen --truncate 1000 src/main.rs
```

## Performance Options

Use specific number of threads:
```bash
astgen --parallel 4 src/
```

Set maximum file size (in MB):
```bash
astgen --max-file-size 50 src/
```

Show progress bar:
```bash
astgen --progress src/
```

## Debugging

Verbose output:
```bash
astgen --verbose src/
```

Dry run (show what would be processed):
```bash
astgen --dry-run src/
```

Quiet mode (suppress warnings):
```bash
astgen --quiet src/
```

## Configuration

Use custom config file:
```bash
astgen --config my-config.toml src/
```

List supported languages:
```bash
astgen --list-languages
```

## Examples

Parse all Rust files in a project, excluding tests:
```bash
astgen --include "*.rs" --exclude "*test*" --exclude "target/*" .
```

Generate pretty JSON for Python files with progress:
```bash
astgen --format pretty-json --include "*.py" --progress --output python-asts.json src/
```

Quick check of what files would be processed:
```bash
astgen --dry-run --verbose src/
```