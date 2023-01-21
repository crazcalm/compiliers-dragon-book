use std::collections::VecDeque;

use crate::tokens::{Token, TokenType};

pub struct Parser {
    tokens: VecDeque<Token>,
    look_ahead: Option<Token>,
    results: String,
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
            results: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> String {
        self.look_ahead = self.tokens.pop_front();

        while self.look_ahead.is_some() {
            self.expr();
            self.match_token((TokenType::Semicolon, ";".to_string()));

            // adding a new line to create space
            self.results.push_str("\n");
        }

        self.results.clone()
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

                    // Adding to result string
                    self.results.push_str(current_token.1.as_str());
                    self.results.push_str(" ");
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn match_token(&mut self, wanted_token: Token) {
        let current_token = self
            .look_ahead
            .clone()
            .expect("Are you missing a semicolon?");

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

                    // Adding to result string
                    self.results.push_str(current_token.1.as_str());
                    self.results.push_str(" ");
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
                // Adding to result string
                self.results.push_str(current_token.1.as_str());
                self.results.push_str(" ");

                self.match_token((TokenType::Num, current_token.1.to_string()));
            }
            TokenType::Id => {
                // Adding to result string
                self.results.push_str(current_token.1.as_str());
                self.results.push_str(" ");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let inputs = vec![
            (
                vec![
                    (TokenType::Num, "1".to_string()),
                    (TokenType::Plus, "+".to_string()),
                    (TokenType::Num, "1".to_string()),
                    (TokenType::Semicolon, ";".to_string()),
                ],
                "1 1 + \n".to_string(),
            ),
            (
                vec![
                    (TokenType::Num, "1".to_string()),
                    (TokenType::Multiply, "*".to_string()),
                    (TokenType::LeftParathesis, "(".to_string()),
                    (TokenType::Num, "12".to_string()),
                    (TokenType::Divide, "/".to_string()),
                    (TokenType::Num, "4".to_string()),
                    (TokenType::RightParathesis, ")".to_string()),
                    (TokenType::Semicolon, ";".to_string()),
                ],
                "1 12 4 / * \n".to_string(),
            ),
            (
                vec![
                    (TokenType::Num, "1".to_string()),
                    (TokenType::Plus, "+".to_string()),
                    (TokenType::Num, "2".to_string()),
                    (TokenType::Semicolon, ";".to_string()),
                    (TokenType::Num, "2".to_string()),
                    (TokenType::Plus, "+".to_string()),
                    (TokenType::Num, "3".to_string()),
                    (TokenType::Semicolon, ";".to_string()),
                ],
                "1 2 + \n2 3 + \n".to_string(),
            ),
        ];

        for (input, expected) in inputs {
            let results = Parser::new(input).parse();

            assert_eq!(results, expected);
        }
    }
}
