use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlobError {
    #[error("glob_experiment::error::globber: {0}")]
    Globber(String),
    #[error("glob_experiment::error::compiler: {0}")]
    Compiler(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Cli(String),
}

pub type Result<T> = std::result::Result<T, GlobError>;

impl GlobError {
    pub fn cli<T: Into<String>>(msg: T) -> Self {
        GlobError::Cli(msg.into())
    }
}
