use crate::token_type::TokenType;
use crate::token::Token;

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

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Expect '{0:#?}' after expression")]
    ExpectedToken(TokenType),
    #[error("[line {0}] Error at '{1}': Expect ')' after expression.")]
    At(u64, String),
    #[error("[line {0}] Error at end: Expect ')' after expression.")]
    Eof(u64)
}
