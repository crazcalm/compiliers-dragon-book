use crate::feed::Feed;
use crate::symbols::SymbolTableTrait;
use crate::tokens::{Token, TokenType};


pub fn lexan(mut input_feed: impl Feed, _SymbolTable: impl SymbolTableTrait) -> Vec<Token> {
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

            results.push((TokenType::NUM, tempt));
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
            results.push((TokenType::ID, tempt));
        } else if character.eq(&'-') {
            results.push((TokenType::MINUS, character.to_string()));
        } else if character.eq(&'+') {
            results.push((TokenType::PLUS, character.to_string()));
        } else if character.eq(&'*') {
            results.push((TokenType::MULTIPLY, character.to_string()));
        } else if character.eq(&'/') {
            results.push((TokenType::DIVIDE, character.to_string()));
        } else if character.eq(&'(') {
            results.push((TokenType::LEFT_PARATHESIS, character.to_string()));
        } else if character.eq(&')') {
            results.push((TokenType::RIGHT_PARATHESIS, character.to_string()));
        } else if character.eq(&'=') {
            results.push((TokenType::EQUAL, character.to_string()));
        } else if character.eq(&';') {
            results.push((TokenType::SEMICOLON, character.to_string()));
        }
    }

    println!("{:?}", results);
    results
}