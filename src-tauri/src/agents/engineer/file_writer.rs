// T050-T051: File output management with atomic writes
use crate::models::GeneratedFile;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use tempfile::NamedTempFile;
use std::io::Write;

pub struct FileWriter {
    output_directory: PathBuf,
}

impl FileWriter {
    pub fn new(output_directory: PathBuf) -> Self {
        Self { output_directory }
    }

    /// T050: Write files atomically to prevent partial writes
    pub fn write_files(&self, files: &[GeneratedFile]) -> Result<Vec<PathBuf>> {
        let mut written_paths = Vec::new();
        
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_directory)?;
        
        for file in files {
            let path = self.write_file_atomic(file)?;
            written_paths.push(path);
        }
        
        Ok(written_paths)
    }

    /// T050: Atomic write using tempfile + rename
    fn write_file_atomic(&self, file: &GeneratedFile) -> Result<PathBuf> {
        let target_path = self.output_directory.join(&file.path);
        
        // Create parent directories
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write to temporary file first
        let mut temp_file = NamedTempFile::new_in(
            target_path.parent().unwrap_or_else(|| Path::new("."))
        )?;
        
        temp_file.write_all(file.content.as_bytes())?;
        temp_file.flush()?;
        
        // Atomic rename (on most filesystems)
        temp_file.persist(&target_path)?;
        
        Ok(target_path)
    }

    /// T051: Preserve only completed files on error
    pub fn write_partial(&self, files: &[GeneratedFile]) -> Result<Vec<PathBuf>> {
        let mut written_paths = Vec::new();
        
        for file in files {
            // Validate file before writing
            if let Err(e) = file.validate() {
                eprintln!("Skipping invalid file {}: {}", file.path, e);
                continue;
            }
            
            match self.write_file_atomic(file) {
                Ok(path) => written_paths.push(path),
                Err(e) => {
                    eprintln!("Failed to write {}: {}", file.path, e);
                    // Continue with other files
                }
            }
        }
        
        Ok(written_paths)
    }

    /// T051: Clean up partial generation
    pub fn cleanup_partial(&self, written_paths: &[PathBuf]) -> Result<()> {
        for path in written_paths {
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    pub fn export_to_directory(&self, files: &[GeneratedFile], target_dir: &Path) -> Result<Vec<PathBuf>> {
        let writer = FileWriter::new(target_dir.to_path_buf());
        writer.write_files(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Language;
    use tempfile::tempdir;

    fn create_test_file() -> GeneratedFile {
        GeneratedFile {
            path: "test/example.ts".to_string(),
            content: "export const test = 42;".to_string(),
            language: Language::TypeScript,
            lines: 1,
            imports: vec![],
            exports: vec![],
            types: vec![],
            confidence: 0.9,
        }
    }

    #[test]
    fn test_write_file_atomic() {
        let temp_dir = tempdir().unwrap();
        let writer = FileWriter::new(temp_dir.path().to_path_buf());
        let file = create_test_file();
        
        let result = writer.write_file_atomic(&file);
        assert!(result.is_ok());
        
        let written_path = result.unwrap();
        assert!(written_path.exists());
        
        let content = fs::read_to_string(&written_path).unwrap();
        assert_eq!(content, file.content);
    }

    #[test]
    fn test_write_multiple_files() {
        let temp_dir = tempdir().unwrap();
        let writer = FileWriter::new(temp_dir.path().to_path_buf());
        
        let files = vec![
            create_test_file(),
            GeneratedFile {
                path: "test/another.ts".to_string(),
                content: "export const another = 100;".to_string(),
                language: Language::TypeScript,
                lines: 1,
                imports: vec![],
                exports: vec![],
                types: vec![],
                confidence: 0.9,
            },
        ];
        
        let result = writer.write_files(&files);
        assert!(result.is_ok());
        
        let paths = result.unwrap();
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().all(|p| p.exists()));
    }
}
