use crate::encoding::Encoding;
use crate::error::{AstgenError, Result};
use crate::json::JsonNode;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use tree_sitter::{Language, Parser};

#[allow(dead_code)]
pub fn parse_file(path: PathBuf, encoding: &Encoding, truncate: Option<usize>) -> bool {
    match parse_file_safe(path, encoding, truncate) {
        Ok(output) => {
            println!("{}", output);
            true
        }
        Err(e) => {
            log::error!("Failed to parse file: {}", e);
            false
        }
    }
}

pub fn parse_file_safe(
    path: PathBuf,
    encoding: &Encoding,
    truncate: Option<usize>,
) -> Result<String> {
    parse_file_safe_with_size_limit(path, encoding, truncate, 10_000_000)
}

pub fn parse_file_safe_with_size_limit(
    path: PathBuf,
    encoding: &Encoding,
    truncate: Option<usize>,
    max_size_bytes: usize,
) -> Result<String> {
    // Check file size before reading
    let metadata = fs::metadata(&path)?;
    let file_size = metadata.len() as usize;

    if file_size > max_size_bytes {
        return Err(AstgenError::FileTooLarge {
            path: path.to_string_lossy().to_string(),
            size: file_size,
            limit: max_size_bytes,
        });
    }

    let content = fs::read_to_string(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::InvalidData {
            AstgenError::InvalidInput(format!(
                "File contains invalid UTF-8: {}\nTry converting the file to UTF-8 encoding first.",
                path.display()
            ))
        } else {
            AstgenError::IoError(e)
        }
    })?;

    let json_tree = build_parse_tree_safe(&content, encoding.language)?;

    let wrapped_json = json!({
        "version": "astgen-0.1",
        "filename": path.to_string_lossy(),
        "language": encoding.name,
        "ast": json_tree
    });

    let json_output = match truncate {
        Some(len) => {
            let full_output = serde_json::to_string(&wrapped_json)?;
            if full_output.len() > len {
                let mut truncated = full_output[..len].to_string();
                // Try to end at a reasonable boundary
                if let Some(last_brace) = truncated.rfind('}') {
                    truncated.truncate(last_brace + 1);
                }
                truncated
            } else {
                full_output
            }
        }
        None => serde_json::to_string(&wrapped_json)?,
    };

    Ok(json_output)
}

#[allow(dead_code)]
pub fn parse_file_with_parser(
    path: PathBuf,
    encoding: &Encoding,
    parser: &mut Parser,
    truncate: Option<usize>,
) -> Result<String> {
    let content = fs::read_to_string(&path)?;

    // Check file size limit
    let max_size = 10_000_000; // 10MB in bytes
    if content.len() > max_size {
        return Err(AstgenError::InvalidInput(format!(
            "File too large: {} bytes",
            content.len()
        )));
    }

    parser.set_language(encoding.language)?;

    let tree = parser
        .parse(&content, None)
        .ok_or_else(|| AstgenError::ParseError("Failed to parse content".to_string()))?;

    let root_node = tree.root_node();
    let json_tree = crate::json::node_to_json(&content, root_node);

    let wrapped_json = json!({
        "version": "astgen-0.1",
        "filename": path.to_string_lossy(),
        "language": encoding.name,
        "ast": json_tree
    });

    let json_output = match truncate {
        Some(len) => {
            let full_output = serde_json::to_string(&wrapped_json)?;
            if full_output.len() > len {
                full_output[..len].to_string()
            } else {
                full_output
            }
        }
        None => serde_json::to_string(&wrapped_json)?,
    };

    Ok(json_output)
}

fn build_parse_tree_safe(content: &str, lang: &Language) -> Result<JsonNode> {
    let mut parser = Parser::new();
    parser.set_language(lang)?;

    let tree = parser
        .parse(content, None)
        .ok_or_else(|| AstgenError::ParseError("Failed to parse content".to_string()))?;

    let root_node = tree.root_node();
    Ok(crate::json::node_to_json(content, root_node))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str, extension: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        let _file_path = file.path().to_str().unwrap();
        let _new_path = format!("{}.{}", _file_path, extension);

        // Write content and rename with proper extension
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();

        // Create new temp file with proper extension
        let mut new_file = NamedTempFile::with_suffix(&format!(".{}", extension)).unwrap();
        new_file.write_all(content.as_bytes()).unwrap();
        new_file.flush().unwrap();
        new_file
    }

    #[test]
    fn test_parse_file_rust_success() {
        let temp_file = create_temp_file("fn main() {}", "rs");
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new("rs$", &rust_language, "Rust");

        let result = parse_file(temp_file.path().to_path_buf(), &encoding, None);
        assert!(result);
    }

    #[test]
    fn test_parse_file_with_truncation() {
        let temp_file = create_temp_file("fn main() { println!(\"hello world\"); }", "rs");
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new("rs$", &rust_language, "Rust");

        // Test with truncation - should still succeed
        let result = parse_file(temp_file.path().to_path_buf(), &encoding, Some(100));
        assert!(result);
    }

    #[test]
    fn test_parse_file_javascript_success() {
        let temp_file = create_temp_file("console.log('hello');", "js");
        let js_language = tree_sitter_javascript::LANGUAGE.into();
        let encoding = Encoding::new("js$", &js_language, "JavaScript");

        let result = parse_file(temp_file.path().to_path_buf(), &encoding, None);
        assert!(result);
    }

    #[test]
    fn test_parse_file_python_success() {
        let temp_file = create_temp_file("print('hello')", "py");
        let python_language = tree_sitter_python::LANGUAGE.into();
        let encoding = Encoding::new("py$", &python_language, "Python");

        let result = parse_file(temp_file.path().to_path_buf(), &encoding, None);
        assert!(result);
    }

    #[test]
    fn test_parse_file_empty_file() {
        let temp_file = create_temp_file("", "rs");
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new("rs$", &rust_language, "Rust");

        let result = parse_file(temp_file.path().to_path_buf(), &encoding, None);
        assert!(result); // Empty files should parse successfully
    }

    #[test]
    fn test_build_parse_tree_rust() {
        let _temp_file = create_temp_file("fn main() {}", "rs");
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        let json_node = build_parse_tree_safe("fn main() {}", &rust_language).unwrap();

        assert_eq!(json_node.kind, "source_file");
        assert!(json_node.children.is_some());
    }

    #[test]
    fn test_build_parse_tree_preserves_content() {
        let content = "fn test() { let x = 42; }";
        let _temp_file = create_temp_file(content, "rs");
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        let json_node = build_parse_tree_safe(content, &rust_language).unwrap();

        assert_eq!(json_node.start_byte, 0);
        assert_eq!(json_node.end_byte, content.len());
    }

    #[test]
    fn test_build_parse_tree_with_different_languages() {
        // Test with multiple languages to ensure the function works generally
        let test_cases = vec![
            ("fn main() {}", "rs", tree_sitter_rust::LANGUAGE.into()),
            (
                "console.log('test');",
                "js",
                tree_sitter_javascript::LANGUAGE.into(),
            ),
            ("print('test')", "py", tree_sitter_python::LANGUAGE.into()),
            ("package main", "go", tree_sitter_go::LANGUAGE.into()),
        ];

        for (content, _ext, language) in test_cases {
            let json_node = build_parse_tree_safe(content, &language).unwrap();

            assert_eq!(json_node.start_byte, 0);
            assert_eq!(json_node.end_byte, content.len());
            assert!(!json_node.kind.is_empty());
        }
    }

    // Note: Testing file not found scenarios is tricky with the current implementation
    // as it uses expect() which panics. In a real application, this should be refactored
    // to return Result<JsonNode, Error> for better error handling.
}
