#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("[line {0}] Error: Unexpected character \"{1}\".")]
    InvalidToken(u64, char),
    #[error("[line {0}] Error: Unterminated string.")]
    UnterminatedString(u64),
    #[error("[line {0}] Error: Unterminated block comment.")]
    UnterminatedBlockComment(u64),
}

#[derive(Debug, thiserror::Error)]
pub enum LoxError {

}
