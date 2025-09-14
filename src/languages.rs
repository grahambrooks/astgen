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
// New high / medium priority languages
static C_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static CPP_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static BASH_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static JSON_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static HTML_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static CSS_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static YAML_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static SWIFT_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static SCALA_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static LUA_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static HCL_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
static GRAPHQL_LANGUAGE: OnceLock<tree_sitter::Language> = OnceLock::new();
/// Return static metadata for all supported languages.
pub fn supported_languages() -> Vec<LanguageInfo> {
    vec![
        LanguageInfo {
            name: "Rust",
            extensions: &[".rs"],
            version: TREE_SITTER_RUST_VERSION,
        },
        LanguageInfo {
            name: "Java",
            extensions: &[".java"],
            version: TREE_SITTER_JAVA_VERSION,
        },
        LanguageInfo {
            name: "C#",
            extensions: &[".cs"],
            version: TREE_SITTER_C_SHARP_VERSION,
        },
        LanguageInfo {
            name: "Go",
            extensions: &[".go"],
            version: TREE_SITTER_GO_VERSION,
        },
        LanguageInfo {
            name: "Python",
            extensions: &[".py"],
            version: TREE_SITTER_PYTHON_VERSION,
        },
        LanguageInfo {
            name: "TypeScript",
            extensions: &[".ts"],
            version: TREE_SITTER_TYPESCRIPT_VERSION,
        },
        LanguageInfo {
            name: "TSX",
            extensions: &[".tsx"],
            version: TREE_SITTER_TYPESCRIPT_VERSION,
        },
        LanguageInfo {
            name: "JavaScript",
            extensions: &[".js"],
            version: TREE_SITTER_JAVASCRIPT_VERSION,
        },
        LanguageInfo {
            name: "Ruby",
            extensions: &[".rb"],
            version: TREE_SITTER_RUBY_VERSION,
        },
        // High priority new
        LanguageInfo {
            name: "C",
            extensions: &[".c", ".h"],
            version: TREE_SITTER_C_VERSION,
        },
        LanguageInfo {
            name: "C++",
            extensions: &[".cpp", ".cc", ".cxx", ".hpp", ".hh"],
            version: TREE_SITTER_CPP_VERSION,
        },
        LanguageInfo {
            name: "Bash",
            extensions: &[".sh", ".bash"],
            version: TREE_SITTER_BASH_VERSION,
        },
        LanguageInfo {
            name: "JSON",
            extensions: &[".json"],
            version: TREE_SITTER_JSON_VERSION,
        },
        LanguageInfo {
            name: "HTML",
            extensions: &[".html", ".htm"],
            version: TREE_SITTER_HTML_VERSION,
        },
        LanguageInfo {
            name: "CSS",
            extensions: &[".css"],
            version: TREE_SITTER_CSS_VERSION,
        },
        LanguageInfo {
            name: "YAML",
            extensions: &[".yml", ".yaml"],
            version: TREE_SITTER_YAML_VERSION,
        },
        // Medium priority new
        LanguageInfo {
            name: "Swift",
            extensions: &[".swift"],
            version: TREE_SITTER_SWIFT_VERSION,
        },
        LanguageInfo {
            name: "Scala",
            extensions: &[".scala"],
            version: TREE_SITTER_SCALA_VERSION,
        },
        LanguageInfo {
            name: "Lua",
            extensions: &[".lua"],
            version: TREE_SITTER_LUA_VERSION,
        },
        LanguageInfo {
            name: "HCL",
            extensions: &[".hcl", ".tf", ".tfvars"],
            version: TREE_SITTER_HCL_VERSION,
        },
        LanguageInfo {
            name: "GraphQL",
            extensions: &[".graphql", ".gql"],
            version: TREE_SITTER_GRAPHQL_VERSION,
        },
    ]
}

/// Create the encodings structure (file extension regex -> language mapping)
pub fn create_encodings() -> encodings::Encodings<'static> {
    let rust_lang = RUST_LANGUAGE.get_or_init(|| tree_sitter_rust::LANGUAGE.into());
    let java_lang = JAVA_LANGUAGE.get_or_init(|| tree_sitter_java::LANGUAGE.into());
    let csharp_lang = CSHARP_LANGUAGE.get_or_init(|| tree_sitter_c_sharp::LANGUAGE.into());
    let go_lang = GO_LANGUAGE.get_or_init(|| tree_sitter_go::LANGUAGE.into());
    let python_lang = PYTHON_LANGUAGE.get_or_init(|| tree_sitter_python::LANGUAGE.into());
    let typescript_lang =
        TYPESCRIPT_LANGUAGE.get_or_init(|| tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
    let tsx_lang = TSX_LANGUAGE.get_or_init(|| tree_sitter_typescript::LANGUAGE_TSX.into());
    let javascript_lang =
        JAVASCRIPT_LANGUAGE.get_or_init(|| tree_sitter_javascript::LANGUAGE.into());
    let ruby_lang = RUBY_LANGUAGE.get_or_init(|| tree_sitter_ruby::LANGUAGE.into());

    // New languages (initialize lazily)
    let c_lang = C_LANGUAGE.get_or_init(|| tree_sitter_c::LANGUAGE.into());
    let cpp_lang = CPP_LANGUAGE.get_or_init(|| tree_sitter_cpp::LANGUAGE.into());
    let bash_lang = BASH_LANGUAGE.get_or_init(|| tree_sitter_bash::LANGUAGE.into());
    let json_lang = JSON_LANGUAGE.get_or_init(|| tree_sitter_json::LANGUAGE.into());
    let html_lang = HTML_LANGUAGE.get_or_init(|| tree_sitter_html::LANGUAGE.into());
    let css_lang = CSS_LANGUAGE.get_or_init(|| tree_sitter_css::LANGUAGE.into());
    let yaml_lang = YAML_LANGUAGE.get_or_init(|| tree_sitter_yaml::LANGUAGE.into());
    let swift_lang = SWIFT_LANGUAGE.get_or_init(|| tree_sitter_swift::LANGUAGE.into());
    let scala_lang = SCALA_LANGUAGE.get_or_init(|| tree_sitter_scala::LANGUAGE.into());
    let lua_lang = LUA_LANGUAGE.get_or_init(|| tree_sitter_lua::LANGUAGE.into());
    let hcl_lang = HCL_LANGUAGE.get_or_init(|| tree_sitter_hcl::LANGUAGE.into());
    let graphql_lang = GRAPHQL_LANGUAGE.get_or_init(|| tree_sitter_graphql::LANGUAGE.into());

    let mut enc = encodings::Encodings::new();
    enc.add("rs$", rust_lang, "Rust")
        .add("java$", java_lang, "Java")
        .add("cs$", csharp_lang, "C#")
        .add("go$", go_lang, "Go")
        .add("py$", python_lang, "Python")
        .add("ts$", typescript_lang, "TypeScript")
        .add("tsx$", tsx_lang, "TSX")
        .add("js$", javascript_lang, "JavaScript")
        .add("rb$", ruby_lang, "Ruby")
        // High priority new encodings
        .add("c$", c_lang, "C")
        .add("h$", c_lang, "C")
        .add("(cpp|cc|cxx)$", cpp_lang, "C++")
        .add("(hpp|hh|hxx)$", cpp_lang, "C++")
        .add("(sh|bash)$", bash_lang, "Bash")
        .add("json$", json_lang, "JSON")
        .add("(html|htm)$", html_lang, "HTML")
        .add("css$", css_lang, "CSS")
        .add("(ya?ml)$", yaml_lang, "YAML")
        // Medium priority new encodings
        .add("swift$", swift_lang, "Swift")
        .add("scala$", scala_lang, "Scala")
        .add("lua$", lua_lang, "Lua")
        .add("(hcl|tf|tfvars)$", hcl_lang, "HCL")
        .add("(graphql|gql)$", graphql_lang, "GraphQL")
        ;

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

    let lang_width = langs
        .iter()
        .map(|l| l.name.len())
        .chain([header_lang.len()])
        .max()
        .unwrap();
    let ext_width = langs
        .iter()
        .map(|l| l.extensions.join(", ").len())
        .chain([header_ext.len()])
        .max()
        .unwrap();
    let ver_width = langs
        .iter()
        .map(|l| l.version.len())
        .chain([header_ver.len()])
        .max()
        .unwrap();

    let border_top = format!(
        "┌{}┬{}┬{}┐",
        "─".repeat(lang_width + 2),
        "─".repeat(ext_width + 2),
        "─".repeat(ver_width + 2)
    );
    let border_mid = format!(
        "├{}┼{}┼{}┤",
        "─".repeat(lang_width + 2),
        "─".repeat(ext_width + 2),
        "─".repeat(ver_width + 2)
    );
    let border_bottom = format!(
        "└{}┴{}┴{}┘",
        "─".repeat(lang_width + 2),
        "─".repeat(ext_width + 2),
        "─".repeat(ver_width + 2)
    );

    println!("Supported Languages:");
    println!("{}", border_top);
    println!(
        "│ {:<lang_width$} │ {:<ext_width$} │ {:<ver_width$} │",
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
        println!(
            "│ {:<lang_width$} │ {:<ext_width$} │ {:<ver_width$} │",
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
        // Updated expectations with Dockerfile listed (listing only)
        print_supported_languages();
        let langs = supported_languages();
        assert!(langs.iter().any(|l| l.name == "GraphQL"));
        assert!(langs.iter().any(|l| l.name == "C++"));
        assert!(langs.iter().any(|l| l.name == "Dockerfile"));
    }
}
