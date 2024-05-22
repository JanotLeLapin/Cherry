use core::panic;

use super::{Lexer, Token, TokenType};

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

    pub fn parse(&mut self) -> Option<Vec<Node<'a>>> {
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

        Some(res)
    }

    fn parse_function(&mut self) -> Option<Node<'a>> {
        let mut modifiers = vec![];
        modifiers.append(&mut self.modifiers);

        let ident = if let Some(token) = self.lexer.next() { Box::new(Node::Identifier(self.value(&token))) }
        else { panic!("Expected identifier"); };

        if let Some(token) = self.lexer.next() {
            if self.value(&token) != "(" {
                panic!("Expected open parenthesis");
            }
        } else { panic!("Expected open parenthesis"); }

        let params = vec![];
        let block = Box::new(Node::Block(vec![]));
        while let Some(token) = self.lexer.next() {
            if self.value(&token) == "}" {
                break;
            }
        }

        Some(Node::FunctionDeclaration { modifiers, ident, params, block })
    }
}
