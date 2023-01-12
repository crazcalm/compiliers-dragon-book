use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

mod feed;

use crate::feed::Feed;

fn get_input_string(path: &str) -> String {
    let mut result = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut result).unwrap();
    result
}

#[derive(Debug, Clone)]
pub enum TokenType {
    NUM,
    ID,
}

type Token = (TokenType, String);

trait SymbolTableTrait {
    fn lookup(self, value: String) -> Option<Token>;
    fn insert(&mut self, token_type: TokenType, value: String);
}

struct SymbolTable {
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

fn lexan(mut input_feed: impl feed::Feed, SymbolTable: impl SymbolTableTrait) {
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
        }
    }

    println!("{:?}", results);
}

fn main() -> ExitCode {
    let mut args = env::args();
    let input_string: String;

    if args.len() > 1 {
        let path = args.nth(1).unwrap();
        input_string = get_input_string(&path);
    } else {
        eprintln!("No file to parse");
        return ExitCode::FAILURE;
    }

    let mut input_feed = feed::InputFeed::new(input_string);
    let mut symbol_table = SymbolTable::new();

    lexan(input_feed, symbol_table);

    return ExitCode::SUCCESS;
}
