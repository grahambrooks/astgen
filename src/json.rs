use serde::Serialize;
use tree_sitter::Node;

#[derive(Serialize)]
#[serde(rename = "node")]
pub(crate) struct JsonNode {
    kind: String,
    start_byte: usize,
    end_byte: usize,
    children: Option<Vec<JsonNode>>,
    text: Option<String>,
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
        text: if text_value.is_empty() { None } else { Some(text_value) },
        children: if children.is_empty() { None } else { Some(children) },
    }
}

