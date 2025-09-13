use crate::cli_types::{format_output, Args};
use crate::encodings;
use crate::error::{AstgenError, Result};
use crate::parser_pool;
use crate::parsing;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

pub fn process_single_file(
    file_path: &std::path::Path,
    encodings: &encodings::Encodings,
    args: &Args,
    _parser_pool: &Arc<parser_pool::ParserPool>,
) -> Result<bool> {
    // Check include/exclude patterns
    if !should_process_file(file_path, args) {
        return Ok(false);
    }

    let file_str = file_path.to_string_lossy();
    let encoding = encodings.match_file(&file_str);

    match encoding {
        Some(lang) => {
            if args.dry_run {
                if !args.quiet {
                    println!("Would parse: {} ({})", file_path.display(), lang.name);
                }
                return Ok(true);
            }

            // Calculate max file size in bytes
            let max_size_bytes = args.max_file_size * 1_000_000; // Convert MB to bytes

            match parsing::parse_file_safe_with_size_limit(
                file_path.to_path_buf(),
                lang,
                args.truncate,
                max_size_bytes,
            ) {
                Ok(output) => {
                    let formatted_output = format_output(&output, &args.format)?;
                    write_output(&formatted_output, args)?;

                    if args.verbose && !args.quiet {
                        log::info!("Parsed file: {}", file_path.display());
                    }
                    Ok(true)
                }
                Err(e) => {
                    if !args.quiet {
                        log::error!("Error parsing file {}: {}", file_path.display(), e);
                    }
                    Ok(false)
                }
            }
        }
        None => {
            if args.verbose && !args.quiet {
                let ext = file_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("unknown");
                if ext != "unknown" {
                    return Err(AstgenError::UnsupportedFileType(
                        file_path.to_string_lossy().to_string(),
                    ));
                }
                log::warn!(
                    "Unsupported file type .{} for file: {}",
                    ext,
                    file_path.display()
                );
            }
            Ok(false)
        }
    }
}

fn should_process_file(file_path: &std::path::Path, args: &Args) -> bool {
    let path_str = file_path.to_string_lossy();

    // Check exclude patterns first
    for exclude_pattern in &args.exclude {
        if glob_match(exclude_pattern, &path_str) {
            return false;
        }
    }

    // If include patterns are specified, file must match at least one
    if !args.include.is_empty() {
        return args
            .include
            .iter()
            .any(|pattern| glob_match(pattern, &path_str));
    }

    true
}

fn glob_match(pattern: &str, path: &str) -> bool {
    // Simple glob matching - could be enhanced with a proper glob library
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let (prefix, suffix) = (parts[0], parts[1]);
            return path.starts_with(prefix) && path.ends_with(suffix);
        }
    }
    path.contains(pattern)
}

fn write_output(content: &str, args: &Args) -> Result<()> {
    match &args.output {
        Some(output_path) => {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_path)?;
            writeln!(file, "{}", content)?;
        }
        None => {
            println!("{}", content);
        }
    }
    Ok(())
}

pub fn process_directory(
    dir_path: &std::path::Path,
    encodings: &encodings::Encodings,
    args: &Args,
    _parser_pool: &Arc<parser_pool::ParserPool>,
) -> Result<(usize, usize)> {
    let mut walker_builder = ignore::WalkBuilder::new(dir_path);
    walker_builder
        .add_custom_ignore_filename(".astgenignore")
        .follow_links(args.follow_links)
        .max_depth(Some(args.max_depth));

    // Add exclude patterns to walker
    for exclude_pattern in &args.exclude {
        walker_builder.add_ignore(format!("**/{}", exclude_pattern));
    }

    let walker = walker_builder.build();
    let files: Vec<PathBuf> = walker
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type()?.is_file() {
                let path = entry.into_path();
                // Additional filtering for include patterns
                if should_process_file(&path, args) {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        if !args.quiet {
            log::warn!(
                "No matching files found in directory: {}",
                dir_path.display()
            );
        }
        return Ok((0, 0));
    }

    if args.verbose && !args.quiet {
        log::info!("Found {} files to process", files.len());
    }

    let show_progress = args.progress || (!args.quiet && files.len() > 10);
    let progress_bar = if show_progress {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        pb.set_message("Processing files");
        Some(pb)
    } else {
        None
    };

    let results: Vec<Result<bool>> = files
        .par_iter()
        .map(|file| {
            let result = process_single_file(
                file,
                encodings,
                args,
                &Arc::new(parser_pool::ParserPool::new()),
            );
            if let Some(ref pb) = progress_bar {
                pb.inc(1);
                if args.verbose {
                    pb.set_message(format!(
                        "Processing {}",
                        file.file_name().unwrap_or_default().to_string_lossy()
                    ));
                }
            }
            result
        })
        .collect();

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Complete");
    }

    let success_count = results
        .iter()
        .filter(|r| r.as_ref().is_ok_and(|&b| b))
        .count();
    let error_count = results.len() - success_count;

    if args.verbose && !args.quiet {
        log::info!(
            "Successfully processed {} files, {} errors",
            success_count,
            error_count
        );
    }

    Ok((success_count, error_count))
}

#[allow(dead_code)]
pub fn should_walk_dir(dir: &str) -> bool {
    let ignore_dirs = vec!["target", "node_modules", ".git", ".venv"];
    for ignore_dir in ignore_dirs {
        if dir.contains(ignore_dir) {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn walk_dir(
    dir: &str,
    encodings: &encodings::Encodings,
    truncate: Option<usize>,
) -> (usize, usize) {
    let mut file_count = 0;
    let mut error_count = 0;
    if let Ok(paths) = fs::read_dir(dir) {
        for path in paths.flatten() {
            let path = path.path();
            if path.is_dir() {
                if should_walk_dir(path.to_str().unwrap_or_default()) {
                    let (f, e) = walk_dir(path.to_str().unwrap_or_default(), encodings, truncate);
                    file_count += f;
                    error_count += e;
                }
            } else {
                let encoding = encodings.match_file(&path.to_string_lossy());
                match encoding {
                    Some(lang) => {
                        if crate::parsing::parse_file(path, lang, truncate) {
                            file_count += 1;
                        } else {
                            error_count += 1;
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }
    (file_count, error_count)
}
