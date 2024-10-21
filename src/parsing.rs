use crate::encoding::Encoding;
use crate::json::JsonNode;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use tree_sitter::{Language, Parser};

pub fn parse_file(path: PathBuf, encoding: &Encoding, truncate: Option<usize>) -> bool {
    use std::panic;
    let result = panic::catch_unwind(|| {
        let filepath = path.to_str().unwrap();
        let json_tree = build_parse_tree(path.clone(), encoding.language);

        let wrapped_json = json!({
            "version": "astgen-0.1",
            "filename": filepath,
            "language": encoding.name,
            "ast": json_tree
        });

        let json_output = serde_json::to_string(&wrapped_json).unwrap();

        match truncate {
            Some(len) => {
                if json_output.len() > len {
                    println!("{}", &json_output[..len]);
                } else {
                    println!("{}", &json_output);
                }
            }
            None => {
                println!("{}", &json_output);
            }
        }
    });

    if result.is_err() {
        println!("A panic occurred while parsing the file.");
        return false;
    }

    true
}

fn build_parse_tree(path: PathBuf, lang: &Language) -> JsonNode {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let mut parser = Parser::new();
    parser.set_language(lang).expect("Error loading grammar");
    let code = file.as_str();
    let tree = parser.parse(code, None).unwrap();
    let root_node = tree.root_node();
    crate::json::node_to_json(code, root_node)
}
