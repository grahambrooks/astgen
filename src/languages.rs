//! Language metadata and utilities for supported tree-sitter languages
use std::sync::OnceLock;

use crate::encodings;
use crate::versions::*;

/// Information about a supported language
pub struct LanguageInfo {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub version: &'static str,
}

// Build tree-sitter language singletons lazily
static RUST_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static JAVA_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static CSHARP_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static GO_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static PYTHON_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static TYPESCRIPT_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static TSX_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static JAVASCRIPT_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static RUBY_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();

/// Return static metadata for all supported languages.
pub fn supported_languages() -> Vec<LanguageInfo> {
    vec![
        LanguageInfo { name: "Rust", extensions: &[".rs"], version: TREE_SITTER_RUST_VERSION },
        LanguageInfo { name: "Java", extensions: &[".java"], version: TREE_SITTER_JAVA_VERSION },
        LanguageInfo { name: "C#", extensions: &[".cs"], version: TREE_SITTER_C_SHARP_VERSION },
        LanguageInfo { name: "Go", extensions: &[".go"], version: TREE_SITTER_GO_VERSION },
        LanguageInfo { name: "Python", extensions: &[".py"], version: TREE_SITTER_PYTHON_VERSION },
        LanguageInfo { name: "TypeScript", extensions: &[".ts"], version: TREE_SITTER_TYPESCRIPT_VERSION },
        LanguageInfo { name: "TSX", extensions: &[".tsx"], version: TREE_SITTER_TYPESCRIPT_VERSION },
        LanguageInfo { name: "JavaScript", extensions: &[".js"], version: TREE_SITTER_JAVASCRIPT_VERSION },
        LanguageInfo { name: "Ruby", extensions: &[".rb"], version: TREE_SITTER_RUBY_VERSION },
    ]
}

/// Create the encodings structure (file extension regex -> language mapping)
pub fn create_encodings() -> encodings::Encodings<'static> {
    let rust_lang = RUST_LANGUAGE.get_or_init(|| tree_sitter_rust::LANGUAGE.into());
    let java_lang = JAVA_LANGUAGE.get_or_init(|| tree_sitter_java::LANGUAGE.into());
    let csharp_lang = CSHARP_LANGUAGE.get_or_init(|| tree_sitter_c_sharp::LANGUAGE.into());
    let go_lang = GO_LANGUAGE.get_or_init(|| tree_sitter_go::LANGUAGE.into());
    let python_lang = PYTHON_LANGUAGE.get_or_init(|| tree_sitter_python::LANGUAGE.into());
    let typescript_lang = TYPESCRIPT_LANGUAGE.get_or_init(|| tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
    let tsx_lang = TSX_LANGUAGE.get_or_init(|| tree_sitter_typescript::LANGUAGE_TSX.into());
    let javascript_lang = JAVASCRIPT_LANGUAGE.get_or_init(|| tree_sitter_javascript::LANGUAGE.into());
    let ruby_lang = RUBY_LANGUAGE.get_or_init(|| tree_sitter_ruby::LANGUAGE.into());

    let mut enc = encodings::Encodings::new();
    enc
        .add("rs$", rust_lang, "Rust")
        .add("java$", java_lang, "Java")
        .add("cs$", csharp_lang, "C#")
        .add("go$", go_lang, "Go")
        .add("py$", python_lang, "Python")
        .add("ts$", typescript_lang, "TypeScript")
        .add("tsx$", tsx_lang, "TSX")
        .add("js$", javascript_lang, "JavaScript")
        .add("rb$", ruby_lang, "Ruby");
    enc
}

/// Print a formatted table of supported languages (dynamic column sizing for alignment)
pub fn print_supported_languages() {
    let langs = supported_languages();

    // Reference the generated parser versions list to validate build-time discovery
    // (keeps the constant from being optimized away / warning about dead code)
    let _discovered_parser_count = TREE_SITTER_PARSERS.len();

    let header_lang = "Language";
    let header_ext = "Extensions";
    let header_ver = "Tree-sitter Version";

    let lang_width = langs.iter().map(|l| l.name.len()).chain([header_lang.len()]).max().unwrap();
    let ext_width = langs.iter().map(|l| l.extensions.join(", ").len()).chain([header_ext.len()]).max().unwrap();
    let ver_width = langs.iter().map(|l| l.version.len()).chain([header_ver.len()]).max().unwrap();

    let border_top = format!("┌{}┬{}┬{}┐",
        "─".repeat(lang_width + 2),
        "─".repeat(ext_width + 2),
        "─".repeat(ver_width + 2)
    );
    let border_mid = format!("├{}┼{}┼{}┤",
        "─".repeat(lang_width + 2),
        "─".repeat(ext_width + 2),
        "─".repeat(ver_width + 2)
    );
    let border_bottom = format!("└{}┴{}┴{}┘",
        "─".repeat(lang_width + 2),
        "─".repeat(ext_width + 2),
        "─".repeat(ver_width + 2)
    );

    println!("Supported Languages:");
    println!("{}", border_top);
    println!("│ {:<lang_width$} │ {:<ext_width$} │ {:<ver_width$} │",
        header_lang,
        header_ext,
        header_ver,
        lang_width = lang_width,
        ext_width = ext_width,
        ver_width = ver_width
    );
    println!("{}", border_mid);

    for info in langs {
        let exts = info.extensions.join(", ");
        println!("│ {:<lang_width$} │ {:<ext_width$} │ {:<ver_width$} │",
            info.name,
            exts,
            info.version,
            lang_width = lang_width,
            ext_width = ext_width,
            ver_width = ver_width
        );
    }

    println!("{}", border_bottom);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_alignment() {
        // Just ensure function runs without panic and language metadata is present
        print_supported_languages();
        let langs = supported_languages();
        assert!(!langs.is_empty());
        assert!(langs.iter().any(|l| l.name == "Rust"));
    }
}
