use lorecore::errors::LoreCoreError;

#[derive(Debug, Clone)]
pub enum LoreGuiError {
    FileError(String),
    InputError(String),
    NoDatabase,
    LoreCoreError(LoreCoreError),
}

impl ToString for LoreGuiError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
