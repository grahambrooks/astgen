use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::cli_types::{Args, OutputFormat};
use crate::error::Result;

pub struct OutputWriter {
    writer: Box<dyn Write + Send>,
}

impl OutputWriter {
    pub fn new(args: &Args) -> Result<Self> {
        let writer: Box<dyn Write + Send> = match &args.output {
            Some(path) => {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)?;
                Box::new(file)
            }
            None => Box::new(io::stdout()),
        };
        
        Ok(OutputWriter { writer })
    }
    
    pub fn write_result(&mut self, content: &str) -> Result<()> {
        writeln!(self.writer, "{}", content)?;
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

pub fn format_summary(success_count: usize, error_count: usize, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Json => {
            serde_json::json!({
                "summary": {
                    "files_processed": success_count,
                    "errors": error_count,
                    "total": success_count + error_count
                }
            }).to_string()
        }
        OutputFormat::PrettyJson => {
            serde_json::to_string_pretty(&serde_json::json!({
                "summary": {
                    "files_processed": success_count,
                    "errors": error_count,
                    "total": success_count + error_count
                }
            })).unwrap_or_default()
        }
        OutputFormat::Yaml => {
            format!(
                "summary:\n  files_processed: {}\n  errors: {}\n  total: {}",
                success_count,
                error_count,
                success_count + error_count
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_format_summary_json() {
        let summary = format_summary(5, 2, &OutputFormat::Json);
        let json: serde_json::Value = serde_json::from_str(&summary).unwrap();
        assert_eq!(json["summary"]["files_processed"], 5);
        assert_eq!(json["summary"]["errors"], 2);
        assert_eq!(json["summary"]["total"], 7);
    }

    #[test]
    fn test_format_summary_yaml() {
        let summary = format_summary(3, 1, &OutputFormat::Yaml);
        assert!(summary.contains("files_processed: 3"));
        assert!(summary.contains("errors: 1"));
        assert!(summary.contains("total: 4"));
    }

    #[test]
    fn test_output_writer_stdout() {
        let args = Args {
            files: vec![],
            format: OutputFormat::Json,
            truncate: None,
            verbose: false,
            quiet: false,
            parallel: None,
            dry_run: false,
            max_file_size: 10,
            follow_links: false,
            max_depth: 100,
            list_languages: false,
            config: None,
            include: vec![],
            exclude: vec![],
            output: None,
            progress: false,
        };
        
        let writer = OutputWriter::new(&args);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_output_writer_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let args = Args {
            files: vec![],
            format: OutputFormat::Json,
            truncate: None,
            verbose: false,
            quiet: false,
            parallel: None,
            dry_run: false,
            max_file_size: 10,
            follow_links: false,
            max_depth: 100,
            list_languages: false,
            config: None,
            include: vec![],
            exclude: vec![],
            output: Some(temp_file.path().to_path_buf()),
            progress: false,
        };
        
        let writer = OutputWriter::new(&args);
        assert!(writer.is_ok());
    }
}