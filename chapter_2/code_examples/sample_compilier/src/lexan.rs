use crate::feed::Feed;
use crate::symbols::SymbolTableTrait;
use crate::tokens::{Token, TokenType};


pub fn lexan(mut input_feed: impl Feed, _symbol_table: impl SymbolTableTrait) -> Vec<Token> {
    let mut results = Vec::new();
    let mut lineno = 1;

    loop {
        let mut character = match input_feed.get_char() {
            Some(input) => input,
            None => break,
        };

        if character.eq(&' ') || character.eq(&'\t') {
            continue;
        } else if character.eq(&'\n') {
            lineno += 1;
            continue;
        } else if character.is_ascii_digit() {
            let mut tempt = character.to_string();

            loop {
                match input_feed.get_char() {
                    Some(next_char) => character = next_char,
                    None => break,
                };

                if character.is_ascii_digit() {
                    tempt.push(character);
                } else {
                    input_feed.unget_char().unwrap();
                    break;
                }
            }

            results.push((TokenType::Num, tempt));
        } else if character.is_alphabetic() {
            let mut tempt = character.to_string();

            loop {
                match input_feed.get_char() {
                    Some(next_char) => character = next_char,
                    None => break,
                };

                if character.is_alphanumeric() {
                    tempt.push(character);
                } else {
                    input_feed.unget_char().unwrap();
                    break;
                }
            }
            results.push((TokenType::Id, tempt));
        } else if character.eq(&'-') {
            results.push((TokenType::Minus, character.to_string()));
        } else if character.eq(&'+') {
            results.push((TokenType::Plus, character.to_string()));
        } else if character.eq(&'*') {
            results.push((TokenType::Multiply, character.to_string()));
        } else if character.eq(&'/') {
            results.push((TokenType::Divide, character.to_string()));
        } else if character.eq(&'(') {
            results.push((TokenType::LeftParathesis, character.to_string()));
        } else if character.eq(&')') {
            results.push((TokenType::RightParathesis, character.to_string()));
        } else if character.eq(&'=') {
            results.push((TokenType::Equal, character.to_string()));
        } else if character.eq(&';') {
            results.push((TokenType::Semicolon, character.to_string()));
        }
    }
    results
}