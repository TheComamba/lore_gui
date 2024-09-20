use std::fmt::Display;

use lorecore::errors::LoreCoreError;

#[derive(Debug, Clone)]
pub enum LoreGuiError {
    FileError(String),
    InputError(String),
    NoDatabase,
    MultipleResults,
    LoreCoreError(LoreCoreError),
}

impl Display for LoreGuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoreGuiError::FileError(msg) => write!(f, "File error: {}", msg),
            LoreGuiError::InputError(msg) => write!(f, "Input error: {}", msg),
            LoreGuiError::NoDatabase => write!(f, "No database"),
            LoreGuiError::MultipleResults => write!(f, "Multiple results"),
            LoreGuiError::LoreCoreError(error) => write!(f, "Lore core error: {}", error),
        }
    }
}

impl From<LoreCoreError> for LoreGuiError {
    fn from(error: LoreCoreError) -> Self {
        LoreGuiError::LoreCoreError(error)
    }
}
