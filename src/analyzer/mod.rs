mod lexer;
mod parser;

pub use lexer::Lexer;

#[derive(Debug)]
pub enum TokenType {
    Keyword,
    Identifier,
    Literal,
    Operator,
    Punctuator,
    Comment,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize
}
