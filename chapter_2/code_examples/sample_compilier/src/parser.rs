use std::collections::VecDeque;

use crate::tokens::{Token, TokenType};

pub struct Parser {
    tokens: VecDeque<Token>,
    look_ahead: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut tokens_vecdeq = VecDeque::new();

        for item in tokens {
            tokens_vecdeq.push_back(item);
        }

        Parser {
            tokens: tokens_vecdeq,
            look_ahead: None,
        }
    }

    pub fn parse(&mut self) {
        self.look_ahead = self.tokens.pop_front();

        while self.look_ahead.is_some() {
            self.expr();
            self.match_token((TokenType::SEMICOLON, ";".to_string()));
            // adding a new line to create space
            println!();
        }
    }

    pub fn expr(&mut self) {
        self.term();
        loop {
            let current_token = self.look_ahead.clone().unwrap();
            match current_token.0 {
                TokenType::PLUS | TokenType::MINUS => {
                    self.match_token((
                        current_token.0.clone(),
                        current_token.1.clone().to_string(),
                    ));
                    self.term();
                    print!("{}", current_token.1);
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn match_token(&mut self, wanted_token: Token) {
        let current_token = self.look_ahead.clone().unwrap();
        //println!("Wanted {:?}, Current: {:?}", &wanted_token, current_token);
        if current_token.eq(&wanted_token) {
            self.look_ahead = self.tokens.pop_front();
        } else {
            eprintln!("Wanted {:?}, Current: {:?}", wanted_token, current_token);
            panic!("Syntax error")
        }
    }

    pub fn term(&mut self) {
        self.factor();
        loop {
            let current_token = self.look_ahead.clone().unwrap();
            match current_token.0 {
                TokenType::MULTIPLY | TokenType::DIVIDE => {
                    self.match_token((
                        current_token.0.clone(),
                        current_token.1.clone().to_string(),
                    ));
                    self.factor();
                    print!("{}", current_token.1);
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn factor(&mut self) {
        let current_token = self.look_ahead.clone().unwrap();
        match current_token.0 {
            TokenType::LEFT_PARATHESIS => {
                self.match_token((TokenType::LEFT_PARATHESIS, '('.to_string()));
                self.expr();
                self.match_token((TokenType::RIGHT_PARATHESIS, ')'.to_string()));
            }
            TokenType::NUM => {
                print!("{:?}", current_token.1);
                self.match_token((TokenType::NUM, current_token.1.to_string()));
            }
            TokenType::ID => {
                print!("{:?}", current_token.1);
                self.match_token((TokenType::ID, current_token.1.to_string()));
            }
            _ => {
                panic!(
                    "{}",
                    &format!("Unexpected look ahead character: {}", &current_token.1)
                )
            }
        }
    }
}