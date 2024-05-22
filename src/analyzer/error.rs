use thiserror::Error;

use super::TokenType;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected {expected:?}, but found {found:?} ({pos})")]
    TokenTypeMismatch {
        expected: TokenType,
        found: Option<TokenType>,
        pos: usize,
    },

    #[error("Expected {expected_type:?} {expected_content:?}, but found {found_type:?} {found_content:?} ({pos})")]
    TokenMismatch {
        expected_type: TokenType,
        expected_content: String,
        found_type: TokenType,
        found_content: String,
        pos: usize,
    },

    #[error("Unexpected end of file")]
    UnexpectedEOF,
}
