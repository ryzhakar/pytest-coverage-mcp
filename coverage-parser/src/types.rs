use thiserror::Error;

const CONTEXT_INSTRUCTION: &str = "Enable `dynamic_context = \"test_function\"` \
in tool.coverage.run section \
of pyproject.toml, \
then run `coverage run -m pytest tests` \
and `coverage json --show-contexts --pretty` \
again to generate the report with contexts.";

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Json parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid path format: {0}")]
    InvalidPath(String),
    #[error(
        "Contexts are not enabled. \
        {CONTEXT_INSTRUCTION}
        "
    )]
    ContextDisabled,
    #[error(
        "Expected context format is \
        is <test_function_name>, \
        but found no matching context.
        {CONTEXT_INSTRUCTION}
        "
    )]
    WrongContextFormat,
}

pub type Result<T> = std::result::Result<T, ParseError>;
pub type LineNumberVector = Vec<u32>;
pub type BranchExit = [i32; 2];
