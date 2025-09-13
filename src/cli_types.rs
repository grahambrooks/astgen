use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use crate::error::{AstgenError, Result};

#[derive(Parser, Debug)]
#[command(
    color = clap::ColorChoice::Auto,
    author = "Graham Brooks",
    version = crate::VERSION,
    about = "Generate Abstract Syntax Trees from source code using Tree-sitter",
    long_about = "astgen parses source code files using Tree-sitter grammars and outputs ASTs in JSON format.\n\nSupported languages: Rust, Java, C#, Go, Python, TypeScript, JavaScript, Ruby"
)]
pub struct Args {
    /// Input files or directories to process
    #[arg(value_name = "FILES", help = "Files or directories to parse")]
    pub files: Vec<PathBuf>,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "json", help = "Output format")]
    pub format: OutputFormat,
    
    /// Truncate output to specified length
    #[arg(long, help = "Truncate JSON output to specified number of characters")]
    pub truncate: Option<usize>,
    
    /// Enable verbose output
    #[arg(short, long, help = "Show detailed processing information")]
    pub verbose: bool,
    
    /// Suppress all output except results
    #[arg(short, long, help = "Suppress progress and warning messages")]
    pub quiet: bool,
    
    /// Number of parallel threads
    #[arg(long, value_name = "THREADS", help = "Number of threads for parallel processing")]
    pub parallel: Option<usize>,
    
    /// Show what would be processed without actually parsing
    #[arg(long, help = "Show files that would be processed without parsing them")]
    pub dry_run: bool,
    
    /// Maximum file size in MB
    #[arg(long, default_value = "10", help = "Maximum file size to process (in MB)")]
    pub max_file_size: usize,
    
    /// Follow symbolic links
    #[arg(long, help = "Follow symbolic links when traversing directories")]
    pub follow_links: bool,
    
    /// Maximum directory traversal depth
    #[arg(long, default_value = "100", help = "Maximum depth for directory traversal")]
    pub max_depth: usize,
    
    /// List supported languages and exit
    #[arg(long, help = "Display supported languages and their versions")]
    pub list_languages: bool,
    
    /// Configuration file path
    #[arg(short, long, value_name = "CONFIG", help = "Path to configuration file")]
    pub config: Option<PathBuf>,
    
    /// Include files matching pattern
    #[arg(long, value_name = "PATTERN", help = "Include files matching glob pattern (can be used multiple times)")]
    pub include: Vec<String>,
    
    /// Exclude files matching pattern
    #[arg(long, value_name = "PATTERN", help = "Exclude files matching glob pattern (can be used multiple times)")]
    pub exclude: Vec<String>,
    
    /// Output file path
    #[arg(short, long, value_name = "FILE", help = "Write output to file instead of stdout")]
    pub output: Option<PathBuf>,
    
    /// Show progress bar
    #[arg(long, help = "Show progress bar for directory processing")]
    pub progress: bool,
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
        // Validate thread count
        if let Some(threads) = self.parallel {
            if threads == 0 {
                return Err(crate::error::AstgenError::InvalidInput(
                    "Thread count must be at least 1. Try using --parallel 1 or omit the flag to use default.".to_string()
                ));
            }
            if threads > 64 {
                return Err(crate::error::AstgenError::InvalidInput(
                    "Thread count cannot exceed 64. Try using a smaller number like --parallel 8.".to_string()
                ));
            }
        }
        
        // Validate file size limit
        if self.max_file_size == 0 {
            return Err(crate::error::AstgenError::InvalidInput(
                "Max file size must be at least 1 MB. Try using --max-file-size 1.".to_string()
            ));
        }
        if self.max_file_size > 1000 {
            return Err(crate::error::AstgenError::InvalidInput(
                "Max file size cannot exceed 1000 MB. Try using a smaller limit like --max-file-size 100.".to_string()
            ));
        }
        
        // Validate max depth
        if self.max_depth == 0 {
            return Err(crate::error::AstgenError::InvalidInput(
                "Max depth must be at least 1. Try using --max-depth 1 or omit the flag to use default.".to_string()
            ));
        }
        
        // Validate conflicting flags
        if self.verbose && self.quiet {
            return Err(crate::error::AstgenError::InvalidInput(
                "Cannot use both --verbose and --quiet flags together. Choose one or neither.".to_string()
            ));
        }
        
        // Validate output file path
        if let Some(output_path) = &self.output {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    return Err(crate::error::AstgenError::InvalidInput(
                        format!("Output directory does not exist: {}. Create the directory first.", parent.display())
                    ));
                }
            }
        }
        
        // Validate include/exclude patterns
        for pattern in &self.include {
            if pattern.is_empty() {
                return Err(crate::error::AstgenError::InvalidInput(
                    "Include pattern cannot be empty. Use a valid glob pattern like '*.rs'.".to_string()
                ));
            }
        }
        
        for pattern in &self.exclude {
            if pattern.is_empty() {
                return Err(crate::error::AstgenError::InvalidInput(
                    "Exclude pattern cannot be empty. Use a valid glob pattern like 'target/*'.".to_string()
                ));
            }
        }
        
        Ok(())
    }
}
