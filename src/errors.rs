use lorecore::errors::LoreCoreError;

#[derive(Debug, Clone)]
pub enum LoreGuiError {
    FileError(String),
    InputError(String),
    NoDatabase,
    MultipleResults,
    LoreCoreError(LoreCoreError),
}

impl ToString for LoreGuiError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<LoreCoreError> for LoreGuiError {
    fn from(error: LoreCoreError) -> Self {
        LoreGuiError::LoreCoreError(error)
    }
}
