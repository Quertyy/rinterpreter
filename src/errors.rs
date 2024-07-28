

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Invalid token: {0}")]
    InvalidToken(char),
}
