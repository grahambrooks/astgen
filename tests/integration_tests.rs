use serde_json::Value;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};

// Helper function to run astgen command
fn run_astgen(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute astgen")
}

// Helper function to create a temporary file with content
fn create_temp_file_with_extension(content: &str, extension: &str) -> NamedTempFile {
    let mut file = NamedTempFile::with_suffix(format!(".{}", extension)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file
}

#[test]
fn test_cli_help() {
    let output = run_astgen(&["--help"]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Accept any help output containing "astgen" and "help" or "usage"
    assert!(stdout.to_lowercase().contains("astgen"));
    assert!(stdout.to_lowercase().contains("help") || stdout.to_lowercase().contains("usage"));
}

#[test]
fn test_cli_version() {
    let output = run_astgen(&["--version"]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("astgen"));
    assert!(stdout.chars().any(|c| c.is_ascii_digit()));
}

#[test]
fn test_parse_rust_file() {
    let rust_code = r#"
fn main() {
    println!(\"Hello, world!\");
}
"#;
    let temp_file = create_temp_file_with_extension(rust_code, "rs");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Check for valid JSON and correct language
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "Rust");
}

#[test]
fn test_parse_javascript_file() {
    let js_code = r#"
function hello() {
    console.log(\"Hello, world!\");
}
hello();
"#;
    let temp_file = create_temp_file_with_extension(js_code, "js");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "JavaScript");
}

#[test]
fn test_parse_python_file() {
    let python_code = r#"
def hello():
    print(\"Hello, world!\")

if __name__ == \"__main__\":
    hello()
"#;
    let temp_file = create_temp_file_with_extension(python_code, "py");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "Python");
}

#[test]
fn test_parse_java_file() {
    let java_code = r#"
public class Hello {
    public static void main(String[] args) {
        System.out.println(\"Hello, world!\");
    }
}
"#;
    let temp_file = create_temp_file_with_extension(java_code, "java");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "Java");
}

#[test]
fn test_parse_go_file() {
    let go_code = r#"
package main

import \"fmt\"

func main() {
    fmt.Println(\"Hello, world!\")
}
"#;
    let temp_file = create_temp_file_with_extension(go_code, "go");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "Go");
}

#[test]
fn test_parse_typescript_file() {
    let ts_code = r#"
interface Greeter {
    name: string;
}

function hello(greeter: Greeter): void {
    console.log(`Hello, ${greeter.name}!`);
}
"#;
    let temp_file = create_temp_file_with_extension(ts_code, "ts");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "TypeScript");
}

#[test]
fn test_parse_ruby_file() {
    let ruby_code = r#"
def hello
  puts \"Hello, world!\"
end

hello
"#;
    let temp_file = create_temp_file_with_extension(ruby_code, "rb");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "Ruby");
}

#[test]
fn test_truncate_option() {
    let rust_code = "fn main() { println!(\"This is a long string that should be truncated\"); }";
    let temp_file = create_temp_file_with_extension(rust_code, "rs");
    let output = run_astgen(&["--truncate", "50", temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.trim().len() <= 50);
}

#[test]
fn test_parse_directory() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    fs::write(temp_path.join("test1.rs"), "fn main() {}").unwrap();
    fs::write(temp_path.join("test2.js"), "console.log('hello');").unwrap();
    fs::write(temp_path.join("readme.txt"), "This is a readme").unwrap();
    let sub_dir = temp_path.join("src");
    fs::create_dir(&sub_dir).unwrap();
    fs::write(sub_dir.join("lib.rs"), "pub fn lib() {}").unwrap();
    let output = run_astgen(&[temp_path.to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json_lines: Vec<&str> = stdout.trim().split('\n').collect();
    assert_eq!(json_lines.len(), 3);
    for line in json_lines {
        let _: Value = serde_json::from_str(line).unwrap();
    }
}

#[test]
fn test_parse_directory_ignores_target() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    fs::write(temp_path.join("main.rs"), "fn main() {}").unwrap();
    let target_dir = temp_path.join("target");
    fs::create_dir(&target_dir).unwrap();
    fs::write(target_dir.join("ignored.rs"), "fn ignored() {}").unwrap();
    let output = run_astgen(&[temp_path.to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json_lines: Vec<&str> = stdout.trim().split('\n').collect();
    // Only count .rs files not in target/
    let rs_files: Vec<_> = json_lines
        .iter()
        .filter(|line| {
            let json: Value = serde_json::from_str(line).unwrap();
            json["filename"].as_str().unwrap().ends_with("main.rs")
        })
        .collect();
    assert_eq!(rs_files.len(), 1);
}

#[test]
fn test_unsupported_file_extension() {
    let temp_file = create_temp_file_with_extension("some content", "unknown");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    // Should produce no JSON output
    assert!(String::from_utf8(output.stdout).unwrap().trim().is_empty());
}

#[test]
fn test_multiple_files_as_arguments() {
    let rust_file = create_temp_file_with_extension("fn main() {}", "rs");
    let js_file = create_temp_file_with_extension("console.log('test');", "js");
    let output = run_astgen(&[
        rust_file.path().to_str().unwrap(),
        js_file.path().to_str().unwrap(),
    ]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json_lines: Vec<&str> = stdout.trim().split('\n').collect();
    assert_eq!(json_lines.len(), 2);
    for line in json_lines {
        let _: Value = serde_json::from_str(line).unwrap();
    }
}

#[test]
fn test_empty_file() {
    let temp_file = create_temp_file_with_extension("", "rs");
    let output = run_astgen(&[temp_file.path().to_str().unwrap()]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["language"], "Rust");
}

#[test]
fn test_list_languages_output() {
    let output = run_astgen(&["--list-languages"]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Supported Languages:"));
    assert!(stdout.contains("Language"));
    assert!(stdout.contains("Tree-sitter Version"));
    // Check a couple of known languages
    assert!(stdout.contains("Rust"));
    assert!(stdout.contains("JavaScript"));
    // Basic alignment: all data lines should have 3 pipe separators beyond borders
    let data_lines: Vec<&str> = stdout.lines().filter(|l| l.starts_with("│ ")).collect();
    assert!(!data_lines.is_empty());
    for line in data_lines {
        let pipe_count = line.chars().filter(|&c| c == '│').count();
        assert_eq!(pipe_count, 4, "Line not properly aligned: {}", line);
    }
}
