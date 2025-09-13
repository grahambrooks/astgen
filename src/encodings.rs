use crate::encoding::Encoding;
use tree_sitter::Language;

pub struct Encodings<'a> {
    encodings: Vec<Encoding<'a>>,
}

impl<'a> Encodings<'a> {
    pub fn new() -> Self {
        Self {
            encodings: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        extension_pattern: &str,
        language: &'a Language,
        name: &'a str,
    ) -> &mut Self {
        self.encodings
            .push(Encoding::new(extension_pattern, language, name));
        self
    }
    pub fn match_file(&self, file_path: &str) -> Option<&Encoding<'_>> {
        self.encodings
            .iter()
            .find(|encoding| encoding.matches(file_path))
    }

    #[allow(dead_code)]
    pub fn match_file_or_error(&self, file_path: &str) -> crate::error::Result<&Encoding<'_>> {
        self.match_file(file_path).ok_or_else(|| {
            let ext = std::path::Path::new(file_path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown");
            crate::error::AstgenError::LanguageNotSupported(ext.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_encodings() {
        let encodings = Encodings::new();
        assert_eq!(encodings.encodings.len(), 0);
    }

    #[test]
    fn add_single_encoding() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        encodings.add("rs$", &rust_language, "Rust");

        assert_eq!(encodings.encodings.len(), 1);
        assert_eq!(encodings.encodings[0].name, "Rust");
    }

    #[test]
    fn add_multiple_encodings_chained() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let java_language = tree_sitter_java::LANGUAGE.into();

        encodings
            .add("rs$", &rust_language, "Rust")
            .add("java$", &java_language, "Java");

        assert_eq!(encodings.encodings.len(), 2);
        assert_eq!(encodings.encodings[0].name, "Rust");
        assert_eq!(encodings.encodings[1].name, "Java");
    }

    #[test]
    fn match_file_finds_correct_encoding() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let java_language = tree_sitter_java::LANGUAGE.into();

        encodings
            .add("rs$", &rust_language, "Rust")
            .add("java$", &java_language, "Java");

        let result = encodings.match_file("src/main.rs");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Rust");

        let result = encodings.match_file("src/Main.java");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Java");
    }

    #[test]
    fn match_file_returns_none_for_no_match() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        encodings.add("rs$", &rust_language, "Rust");

        let result = encodings.match_file("src/main.txt");
        assert!(result.is_none());
    }

    #[test]
    fn match_file_returns_none_for_empty_encodings() {
        let encodings = Encodings::new();

        let result = encodings.match_file("src/main.rs");
        assert!(result.is_none());
    }

    #[test]
    fn match_file_returns_first_match_when_multiple_patterns_match() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let go_language = tree_sitter_go::LANGUAGE.into();

        // Both patterns could match "test.rs" but first one should win
        encodings
            .add("s$", &rust_language, "Rust") // matches 's' at end
            .add("rs$", &go_language, "Go"); // matches 'rs' at end

        let result = encodings.match_file("test.rs");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Rust"); // First match wins
    }

    #[test]
    fn match_file_with_complex_path() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        encodings.add("rs$", &rust_language, "Rust");

        let result = encodings.match_file("/very/long/path/to/src/main.rs");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Rust");
    }

    #[test]
    fn match_file_with_case_sensitive_pattern() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        encodings.add("rs$", &rust_language, "Rust");

        // Case sensitive by default
        let result = encodings.match_file("src/main.RS");
        assert!(result.is_none());
    }

    #[test]
    fn match_file_with_case_insensitive_pattern() {
        let mut encodings = Encodings::new();
        let rust_language = tree_sitter_rust::LANGUAGE.into();

        encodings.add("(?i)rs$", &rust_language, "Rust");

        let result = encodings.match_file("src/main.RS");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Rust");
    }
}
