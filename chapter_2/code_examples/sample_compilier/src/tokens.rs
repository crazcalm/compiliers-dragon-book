#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    NUM,
    ID,
    PLUS,
    MINUS,
    EQUAL,
    SEMICOLON,
    LEFT_PARATHESIS,
    RIGHT_PARATHESIS,
    MULTIPLY,
    DIVIDE,
}

pub type Token = (TokenType, String);