// T024: FileTemplate struct - specification for a file to be generated
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTemplate {
    pub path: String,
    pub purpose: String,
    pub estimated_lines: usize,
    pub language: Language,
    pub dependencies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_hints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Language {
    TypeScript,
    JavaScript,
    Rust,
    Python,
    CSS,
    HTML,
    JSON,
    Markdown,
    YAML,
    TOML,
}

impl Language {
    pub fn file_extension(&self) -> &str {
        match self {
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Rust => "rs",
            Language::Python => "py",
            Language::CSS => "css",
            Language::HTML => "html",
            Language::JSON => "json",
            Language::Markdown => "md",
            Language::YAML => "yaml",
            Language::TOML => "toml",
        }
    }

    pub fn from_path(path: &str) -> Option<Self> {
        let ext = path.split('.').last()?;
        match ext {
            "ts" | "tsx" => Some(Language::TypeScript),
            "js" | "jsx" => Some(Language::JavaScript),
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            "css" => Some(Language::CSS),
            "html" => Some(Language::HTML),
            "json" => Some(Language::JSON),
            "md" => Some(Language::Markdown),
            "yaml" | "yml" => Some(Language::YAML),
            "toml" => Some(Language::TOML),
            _ => None,
        }
    }
}

impl FileTemplate {
    pub fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        
        if self.purpose.is_empty() {
            return Err("File purpose cannot be empty".to_string());
        }
        
        if self.estimated_lines == 0 {
            return Err("Estimated lines must be greater than 0".to_string());
        }
        
        // Check for circular dependencies
        if self.dependencies.contains(&self.path) {
            return Err(format!("File {} has circular dependency on itself", self.path));
        }
        
        Ok(())
    }

    pub fn is_independent(&self) -> bool {
        self.dependencies.is_empty()
    }
}
