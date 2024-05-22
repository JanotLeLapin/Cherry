use super::{Lexer, Token, TokenType};
use super::error::ParseError;

#[derive(Debug)]
pub enum Node<'a> {
    PublicKeyword,

    Identifier(&'a str),

    StringLiteral(&'a str),
    I8Literal(i8),
    I16Literal(i16),
    I32Literal(i32),
    I64Literal(i64),
    BoolLiteral(bool),

    Parameter {
        name: Box<Node<'a>>,
        ty: Option<Box<Node<'a>>>,
    },
    FunctionDeclaration {
        modifiers: Vec<Node<'a>>,
        ident: Box<Node<'a>>,
        params: Vec<Node<'a>>,
        block: Box<Node<'a>>,
    },

    Block(Vec<Node<'a>>),
    VariableDeclaration {
        modifiers: Vec<Node<'a>>,
        ident: Box<Node<'a>>,
        value: Box<Node<'a>>,
    },
    CallExpression {
        ident: Box<Node<'a>>,
        params: Vec<Node<'a>>,
    },
    ReturnStatement(Box<Node<'a>>),
}

pub struct Parser<'a> {
    src: &'a str,
    lexer: Lexer<'a>,
    modifiers: Vec<Node<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str, lexer: Lexer<'a>) -> Self {
        Self {
            src,
            lexer,
            modifiers: vec![],
        }
    }

    fn value(&self, token: &Token) -> &'a str {
        &self.src[token.start..token.end]
    }

    pub fn parse(&mut self) -> Result<Vec<Node<'a>>, ParseError> {
        let mut res = vec![];
        while let Some(token) = self.lexer.next() {
            match token.token_type {
                TokenType::Keyword => match self.value(&token) {
                    "pub" => self.modifiers.push(Node::PublicKeyword),
                    "fn" => res.push(self.parse_function()?),
                    kw => unimplemented!("Unknown keyword {kw:?}"),
                },
                tt => panic!("Unexpected {tt:?}"),
            }
        }

        Ok(res)
    }

    fn parse_function(&mut self) -> Result<Node<'a>, ParseError> {
        let mut modifiers = vec![];
        modifiers.append(&mut self.modifiers);

        let ident = Box::new(self.parse_identifier()?);
        let _ = self.expect(TokenType::Punctuator, "(")?;

        let params = vec![];
        let block = Box::new(Node::Block(vec![]));
        while let Some(token) = self.lexer.next() {
            if self.value(&token) == "}" {
                break;
            }
        }

        Ok(Node::FunctionDeclaration { modifiers, ident, params, block })
    }

    fn parse_identifier(&mut self) -> Result<Node<'a>, ParseError> {
        if let Some(token) = self.lexer.next() {
            match token.token_type {
                TokenType::Identifier => Ok(Node::Identifier(self.value(&token))),
                tt => Err(ParseError::TokenTypeMismatch { expected: TokenType::Identifier, found: Some(tt), pos: token.start })
            }
        } else { Err(ParseError::TokenTypeMismatch { expected: TokenType::Identifier, found: None, pos: 0 }) }
    }

    fn expect_type(&mut self, expected: TokenType) -> Result<(), ParseError> {
        if let Some(token) = self.lexer.next() {
            if token.token_type != expected { Err(ParseError::TokenTypeMismatch { expected, found: Some(token.token_type), pos: token.start }) }
            else { Ok(()) }
        } else { Err(ParseError::UnexpectedEOF) }
    }

    fn expect(&mut self, expected_type: TokenType, expected_content: &str) -> Result<(), ParseError> {
        if let Some(token) = self.lexer.next() {
            let found_content = self.value(&token);
            let found_type = token.token_type;

            if expected_type != found_type || expected_content != found_content {
                Err(ParseError::TokenMismatch { expected_type, expected_content: expected_content.to_string(), found_type, found_content: found_content.to_string(), pos: token.start })
            } else { Ok(()) }
        } else { Err(ParseError::UnexpectedEOF) }
    }
}
