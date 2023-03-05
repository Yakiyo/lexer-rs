use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
    // pub value: TokenValue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof,
    Plus,
    If,
    While,
    For,
    Identifier
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(Atom),
}
