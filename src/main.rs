mod cli_types;
mod config;
mod encoding;
mod encodings;
mod error;
mod json;
mod parser_pool;
mod parsing;
mod versions; // Add new module
mod walk;

use clap::Parser;
use cli_types::Args;
use error::{AstgenError, Result};
use std::fs;
use std::sync::Arc;
// Import the version constants
use versions::*;

static VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    ".",
    include_str!(concat!(env!("OUT_DIR"), "/version.txt"))
);

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    // Handle special flags first
    if args.list_languages {
        list_supported_languages();
        return Ok(());
    }

    // Validate arguments
    args.validate()?;

    // Load configuration
    let config = if let Some(config_path) = &args.config {
        config::Config::load(config_path)?
    } else {
        config::Config::load_default()?
    };

    // Set up thread pool
    let num_threads = args
        .parallel
        .or(config.performance.as_ref().and_then(|p| p.max_threads))
        .unwrap_or_else(num_cpus::get);

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .map_err(|e| {
            AstgenError::InvalidInput(format!("Failed to initialize thread pool: {}", e))
        })?;

    if args.verbose {
        log::info!("Using {} threads for parallel processing", num_threads);
    }

    // Set up encodings
    let encodings = create_encodings();
    let parser_pool = Arc::new(parser_pool::ParserPool::new());

    // Process files
    if args.files.is_empty() {
        return Err(AstgenError::InvalidInput(
            "No input files specified".to_string(),
        ));
    }

    let total_start_time = std::time::Instant::now();
    let mut total_files = 0;
    let mut total_errors = 0;

    for file_arg in &args.files {
        match fs::metadata(&file_arg) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    if args.verbose && !args.quiet {
                        log::info!("Processing directory: {}", file_arg.display());
                    }
                    let (files, errors) =
                        walk::process_directory(&file_arg, &encodings, &args, &parser_pool)?;
                    total_files += files;
                    total_errors += errors;
                } else {
                    let result =
                        walk::process_single_file(&file_arg, &encodings, &args, &parser_pool)?;
                    if result {
                        total_files += 1;
                    } else {
                        total_errors += 1;
                    }
                }
            }
            Err(e) => {
                log::error!("Cannot access {}: {}", file_arg.display(), e);
                total_errors += 1;
            }
        }
    }

    let duration = total_start_time.elapsed();
    if args.verbose && !args.quiet {
        log::info!(
            "Processed {} files with {} errors in {:?}",
            total_files,
            total_errors,
            duration
        );
    }

    if total_errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn create_encodings() -> encodings::Encodings<'static> {
    use std::sync::OnceLock;

    static RUST_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static JAVA_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static CSHARP_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static GO_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static PYTHON_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static TYPESCRIPT_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static TSX_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static JAVASCRIPT_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
    static RUBY_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();

    let rust_lang = RUST_LANGUAGE.get_or_init(|| tree_sitter_rust::LANGUAGE.into());
    let java_lang = JAVA_LANGUAGE.get_or_init(|| tree_sitter_java::LANGUAGE.into());
    let csharp_lang = CSHARP_LANGUAGE.get_or_init(|| tree_sitter_c_sharp::LANGUAGE.into());
    let go_lang = GO_LANGUAGE.get_or_init(|| tree_sitter_go::LANGUAGE.into());
    let python_lang = PYTHON_LANGUAGE.get_or_init(|| tree_sitter_python::LANGUAGE.into());
    let typescript_lang =
        TYPESCRIPT_LANGUAGE.get_or_init(|| tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
    let tsx_lang = TSX_LANGUAGE.get_or_init(|| tree_sitter_typescript::LANGUAGE_TSX.into());
    let javascript_lang =
        JAVASCRIPT_LANGUAGE.get_or_init(|| tree_sitter_javascript::LANGUAGE.into());
    let ruby_lang = RUBY_LANGUAGE.get_or_init(|| tree_sitter_ruby::LANGUAGE.into());

    let mut encodings = encodings::Encodings::new();
    encodings
        .add("rs$", rust_lang, "Rust")
        .add("java$", java_lang, "Java")
        .add("cs$", csharp_lang, "C#")
        .add("go$", go_lang, "Go")
        .add("py$", python_lang, "Python")
        .add("ts$", typescript_lang, "TypeScript")
        .add("tsx$", tsx_lang, "TSX")
        .add("js$", javascript_lang, "JavaScript")
        .add("rb$", ruby_lang, "Ruby");

    encodings
}

fn list_supported_languages() {
    println!("Supported Languages:");
    println!("┌─────────────┬─────────────────┬─────────────────────────┐");
    println!("│ Language    │ Extensions      │ Tree-sitter Version     │");
    println!("├─────────────┼─────────────────┼─────────────────────────┤");
    println!("│ Rust        │ .rs             │ {:<23} │", TREE_SITTER_RUST_VERSION);
    println!("│ Java        │ .java           │ {:<23} │", TREE_SITTER_JAVA_VERSION);
    println!("│ C#          │ .cs             │ {:<23} │", TREE_SITTER_C_SHARP_VERSION);
    println!("│ Go          │ .go             │ {:<23} │", TREE_SITTER_GO_VERSION);
    println!("│ Python      │ .py             │ {:<23} │", TREE_SITTER_PYTHON_VERSION);
    println!("│ TypeScript  │ .ts             │ {:<23} │", TREE_SITTER_TYPESCRIPT_VERSION);
    println!("│ TSX         │ .tsx            │ {:<23} │", TREE_SITTER_TYPESCRIPT_VERSION);
    println!("│ JavaScript  │ .js             │ {:<23} │", TREE_SITTER_JAVASCRIPT_VERSION);
    println!("│ Ruby        │ .rb             │ {:<23} │", TREE_SITTER_RUBY_VERSION);
    println!("└─────────────┴─────────────────┴─────────────────────────┘");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use crate::walk::walk_dir;

    #[test]
    fn test_should_walk_dir_allows_normal_dirs() {
        assert!(walk::should_walk_dir("src"));
        assert!(walk::should_walk_dir("lib"));
        assert!(walk::should_walk_dir("tests"));
        assert!(walk::should_walk_dir("examples"));
        assert!(walk::should_walk_dir("/path/to/src"));
        assert!(walk::should_walk_dir("./my-project/src"));
    }

    #[test]
    fn test_should_walk_dir_ignores_target_dir() {
        assert!(!walk::should_walk_dir("target"));
        assert!(!walk::should_walk_dir("./target"));
        assert!(!walk::should_walk_dir("/path/to/target"));
        assert!(!walk::should_walk_dir("my-project/target"));
        assert!(!walk::should_walk_dir("target/debug"));
    }

    #[test]
    fn test_should_walk_dir_ignores_node_modules() {
        assert!(!walk::should_walk_dir("node_modules"));
        assert!(!walk::should_walk_dir("./node_modules"));
        assert!(!walk::should_walk_dir("/path/to/node_modules"));
        assert!(!walk::should_walk_dir("my-project/node_modules"));
        assert!(!walk::should_walk_dir("node_modules/some-package"));
    }

    #[test]
    fn test_should_walk_dir_ignores_git_dir() {
        assert!(!walk::should_walk_dir(".git"));
        assert!(!walk::should_walk_dir("./.git"));
        assert!(!walk::should_walk_dir("/path/to/.git"));
        assert!(!walk::should_walk_dir("my-project/.git"));
        assert!(!walk::should_walk_dir(".git/hooks"));
    }

    #[test]
    fn test_should_walk_dir_ignores_venv_dir() {
        assert!(!walk::should_walk_dir(".venv"));
        assert!(!walk::should_walk_dir("./.venv"));
        assert!(!walk::should_walk_dir("/path/to/.venv"));
        assert!(!walk::should_walk_dir("my-project/.venv"));
        // Note: "venv" without dot IS allowed - only ".venv" is ignored
        assert!(walk::should_walk_dir("venv"));
        assert!(walk::should_walk_dir("./venv"));
    }

    #[test]
    fn test_should_walk_dir_case_sensitive() {
        // Should be case sensitive
        assert!(walk::should_walk_dir("TARGET")); // Different case
        assert!(walk::should_walk_dir("Target"));
        assert!(walk::should_walk_dir("NODE_MODULES"));
    }

    #[test]
    fn test_walk_dir_counts_files_correctly() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test files
        let rust_file = temp_path.join("test.rs");
        fs::write(&rust_file, "fn main() {}").unwrap();

        let js_file = temp_path.join("test.js");
        fs::write(&js_file, "console.log('hello');").unwrap();

        let unknown_file = temp_path.join("test.txt");
        fs::write(&unknown_file, "some text").unwrap();

        // Create encodings
        let mut encodings = encodings::Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let js_language = tree_sitter_javascript::LANGUAGE.into();
        encodings
            .add("rs$", &rust_language, "Rust")
            .add("js$", &js_language, "JavaScript");

        let (file_count, error_count) = walk_dir(temp_path.to_str().unwrap(), &encodings, None);

        assert_eq!(file_count, 2); // .rs and .js files
        assert_eq!(error_count, 0); // No parsing errors expected
    }

    #[test]
    fn test_walk_dir_skips_ignored_directories() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a normal file
        let rust_file = temp_path.join("test.rs");
        fs::write(&rust_file, "fn main() {}").unwrap();

        // Create ignored directory with file
        let target_dir = temp_path.join("target");
        fs::create_dir(&target_dir).unwrap();
        let target_file = target_dir.join("ignored.rs");
        fs::write(&target_file, "fn ignored() {}").unwrap();

        // Create encodings
        let mut encodings = encodings::Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        encodings.add("rs$", &rust_language, "Rust");

        let (file_count, error_count) = walk_dir(temp_path.to_str().unwrap(), &encodings, None);

        assert_eq!(file_count, 1); // Only the file outside target/
        assert_eq!(error_count, 0);
    }

    #[test]
    fn test_walk_dir_handles_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create nested structure
        let src_dir = temp_path.join("src");
        fs::create_dir(&src_dir).unwrap();
        let nested_dir = src_dir.join("nested");
        fs::create_dir(&nested_dir).unwrap();

        // Create files at different levels
        fs::write(temp_path.join("root.rs"), "fn main() {}").unwrap();
        fs::write(src_dir.join("lib.rs"), "pub fn lib() {}").unwrap();
        fs::write(nested_dir.join("module.rs"), "pub fn module() {}").unwrap();

        // Create encodings
        let mut encodings = encodings::Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        encodings.add("rs$", &rust_language, "Rust");

        let (file_count, error_count) = walk_dir(temp_path.to_str().unwrap(), &encodings, None);

        assert_eq!(file_count, 3); // All three .rs files
        assert_eq!(error_count, 0);
    }

    #[test]
    fn test_walk_dir_with_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let mut encodings = encodings::Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        encodings.add("rs$", &rust_language, "Rust");

        let (file_count, error_count) = walk_dir(temp_path.to_str().unwrap(), &encodings, None);

        assert_eq!(file_count, 0);
        assert_eq!(error_count, 0);
    }
}
