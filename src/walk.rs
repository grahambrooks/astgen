use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::encodings;
use crate::parser_pool;
use crate::parsing;
use crate::cli_types::{Args, format_output};
use crate::error::Result;

pub fn process_single_file(
    file_path: &PathBuf,
    encodings: &encodings::Encodings,
    args: &Args,
    _parser_pool: &Arc<parser_pool::ParserPool>,
) -> Result<bool> {
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
            match parsing::parse_file_safe(file_path.clone(), lang, args.truncate) {
                Ok(output) => {
                    let formatted_output = format_output(&output, &args.format)?;
                    println!("{}", formatted_output);
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
                log::warn!("No language found for file: {}", file_path.display());
            }
            Ok(false)
        }
    }
}

pub fn process_directory(
    dir_path: &PathBuf,
    encodings: &encodings::Encodings,
    args: &Args,
    _parser_pool: &Arc<parser_pool::ParserPool>,
) -> Result<(usize, usize)> {
    let walker = ignore::WalkBuilder::new(dir_path)
        .add_custom_ignore_filename(".astgenignore")
        .follow_links(args.follow_links)
        .max_depth(Some(args.max_depth))
        .build();
    let files: Vec<PathBuf> = walker
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type()?.is_file() {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect();
    if args.verbose && !args.quiet {
        log::info!("Found {} files to process", files.len());
    }
    let progress_bar = if !args.quiet && files.len() > 10 {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
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
            let result = process_single_file(file, encodings, args, &Arc::new(parser_pool::ParserPool::new()));
            if let Some(ref pb) = progress_bar {
                pb.inc(1);
            }
            result
        })
        .collect();
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Complete");
    }
    let success_count = results.iter().filter(|r| r.as_ref().map_or(false, |&b| b)).count();
    let error_count = results.len() - success_count;
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
        for path in paths {
            if let Ok(path) = path {
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
    }
    (file_count, error_count)
}
