// T025: GeneratedFile struct - output file containing generated code
use serde::{Deserialize, Serialize};
use super::file_template::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
    pub language: Language,
    pub lines: usize,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub types: Vec<TypeDefinition>,
    pub confidence: f32, // 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub source: String,
    pub items: Vec<String>,
    pub is_type_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub export_type: ExportType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ExportType {
    Default,
    Named,
    Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub kind: TypeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TypeKind {
    Interface,
    Type,
    Enum,
    Class,
}

impl GeneratedFile {
    pub fn new(path: String, content: String, language: Language) -> Self {
        let lines = content.lines().count();
        
        Self {
            path,
            content,
            language,
            lines,
            imports: Vec::new(),
            exports: Vec::new(),
            types: Vec::new(),
            confidence: 0.0,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        
        if self.content.is_empty() {
            return Err("File content cannot be empty".to_string());
        }
        
        if self.lines == 0 {
            return Err("File must have at least one line".to_string());
        }
        
        if self.confidence < 0.0 || self.confidence > 1.0 {
            return Err("Confidence must be between 0.0 and 1.0".to_string());
        }
        
        Ok(())
    }

    pub fn has_imports(&self) -> bool {
        !self.imports.is_empty()
    }

    pub fn has_exports(&self) -> bool {
        !self.exports.is_empty()
    }

    pub fn has_types(&self) -> bool {
        !self.types.is_empty()
    }
}

impl Import {
    pub fn new(source: String, items: Vec<String>) -> Self {
        Self {
            source,
            items,
            is_type_only: false,
        }
    }

    pub fn type_only(source: String, items: Vec<String>) -> Self {
        Self {
            source,
            items,
            is_type_only: true,
        }
    }
}

impl Export {
    pub fn default_export(name: String) -> Self {
        Self {
            name,
            export_type: ExportType::Default,
        }
    }

    pub fn named_export(name: String) -> Self {
        Self {
            name,
            export_type: ExportType::Named,
        }
    }

    pub fn type_export(name: String) -> Self {
        Self {
            name,
            export_type: ExportType::Type,
        }
    }
}
