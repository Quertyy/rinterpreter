

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("[line {0}] Error: Unexpected character \"{0}\"")]
    InvalidToken(u64, char),
}
