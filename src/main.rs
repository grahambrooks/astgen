mod cli_types;
mod config;
mod encoding;
mod encodings;
mod error;
mod json;
mod parsing;
mod versions; // Add new module
mod walk;
mod languages; // new module

use clap::Parser;
use cli_types::Args;
use error::{AstgenError, Result};
use std::fs;
use languages::{create_encodings, print_supported_languages};

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
        print_supported_languages();
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
