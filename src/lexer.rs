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
}
