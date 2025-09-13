use std::fmt;

#[derive(Debug)]
pub enum AstgenError {
    IoError(std::io::Error),
    ParseError(String),
    #[allow(dead_code)]
    LanguageNotSupported(String),
    InvalidInput(String),
    TreeSitterError(tree_sitter::LanguageError),
    SerializationError(String),
}

impl fmt::Display for AstgenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstgenError::IoError(err) => write!(f, "IO error: {}", err),
            AstgenError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AstgenError::LanguageNotSupported(ext) => {
                write!(f, "Language not supported for extension: {}", ext)
            }
            AstgenError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AstgenError::TreeSitterError(err) => write!(f, "Tree-sitter error: {:?}", err),
            AstgenError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for AstgenError {}

impl From<std::io::Error> for AstgenError {
    fn from(error: std::io::Error) -> Self {
        AstgenError::IoError(error)
    }
}

impl From<tree_sitter::LanguageError> for AstgenError {
    fn from(error: tree_sitter::LanguageError) -> Self {
        AstgenError::TreeSitterError(error)
    }
}

impl From<serde_json::Error> for AstgenError {
    fn from(error: serde_json::Error) -> Self {
        AstgenError::SerializationError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AstgenError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_io_error_conversion() {
        let err = io::Error::new(io::ErrorKind::Other, "fail");
        let astgen_err: AstgenError = err.into();
        match astgen_err {
            AstgenError::IoError(_) => {}
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_display_format() {
        let err = AstgenError::ParseError("bad parse".to_string());
        assert_eq!(format!("{}", err), "Parse error: bad parse");
    }
}
