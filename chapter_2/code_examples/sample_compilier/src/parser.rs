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
            self.match_token((TokenType::Semicolon, ";".to_string()));
            // adding a new line to create space
            println!();
        }
    }

    pub fn expr(&mut self) {
        self.term();
        loop {
            let current_token = self.look_ahead.clone().unwrap();
            match current_token.0 {
                TokenType::Plus | TokenType::Minus => {
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
                TokenType::Multiply | TokenType::Divide => {
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
            TokenType::LeftParathesis => {
                self.match_token((TokenType::LeftParathesis, '('.to_string()));
                self.expr();
                self.match_token((TokenType::RightParathesis, ')'.to_string()));
            }
            TokenType::Num => {
                print!("{:?}", current_token.1);
                self.match_token((TokenType::Num, current_token.1.to_string()));
            }
            TokenType::Id => {
                print!("{:?}", current_token.1);
                self.match_token((TokenType::Id, current_token.1.to_string()));
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
