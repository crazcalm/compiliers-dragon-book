use crate::tokens::{Token, TokenType};

pub trait SymbolTableTrait {
    fn lookup(self, value: String) -> Option<Token>;
    fn insert(&mut self, token_type: TokenType, value: String);
}

pub struct SymbolTable {
    inner: Vec<Token>,
}

impl SymbolTableTrait for SymbolTable {
    fn lookup(self, value: String) -> Option<Token> {
        for (token_type, token_value) in self.inner.iter() {
            if value.eq(token_value) {
                return Some((token_type.clone(), value));
            }
        }

        None
    }

    fn insert(&mut self, token_type: TokenType, value: String) {
        self.inner.push((token_type, value))
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { inner: Vec::new() }
    }
}