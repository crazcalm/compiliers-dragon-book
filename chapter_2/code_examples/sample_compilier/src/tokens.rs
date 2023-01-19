#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Num,
    Id,
    Plus,
    Minus,
    Equal,
    Semicolon,
    LeftParathesis,
    RightParathesis,
    Multiply,
    Divide,
}

pub type Token = (TokenType, String);