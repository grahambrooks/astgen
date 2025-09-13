use serde::Serialize;
use tree_sitter::Node;

#[derive(Serialize)]
#[serde(rename = "node")]
pub(crate) struct JsonNode {
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub children: Option<Vec<JsonNode>>,
    pub text: Option<String>,
}

pub(crate) fn node_to_json(source_code: &str, node: Node) -> JsonNode {
    let mut children = Vec::new();
    for i in 0..node.child_count() {
        children.push(node_to_json(source_code, node.child(i).unwrap()));
    }
    let text_value = source_code[node.start_byte()..node.end_byte()].to_string();
    JsonNode {
        kind: node.kind().to_string(),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        text: if children.is_empty() && !text_value.is_empty() {
            Some(text_value)
        } else {
            None
        },
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::{Language, Parser};

    fn setup_parser(lang: &Language) -> Parser {
        let mut parser = Parser::new();
        parser.set_language(lang).expect("Error loading grammar");
        parser
    }

    #[test]
    fn test_json_node_serialization() {
        let node = JsonNode {
            kind: "source_file".to_string(),
            start_byte: 0,
            end_byte: 10,
            children: None,
            text: Some("test".to_string()),
        };

        let serialized = serde_json::to_string(&node).unwrap();
        assert!(serialized.contains("source_file"));
        assert!(serialized.contains("test"));
        assert!(serialized.contains("start_byte"));
        assert!(serialized.contains("end_byte"));
    }

    #[test]
    fn test_node_to_json_simple_rust_code() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let mut parser = setup_parser(&rust_language);

        let code = "fn main() {}";
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let json_node = node_to_json(code, root_node);

        assert_eq!(json_node.kind, "source_file");
        assert_eq!(json_node.start_byte, 0);
        assert_eq!(json_node.end_byte, code.len());
        assert!(json_node.children.is_some());
        assert!(json_node.text.is_none()); // Has children, so no text
    }

    #[test]
    fn test_node_to_json_empty_file() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let mut parser = setup_parser(&rust_language);

        let code = "";
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let json_node = node_to_json(code, root_node);

        assert_eq!(json_node.kind, "source_file");
        assert_eq!(json_node.start_byte, 0);
        assert_eq!(json_node.end_byte, 0);
        assert!(json_node.children.is_none());
        assert!(json_node.text.is_none()); // Empty text is not included
    }

    #[test]
    fn test_node_to_json_with_javascript() {
        let js_language = tree_sitter_javascript::LANGUAGE.into();
        let mut parser = setup_parser(&js_language);

        let code = "console.log('hello');";
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let json_node = node_to_json(code, root_node);

        assert_eq!(json_node.kind, "program");
        assert_eq!(json_node.start_byte, 0);
        assert_eq!(json_node.end_byte, code.len());
        assert!(json_node.children.is_some());
    }

    #[test]
    fn test_node_to_json_with_python() {
        let python_language = tree_sitter_python::LANGUAGE.into();
        let mut parser = setup_parser(&python_language);

        let code = "print('hello')";
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let json_node = node_to_json(code, root_node);

        assert_eq!(json_node.kind, "module");
        assert_eq!(json_node.start_byte, 0);
        assert_eq!(json_node.end_byte, code.len());
        assert!(json_node.children.is_some());
    }

    #[test]
    fn test_node_to_json_preserves_byte_positions() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let mut parser = setup_parser(&rust_language);

        let code = "fn test() {}"; // Simple code without leading whitespace
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let json_node = node_to_json(code, root_node);

        assert_eq!(json_node.start_byte, 0);
        assert_eq!(json_node.end_byte, code.len());

        // Check that children preserve correct positions
        if let Some(children) = &json_node.children {
            if !children.is_empty() {
                let first_child = &children[0];
                assert!(first_child.start_byte < first_child.end_byte);
                assert!(first_child.end_byte <= code.len());
                assert!(first_child.start_byte <= code.len());
            }
        }
    }

    #[test]
    fn test_node_to_json_leaf_nodes_have_text() {
        let rust_language = tree_sitter_rust::LANGUAGE.into();
        let mut parser = setup_parser(&rust_language);

        let code = "42";
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let json_node = node_to_json(code, root_node);

        // Walk to find a leaf node with text
        fn find_leaf_with_text(node: &JsonNode) -> Option<&JsonNode> {
            if node.children.is_none() && node.text.is_some() {
                return Some(node);
            } else if let Some(children) = &node.children {
                for child in children {
                    if let Some(leaf) = find_leaf_with_text(child) {
                        return Some(leaf);
                    }
                }
            }
            None
        }

        let leaf = find_leaf_with_text(&json_node);
        if let Some(leaf_node) = leaf {
            assert!(leaf_node.text.is_some());
            assert!(leaf_node.children.is_none());
        }
    }
}
