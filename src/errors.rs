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
        format!("{:?}", self).fmt(f)
    }
}

impl From<LoreCoreError> for LoreGuiError {
    fn from(error: LoreCoreError) -> Self {
        LoreGuiError::LoreCoreError(error)
    }
}
