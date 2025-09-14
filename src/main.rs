mod cli_types;
mod config;
mod encoding;
mod encodings;
mod error;
mod json;
mod parsing;
mod versions; // Add new module
mod walk;

use clap::Parser;
use cli_types::Args;
use error::{AstgenError, Result};
use std::fs;
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

    // Process files
    if args.files.is_empty() {
        return Err(AstgenError::InvalidInput(
            "No input files specified.\n\nUsage: astgen <files...>\nExample: astgen src/main.rs\nExample: astgen src/\n\nUse --help for more options.".to_string(),
        ));
    }

    let total_start_time = std::time::Instant::now();
    let mut total_files = 0;
    let mut total_errors = 0;

    for file_arg in &args.files {
        match fs::metadata(file_arg) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    if args.verbose && !args.quiet {
                        log::info!("Processing directory: {}", file_arg.display());
                    }
                    let (files, errors) = walk::process_directory(file_arg, &encodings, &args)?;
                    total_files += files;
                    total_errors += errors;
                } else {
                    let result = walk::process_single_file(file_arg, &encodings, &args)?;
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
    println!(
        "│ Rust        │ .rs             │ {:<23} │",
        TREE_SITTER_RUST_VERSION
    );
    println!(
        "│ Java        │ .java           │ {:<23} │",
        TREE_SITTER_JAVA_VERSION
    );
    println!(
        "│ C#          │ .cs             │ {:<23} │",
        TREE_SITTER_C_SHARP_VERSION
    );
    println!(
        "│ Go          │ .go             │ {:<23} │",
        TREE_SITTER_GO_VERSION
    );
    println!(
        "│ Python      │ .py             │ {:<23} │",
        TREE_SITTER_PYTHON_VERSION
    );
    println!(
        "│ TypeScript  │ .ts             │ {:<23} │",
        TREE_SITTER_TYPESCRIPT_VERSION
    );
    println!(
        "│ TSX         │ .tsx            │ {:<23} │",
        TREE_SITTER_TYPESCRIPT_VERSION
    );
    println!(
        "│ JavaScript  │ .js             │ {:<23} │",
        TREE_SITTER_JAVASCRIPT_VERSION
    );
    println!(
        "│ Ruby        │ .rb             │ {:<23} │",
        TREE_SITTER_RUBY_VERSION
    );
    println!("└─────────────┴─────────────────┴─────────────────────────┘");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_create_encodings_not_empty() {
        let encodings = create_encodings();

        // Test that we can match some common file extensions
        assert!(encodings.match_file("test.rs").is_some());
        assert!(encodings.match_file("test.js").is_some());
        assert!(encodings.match_file("test.py").is_some());
        assert!(encodings.match_file("test.java").is_some());
        assert!(encodings.match_file("test.go").is_some());
        assert!(encodings.match_file("test.ts").is_some());
        assert!(encodings.match_file("test.tsx").is_some());
        assert!(encodings.match_file("test.cs").is_some());
        assert!(encodings.match_file("test.rb").is_some());
    }

    #[test]
    fn test_create_encodings_handles_unknown_extensions() {
        let encodings = create_encodings();
        assert!(encodings.match_file("test.unknown").is_none());
        assert!(encodings.match_file("test.txt").is_none());
        assert!(encodings.match_file("test").is_none());
    }

    #[test]
    fn test_walk_directory_processes_files() {
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
        let encodings = create_encodings();

        // Test with dry run to avoid actual processing in tests
        let args = crate::cli_types::Args {
            files: vec![temp_path.to_path_buf()],
            format: crate::cli_types::OutputFormat::Json,
            truncate: None,
            verbose: false,
            quiet: true,
            parallel: None,
            dry_run: true,
            max_file_size: 10,
            follow_links: false,
            max_depth: 100,
            list_languages: false,
            config: None,
            include: vec![],
            exclude: vec![],
            output: None,
            progress: false,
        };

        let result = walk::process_directory(temp_path, &encodings, &args);

        assert!(result.is_ok());
        let (file_count, error_count) = result.unwrap();
        assert_eq!(file_count, 2); // .rs and .js files
        assert_eq!(error_count, 1); // .txt file is unsupported
    }

    #[test]
    fn test_walk_directory_with_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let encodings = create_encodings();

        let args = crate::cli_types::Args {
            files: vec![temp_path.to_path_buf()],
            format: crate::cli_types::OutputFormat::Json,
            truncate: None,
            verbose: false,
            quiet: true,
            parallel: None,
            dry_run: true,
            max_file_size: 10,
            follow_links: false,
            max_depth: 100,
            list_languages: false,
            config: None,
            include: vec![],
            exclude: vec![],
            output: None,
            progress: false,
        };

        let result = walk::process_directory(temp_path, &encodings, &args);

        assert!(result.is_ok());
        let (file_count, error_count) = result.unwrap();
        assert_eq!(file_count, 0);
        assert_eq!(error_count, 0);
    }

    #[test]
    fn test_walk_directory_with_nested_structure() {
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

        let encodings = create_encodings();

        let args = crate::cli_types::Args {
            files: vec![temp_path.to_path_buf()],
            format: crate::cli_types::OutputFormat::Json,
            truncate: None,
            verbose: false,
            quiet: true,
            parallel: None,
            dry_run: true,
            max_file_size: 10,
            follow_links: false,
            max_depth: 100,
            list_languages: false,
            config: None,
            include: vec![],
            exclude: vec![],
            output: None,
            progress: false,
        };

        let result = walk::process_directory(temp_path, &encodings, &args);

        assert!(result.is_ok());
        let (file_count, error_count) = result.unwrap();
        assert_eq!(file_count, 3); // All three .rs files
        assert_eq!(error_count, 0);
    }
}
