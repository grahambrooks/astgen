use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use crate::error::{AstgenError, Result};

#[derive(Parser, Debug)]
#[command(
    color = clap::ColorChoice::Auto,
    author = "Graham Brooks",
    version = crate::VERSION,
    about = "Generate Abstract Syntax Trees from source code using Tree-sitter",
)]
pub struct Args {
    #[arg(value_name = "FILES")]
    pub files: Vec<PathBuf>,
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
    #[arg(long)]
    pub truncate: Option<usize>,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub quiet: bool,
    #[arg(long, value_name = "THREADS")]
    pub parallel: Option<usize>,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long, default_value = "10")]
    pub max_file_size: usize,
    #[arg(long)]
    pub follow_links: bool,
    #[arg(long, default_value = "100")]
    pub max_depth: usize,
    #[arg(long)]
    pub list_languages: bool,
    #[arg(short, long, value_name = "CONFIG")]
    pub config: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Json,
    PrettyJson,
    Yaml,
}

pub fn format_output(json_str: &str, format: &OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(json_str.to_string()),
        OutputFormat::PrettyJson => {
            let value: serde_json::Value = serde_json::from_str(json_str)?;
            Ok(serde_json::to_string_pretty(&value)?)
        }
        OutputFormat::Yaml => {
            let value: serde_json::Value = serde_json::from_str(json_str)?;
            serde_yaml::to_string(&value)
                .map_err(|e| AstgenError::SerializationError(format!("YAML serialization failed: {}", e)))
        }
    }
}

impl Args {
    pub fn validate(&self) -> crate::error::Result<()> {
        if let Some(threads) = self.parallel {
            if threads == 0 || threads > 64 {
                return Err(crate::error::AstgenError::InvalidInput(
                    "Thread count must be between 1 and 64".to_string()
                ));
            }
        }
        if self.max_file_size == 0 || self.max_file_size > 1000 {
            return Err(crate::error::AstgenError::InvalidInput(
                "Max file size must be between 1 and 1000 MB".to_string()
            ));
        }
        if self.max_depth == 0 {
            return Err(crate::error::AstgenError::InvalidInput(
                "Max depth must be greater than 0".to_string()
            ));
        }
        Ok(())
    }
}
