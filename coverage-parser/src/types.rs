use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Json parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalida path format: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;
pub type LineNumberVector = Vec<u32>;
pub type BranchExit = [i32; 2];
