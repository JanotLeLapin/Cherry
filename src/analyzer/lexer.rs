#[derive(Debug, PartialEq, Eq)]
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

pub struct Lexer<'a> {
    src: &'a str,
    chars: std::str::Chars<'a>,
    current: Option<char>,
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let current = chars.next();
        Lexer {
            src,
            chars,
            current,
            idx: 0,
        }
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.current = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if c.is_whitespace() { self.advance(); }
            else { break; }
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.idx;
        while let Some(c) = self.current {
            if !c.is_ascii_alphanumeric() && c != '_' { break; }
            self.advance();
        }
        let end = self.idx;

        let token_type = match &self.src[start..end] {
             "pub" | "var" | "val" | "fn" | "return" | "true" | "false" => TokenType::Keyword,
            _ => TokenType::Identifier,
        };

        Token { token_type, start, end }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.idx;
        while let Some(c) = self.current {
            if !c.is_ascii_digit() && c != '.' { break; }
            self.advance();
        }

        Token { token_type: TokenType::Literal, start, end: self.idx }
    }

    fn lex_string(&mut self) -> Token {
        let start = self.idx;
        self.advance();
        while let Some(c) = self.current {
            if c == '"' { break; }
            else if c == '\\' { self.advance() }
            self.advance();
        }
        self.advance();

        Token { token_type: TokenType::Literal, start, end: self.idx }
    }

    fn lex_operator(&mut self) -> Token {
        let start = self.idx;
        while let Some(c) = self.current {
            match c {
                '+' | '-' | '=' | '<' | '>' => { self.advance(); },
                _ => { break; }
            }
        }

        Token { token_type: TokenType::Operator, start, end: self.idx }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if let Some(c) = self.current {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.lex_identifier_or_keyword()),
                '0'..='9' | '.' => return Some(self.lex_number()),
                '"' => return Some(self.lex_string()),
                '+' | '-' | '*' | '=' | '<' | '>' | '!' => return Some(self.lex_operator()),
                '(' | ')' | '{' | '}' | ',' | ':' | ';' => {
                    let start = self.idx;
                    self.advance();
                    return Some(Token { token_type: TokenType::Punctuator, start, end: self.idx })
                }
                _ => return None,
            }
        }

        None
    }
}
