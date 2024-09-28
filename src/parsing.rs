use std::path::PathBuf;
use tree_sitter::Language;

const OUTPUT_LENGTH: usize = 200;

pub fn parse_file(path: PathBuf, lang: &Language) -> bool {
    use std::{fs, panic};
    use tree_sitter::Parser;

    print!("Parsing file: {}: ", path.to_str().unwrap());
    let result = panic::catch_unwind(|| {
        let file = fs::read_to_string(path).expect("Unable to read file");
        let mut parser = Parser::new();
        parser.set_language(lang).expect("Error loading grammar");
        let code = file.as_str();
        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();
        let json_tree = crate::json::node_to_json(code, root_node);
        let json_output = serde_json::to_string(&json_tree).unwrap();
        if json_output.len() > OUTPUT_LENGTH {
            println!("{}", &json_output[..OUTPUT_LENGTH]);
        } else {
            println!("{}", &json_output);
        }
    });

    if result.is_err() {
        println!("A panic occurred while parsing the file.");
        return false
    }
    
    true
}