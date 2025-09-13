use crate::error::Result;
use dashmap::DashMap;
use tree_sitter::{Language, Parser};

#[allow(dead_code)]
pub struct ParserPool {
    parsers: DashMap<String, Vec<Parser>>,
    max_pool_size: usize,
}

impl ParserPool {
    pub fn new() -> Self {
        Self {
            parsers: DashMap::new(),
            max_pool_size: 10,
        }
    }

    #[allow(dead_code)]
    pub fn get_parser(&self, language_name: &str, language: &Language) -> Result<Parser> {
        let mut entry = self
            .parsers
            .entry(language_name.to_string())
            .or_default();

        if let Some(mut parser) = entry.pop() {
            parser.set_language(language)?;
            Ok(parser)
        } else {
            let mut parser = Parser::new();
            parser.set_language(language)?;
            Ok(parser)
        }
    }

    #[allow(dead_code)]
    pub fn return_parser(&self, language_name: &str, parser: Parser) {
        if let Some(mut entry) = self.parsers.get_mut(language_name) {
            if entry.len() < self.max_pool_size {
                entry.push(parser);
            }
        }
    }
}

impl Default for ParserPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter_rust::LANGUAGE as RUST_LANGUAGE;

    #[test]
    fn test_get_and_return_parser() {
        let pool = ParserPool::new();
        let rust_lang = RUST_LANGUAGE.into();
        let parser = pool.get_parser("Rust", &rust_lang).unwrap();
        pool.return_parser("Rust", parser);
        // Should be able to get a parser again
        let _ = pool.get_parser("Rust", &rust_lang).unwrap();
    }

    #[test]
    fn test_pool_size_limit() {
        let pool = ParserPool::new();
        let rust_lang = RUST_LANGUAGE.into();
        let mut parsers = Vec::new();
        for _ in 0..15 {
            let parser = pool.get_parser("Rust", &rust_lang).unwrap();
            parsers.push(parser);
        }
        for parser in parsers {
            pool.return_parser("Rust", parser);
        }
        // Pool should not exceed max_pool_size (10)
        let entry = pool.parsers.get("Rust").unwrap();
        assert!(entry.len() <= 10);
    }
}
