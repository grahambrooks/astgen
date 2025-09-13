use regex::Regex;
use std::path::Path;
use tree_sitter::Language;

pub struct Encoding<'a> {
    extension_pattern: Regex,
    pub(crate) language: &'a Language,
    pub(crate) name: String,
}

impl<'a> Encoding<'a> {
    pub(crate) fn new(extension_pattern: &str, x: &'a Language, name: &'a str) -> Self {
        let regex_pattern = Regex::new(extension_pattern).expect("Invalid regex pattern");
        Self {
            extension_pattern: regex_pattern,
            language: x,
            name: name.to_string(),
        }
    }

    pub(crate) fn matches(&self, file_path: &str) -> bool {
        if let Some(extension) = Path::new(file_path).extension().and_then(|e| e.to_str()) {
            return self.extension_pattern.is_match(extension);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_single_extension() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new(r"rs$", &rust_language, "Rust");
        assert!(encoding.matches("src/main.rs"));
    }

    #[test]
    fn matches_multiple_extensions() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new(r"(rs|txt)$", &rust_language, "Rust");
        assert!(encoding.matches("src/main.rs"));
        assert!(encoding.matches("src/main.txt"));
    }

    #[test]
    fn does_not_match_incorrect_extension() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new(r"rs$", &rust_language, "Rust");
        assert!(!encoding.matches("src/main.txt"));
    }

    #[test]
    fn does_not_match_no_extension() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new(r"rs$", &rust_language, "Rust");
        assert!(!encoding.matches("src/main"));
    }

    #[test]
    fn matches_case_insensitive_extension() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let encoding = Encoding::new(r"(?i)rs$", &rust_language, "Rust");
        assert!(encoding.matches("src/main.RS"));
    }

    #[test]
    fn matches_ruby_extension() {
        let ruby_language = tree_sitter_ruby::LANGUAGE.into();
        let encoding = Encoding::new(r"rb$", &ruby_language, "Ruby");
        assert!(encoding.matches("src/main.rb"));
    }
}
