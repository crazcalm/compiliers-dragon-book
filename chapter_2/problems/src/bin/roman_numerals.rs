use problems::feed::{Feed, InputFeed};
use problems::roman_tokens::{HundredValue, TenValue, ThousandValue, Token, UnitValue};

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

    let mut input = InputFeed::new(testing.to_string());
    println!("{}", input.get_char().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

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
