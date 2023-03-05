use crate::token::{Kind, Token};
use std::str::Chars;

struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    fn read_next_line(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            match c {
                '+' => return Kind::Plus,
                _ => {}
            }
        }
        Kind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_line();
        let end = self.offset();
        Token { start, end, kind }
    }

    fn offset(&mut self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn match_keyword(&self, ident: &str) -> Kind {
        // all keywords are 1 <= length <= 10
        if ident.len() == 1 || ident.len() > 10 {
            return Kind::Identifier;
        }
        match ident {
            "if" => Kind::If,
            "while" => Kind::While,
            "for" => Kind::For,
            _ => Kind::Identifier
        }
    }
}
