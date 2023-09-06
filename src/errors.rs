#[derive(Debug, Clone)]
pub enum LoreGuiError {
    FileError(String),
    InputError(String),
}

impl ToString for LoreGuiError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
