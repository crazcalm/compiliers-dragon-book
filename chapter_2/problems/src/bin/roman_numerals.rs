use std::collections::VecDeque;

use problems::feed::{Feed, InputFeed};
use problems::roman_tokens::{HundredValue, TenValue, ThousandValue, Token, UnitValue};

pub fn token_to_value(token: Token) -> i32 {
    match token {
        Token::Thousand(value) => match value {
            ThousandValue::One => 1000,
            ThousandValue::Two => 2000,
            ThousandValue::Three => 3000,
        },
        Token::Hundred(value) => match value {
            HundredValue::One => 100,
            HundredValue::Two => 200,
            HundredValue::Three => 300,
            HundredValue::Four => 400,
            HundredValue::Five => 500,
            HundredValue::Six => 600,
            HundredValue::Seven => 700,
            HundredValue::Eight => 800,
            HundredValue::Nine => 900,
        },
        Token::Ten(value) => match value {
            TenValue::Ten => 10,
            TenValue::Twenty => 20,
            TenValue::Thirty => 30,
            TenValue::Fourty => 40,
            TenValue::Fifty => 50,
            TenValue::Sixty => 60,
            TenValue::Seventy => 70,
            TenValue::Eighty => 80,
            TenValue::Ninty => 90,
        },
        Token::Unit(value) => match value {
            UnitValue::One => 1,
            UnitValue::Two => 2,
            UnitValue::Three => 3,
            UnitValue::Four => 4,
            UnitValue::Five => 5,
            UnitValue::Six => 6,
            UnitValue::Seven => 7,
            UnitValue::Eight => 8,
            UnitValue::Nine => 9,
        },
        Token::EOF => 0,
    }
}

pub struct Parser {
    tokens: VecDeque<Token>,
    look_ahead: Option<Token>,
    result: i32,
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
            result: 0,
        }
    }

    pub fn parse(&mut self) -> i32 {
        self.look_ahead = self.tokens.pop_front();

        if let Some(current_token) = self.look_ahead.clone() {
            match current_token {
                Token::Thousand(thousand_value) => {
                    let value = token_to_value(Token::Thousand(thousand_value));

                    self.result += value;

                    // Parse next token
                    self.hundred_or_lower();
                }
                Token::Hundred(hundred_value) => {
                    self.result += token_to_value(Token::Hundred(hundred_value));

                    // Parse next token
                    self.ten_or_lower();
                }
                Token::Ten(ten_value) => {
                    self.result += token_to_value(Token::Ten(ten_value));

                    // Parse next token
                    self.unit_or_lower();
                }
                Token::Unit(unit_value) => {
                    self.result += token_to_value(Token::Unit(unit_value));

                    match self.tokens.pop_front() {
                        Some(token) => panic!("unexpected extra token: {:?}", token),
                        None => {}
                    }
                }
                Token::EOF => {}
            };
        }

        self.result
    }

    pub fn hundred_or_lower(&mut self) {
        self.look_ahead = self.tokens.pop_front();
        let current_token = self.look_ahead.clone().unwrap();

        match current_token {
            Token::Hundred(hundred_value) => {
                self.result += token_to_value(Token::Hundred(hundred_value));

                // Parse next token
                self.ten_or_lower();
            }
            Token::Ten(ten_value) => {
                self.result += token_to_value(Token::Ten(ten_value));

                // Parse next token
                self.unit_or_lower();
            }
            Token::Unit(unit_value) => {
                self.result += token_to_value(Token::Unit(unit_value));
            }
            Token::EOF => {}
            _ => panic!("unexpected token: {:?}", current_token),
        }
    }
    pub fn ten_or_lower(&mut self) {
        self.look_ahead = self.tokens.pop_front();
        let current_token = self.look_ahead.clone().unwrap();

        match current_token {
            Token::Ten(ten_value) => {
                self.result += token_to_value(Token::Ten(ten_value));

                // Parse next token
                self.unit_or_lower();
            }
            Token::Unit(unit_value) => {
                self.result += token_to_value(Token::Unit(unit_value));
            }
            Token::EOF => {}
            _ => panic!("unexpected token: {:?}", current_token),
        }
    }
    pub fn unit_or_lower(&mut self) {
        self.look_ahead = self.tokens.pop_front();
        let current_token = self.look_ahead.clone().unwrap();

        match current_token {
            Token::Unit(unit_value) => {
                self.result += token_to_value(Token::Unit(unit_value));

                match self.tokens.pop_front() {
                    Some(token) => panic!("unexpected extra token: {:?}", token),
                    None => {}
                }
            }
            Token::EOF => {}
            _ => panic!("unexpected token: {:?}", current_token),
        }
    }
}

fn lexan(mut input: InputFeed) -> Vec<Token> {
    let mut result = Vec::new();

    // Loop over all the available characters
    while let Some(character) = input.get_char() {
        // Thousands
        if character.eq(&'M') {
            /*
            Cases:
            - MMM
            - MM
            - M
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'M') {
                    temp.push(ch);
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("MMM") {
                result.push(Token::Thousand(ThousandValue::Three));
            } else if temp.eq("MM") {
                result.push(Token::Thousand(ThousandValue::Two));
            } else if temp.eq("M") {
                result.push(Token::Thousand(ThousandValue::One))
            } else {
                panic!("Invalid string: {}", temp);
            }
        }

        // Hundreds case part 1
        if character.eq(&'C') {
            /*
            Cases:
            - CCC
            - CC
            - C
            - CD
            - CM
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'C') {
                    temp.push(ch);
                } else if ch.eq(&'D') || ch.eq(&'M') {
                    if temp.len() > 1 {
                        panic!("Invalid String {}{}", temp, ch);
                    }
                    temp.push(ch);
                    break;
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("CM") {
                result.push(Token::Hundred(HundredValue::Nine));
            } else if temp.eq("CD") {
                result.push(Token::Hundred(HundredValue::Four));
            } else if temp.eq("CCC") {
                result.push(Token::Hundred(HundredValue::Three));
            } else if temp.eq("CC") {
                result.push(Token::Hundred(HundredValue::Two));
            } else if temp.eq("C") {
                result.push(Token::Hundred(HundredValue::One))
            } else {
                panic!("Invalid string: {}", temp);
            }
        }

        // Hundreds case part 2
        if character.eq(&'D') {
            /*
            Cases:
            - DCCC
            - DCC
            - DC
            - D
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'C') {
                    temp.push(ch);
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("DCCC") {
                result.push(Token::Hundred(HundredValue::Eight));
            } else if temp.eq("DCC") {
                result.push(Token::Hundred(HundredValue::Seven));
            } else if temp.eq("DC") {
                result.push(Token::Hundred(HundredValue::Six));
            } else if temp.eq("D") {
                result.push(Token::Hundred(HundredValue::Five));
            } else {
                panic!("Invalid string: {}", temp);
            }
        }

        // Tens case part 1
        if character.eq(&'X') {
            /*
            Cases:
            - XXX
            - XX
            - X
            - XL
            - XC
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'X') {
                    temp.push(ch);
                } else if ch.eq(&'L') || ch.eq(&'C') {
                    if temp.len() > 1 {
                        panic!("Invalid String {}{}", temp, ch);
                    }
                    temp.push(ch);
                    break;
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("XC") {
                result.push(Token::Ten(TenValue::Ninty));
            } else if temp.eq("XL") {
                result.push(Token::Ten(TenValue::Fourty));
            } else if temp.eq("XXX") {
                result.push(Token::Ten(TenValue::Thirty));
            } else if temp.eq("XX") {
                result.push(Token::Ten(TenValue::Twenty));
            } else if temp.eq("X") {
                result.push(Token::Ten(TenValue::Ten))
            } else {
                panic!("Invalid string: {}", temp);
            }
        }

        // Tens case part 2
        if character.eq(&'L') {
            /*
            Cases:
            - LXXX
            - LXX
            - LX
            - L
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'X') {
                    temp.push(ch);
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("LXXX") {
                result.push(Token::Ten(TenValue::Eighty));
            } else if temp.eq("LXX") {
                result.push(Token::Ten(TenValue::Seventy));
            } else if temp.eq("LX") {
                result.push(Token::Ten(TenValue::Sixty));
            } else if temp.eq("L") {
                result.push(Token::Ten(TenValue::Fifty));
            } else {
                panic!("Invalid string: {}", temp);
            }
        }

        // Units case part 1
        if character.eq(&'I') {
            /*
            Cases:
            - III
            - II
            - I
            - IV
            - IX
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'I') {
                    temp.push(ch);
                } else if ch.eq(&'V') || ch.eq(&'X') {
                    if temp.len() > 1 {
                        panic!("Invalid String {}{}", temp, ch);
                    }
                    temp.push(ch);
                    break;
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("IX") {
                result.push(Token::Unit(UnitValue::Nine));
            } else if temp.eq("IV") {
                result.push(Token::Unit(UnitValue::Four));
            } else if temp.eq("III") {
                result.push(Token::Unit(UnitValue::Three));
            } else if temp.eq("II") {
                result.push(Token::Unit(UnitValue::Two));
            } else if temp.eq("I") {
                result.push(Token::Unit(UnitValue::One))
            } else {
                panic!("Invalid string: {}", temp);
            }
        }

        // Unit case part 2
        if character.eq(&'V') {
            /*
            Cases:
            - VIII
            - VII
            - VI
            - V
            */

            let mut temp = character.to_string();

            while let Some(ch) = input.get_char() {
                if ch.eq(&'I') {
                    temp.push(ch);
                } else {
                    input.unget_char();
                    break;
                }
            }

            if temp.eq("VIII") {
                result.push(Token::Unit(UnitValue::Eight));
            } else if temp.eq("VII") {
                result.push(Token::Unit(UnitValue::Seven));
            } else if temp.eq("VI") {
                result.push(Token::Unit(UnitValue::Six));
            } else if temp.eq("V") {
                result.push(Token::Unit(UnitValue::Five));
            } else {
                panic!("Invalid string: {}", temp);
            }
        }
    }

    result
}

fn main() {
    let testing = "XXXIX;";

    let input = InputFeed::new(testing.to_string());
    let tokens = lexan(input);

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    println!("{} --> {}", testing, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let test_cases = vec![
            (
                vec![
                    Token::Thousand(ThousandValue::Three),
                    Token::Hundred(HundredValue::Seven),
                    Token::Ten(TenValue::Fifty),
                    Token::Unit(UnitValue::Two),
                ],
                3752,
            ),
            (
                vec![
                    Token::Hundred(HundredValue::Seven),
                    Token::Ten(TenValue::Fifty),
                    Token::Unit(UnitValue::Two),
                ],
                752,
            ),
            (
                vec![Token::Ten(TenValue::Fifty), Token::Unit(UnitValue::Two)],
                52,
            ),
            (vec![Token::Unit(UnitValue::Two)], 2),
        ];

        for (tokens, expected) in test_cases {
            let mut parser = Parser::new(tokens);
            let result = parser.parse();

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_toke_to_value() {
        let test_cases = vec![
            (Token::Thousand(ThousandValue::Three), 3000),
            (Token::Thousand(ThousandValue::Two), 2000),
            (Token::Thousand(ThousandValue::One), 1000),
            (Token::Hundred(HundredValue::One), 100),
            (Token::Hundred(HundredValue::Two), 200),
            (Token::Hundred(HundredValue::Three), 300),
            (Token::Hundred(HundredValue::Four), 400),
            (Token::Hundred(HundredValue::Five), 500),
            (Token::Hundred(HundredValue::Six), 600),
            (Token::Hundred(HundredValue::Seven), 700),
            (Token::Hundred(HundredValue::Eight), 800),
            (Token::Hundred(HundredValue::Nine), 900),
            (Token::Ten(TenValue::Ten), 10),
            (Token::Ten(TenValue::Twenty), 20),
            (Token::Ten(TenValue::Thirty), 30),
            (Token::Ten(TenValue::Fourty), 40),
            (Token::Ten(TenValue::Fifty), 50),
            (Token::Ten(TenValue::Sixty), 60),
            (Token::Ten(TenValue::Seventy), 70),
            (Token::Ten(TenValue::Eighty), 80),
            (Token::Ten(TenValue::Ninty), 90),
            (Token::Unit(UnitValue::One), 1),
            (Token::Unit(UnitValue::Two), 2),
            (Token::Unit(UnitValue::Three), 3),
            (Token::Unit(UnitValue::Four), 4),
            (Token::Unit(UnitValue::Five), 5),
            (Token::Unit(UnitValue::Six), 6),
            (Token::Unit(UnitValue::Seven), 7),
            (Token::Unit(UnitValue::Eight), 8),
            (Token::Unit(UnitValue::Nine), 9),
        ];

        for (token, expected) in test_cases {
            let result = token_to_value(token);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_lexan() {
        let test_cases = vec![
            ("MMM", vec![Token::Thousand(ThousandValue::Three)]),
            ("MM", vec![Token::Thousand(ThousandValue::Two)]),
            ("M", vec![Token::Thousand(ThousandValue::One)]),
            ("CM", vec![Token::Hundred(HundredValue::Nine)]),
            ("DCCC", vec![Token::Hundred(HundredValue::Eight)]),
            ("DCC", vec![Token::Hundred(HundredValue::Seven)]),
            ("DC", vec![Token::Hundred(HundredValue::Six)]),
            ("D", vec![Token::Hundred(HundredValue::Five)]),
            ("CD", vec![Token::Hundred(HundredValue::Four)]),
            ("CCC", vec![Token::Hundred(HundredValue::Three)]),
            ("CC", vec![Token::Hundred(HundredValue::Two)]),
            ("C", vec![Token::Hundred(HundredValue::One)]),
            ("XC", vec![Token::Ten(TenValue::Ninty)]),
            ("LXXX", vec![Token::Ten(TenValue::Eighty)]),
            ("LXX", vec![Token::Ten(TenValue::Seventy)]),
            ("LX", vec![Token::Ten(TenValue::Sixty)]),
            ("L", vec![Token::Ten(TenValue::Fifty)]),
            ("XL", vec![Token::Ten(TenValue::Fourty)]),
            ("XXX", vec![Token::Ten(TenValue::Thirty)]),
            ("XX", vec![Token::Ten(TenValue::Twenty)]),
            ("X", vec![Token::Ten(TenValue::Ten)]),
            ("IX", vec![Token::Unit(UnitValue::Nine)]),
            ("VIII", vec![Token::Unit(UnitValue::Eight)]),
            ("VII", vec![Token::Unit(UnitValue::Seven)]),
            ("VI", vec![Token::Unit(UnitValue::Six)]),
            ("V", vec![Token::Unit(UnitValue::Five)]),
            ("IV", vec![Token::Unit(UnitValue::Four)]),
            ("III", vec![Token::Unit(UnitValue::Three)]),
            ("II", vec![Token::Unit(UnitValue::Two)]),
            ("I", vec![Token::Unit(UnitValue::One)]),
            (
                "MMMDCCCXCIV",
                vec![
                    Token::Thousand(ThousandValue::Three),
                    Token::Hundred(HundredValue::Eight),
                    Token::Ten(TenValue::Ninty),
                    Token::Unit(UnitValue::Four),
                ],
            ),
            (
                "MMMVIII",
                vec![
                    Token::Thousand(ThousandValue::Three),
                    Token::Unit(UnitValue::Eight),
                ],
            ),
        ];

        for (case, expected) in test_cases {
            let input = InputFeed::new(case.to_string());
            let result = lexan(input);

            assert_eq!(result, expected)
        }
    }
}
