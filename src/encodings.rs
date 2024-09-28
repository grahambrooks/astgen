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

    pub fn add(&mut self, extension_pattern: &str, language: &'a Language) -> &mut Self {
        self.encodings.push(Encoding::new(extension_pattern, language));
        self
    }

    pub fn match_file(&self, file_path: &str) -> Option<&Language> {
        for encoding in &self.encodings {
            if encoding.matches(file_path) {
                return Some(&encoding.language);
            }
        }
        None
    }
}
