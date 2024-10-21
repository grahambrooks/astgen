mod encoding;
mod encodings;
mod parsing;
mod json;

use clap::Parser;
use std::fs;

static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), ".", include_str!(concat!(env!("OUT_DIR"), "/version.txt")));

#[derive(Parser, Debug)]
#[command(
color = clap::ColorChoice::Auto,
author = "Graham Brooks",
version = VERSION,
about = "AST generator based on tree-sitter",
long_about = r#"
CLI for generating ASTs for

  * Rust
  * Java
  * C#
  * Go
  * Python
  * Typescript
  * TSX
  * JavaScript
  * Ruby

astgen is a fairly simple wrapper around https://tree-sitter.github.io/tree-sitter/ parsers.

"#
)]
struct Args {
    #[arg(long, help = "Truncate the JSON line output for each line. Useful for previewing the output when scanning a large number of files")]
    truncate: Option<usize>,
    files: Vec<String>,
}


fn main() {
    let args = Args::parse();

    let mut encodings = encodings::Encodings::new();
    let rust_language = tree_sitter_rust::LANGUAGE.into();
    let java_language = tree_sitter_java::LANGUAGE.into();
    let csharp_language = tree_sitter_c_sharp::LANGUAGE.into();
    let go_language = tree_sitter_go::LANGUAGE.into();
    let python_language = tree_sitter_python::LANGUAGE.into();
    let typescript_language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    let tsx_language = tree_sitter_typescript::LANGUAGE_TSX.into();
    let javascript_language = tree_sitter_javascript::LANGUAGE.into();
    let ruby_language = tree_sitter_ruby::LANGUAGE.into();

    encodings.add("^rs$", &rust_language, "Rust")
        .add("^java$", &java_language, "Java")
        .add("^cs$", &csharp_language, "C#")
        .add("^go$", &go_language, "Go")
        .add("^py$", &python_language, "Python")
        .add("^ts$", &typescript_language, "TypeScript")
        .add("^tsx$", &tsx_language, "TSX")
        .add("^js$", &javascript_language, "JavaScript")
        .add("^rb$", &ruby_language, "Ruby");
    for arg in args.files {
        let start_time = std::time::Instant::now();

        if !fs::metadata(&arg).unwrap().is_dir() {
            let encoding = encodings.match_file(&arg);
            match encoding {
                Some(lang) => {
                    if parsing::parse_file(arg.clone().into(), lang, args.truncate) {
                        eprintln!("Parsed file: {}", arg.clone());
                    } else {
                        eprintln!("Error parsing file: {}", arg.clone());
                    }
                }
                None => {
                    eprintln!("No language found for file: {}", arg);
                }
            }
            continue;
        }

        eprintln!("Walking directory: {}", arg);
        let (file_count, error_count) = walk_dir(&arg, &encodings, args.truncate);
        let duration = start_time.elapsed();
        eprintln!("Parsed {} files with {} errors in {:?}", file_count, error_count, duration);
    }
}

fn walk_dir(dir: &str, encodings: &encodings::Encodings, truncate: Option<usize>) -> (usize, usize) {
    let mut file_count = 0;
    let mut error_count = 0;
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            if should_walk_dir(path.to_str().unwrap()) {
                let (f, e) = walk_dir(path.to_str().unwrap(), encodings, truncate);
                file_count += f;
                error_count += e;
            }
        } else {
            let encoding = encodings.match_file(path.to_str().unwrap());

            match encoding {
                Some(lang) => {
                    if parsing::parse_file(path, lang, truncate) {
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
    (file_count, error_count)
}

fn should_walk_dir(dir: &str) -> bool {
    let ignore_dirs = vec!["target", "node_modules", ".git", ".venv"];
    for ignore_dir in ignore_dirs {
        if dir.contains(ignore_dir) {
            return false;
        }
    }
    true
}

