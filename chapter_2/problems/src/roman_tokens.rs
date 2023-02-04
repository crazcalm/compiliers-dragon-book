#[derive(Debug, PartialEq, Eq)]
pub enum UnitValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TenValue {
    Ten,
    Twenty,
    Thirty,
    Fourty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HundredValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ThousandValue {
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Unit(UnitValue),
    Ten(TenValue),
    Hundred(HundredValue),
    Thousand(ThousandValue),
    EOF,
}

pub fn token_value(token: Token) -> String {
    match token {
        Token::Unit(unit_value) => match unit_value {
            UnitValue::One => "I".to_string(),
            UnitValue::Two => "II".to_string(),
            UnitValue::Three => "III".to_string(),
            UnitValue::Four => "IV".to_string(),
            UnitValue::Five => "V".to_string(),
            UnitValue::Six => "VI".to_string(),
            UnitValue::Seven => "VII".to_string(),
            UnitValue::Eight => "VIII".to_string(),
            UnitValue::Nine => "IX".to_string(),
        },
        Token::Ten(ten_value) => match ten_value {
            TenValue::Ten => "X".to_string(),
            TenValue::Twenty => "XX".to_string(),
            TenValue::Thirty => "XXX".to_string(),
            TenValue::Fourty => "XL".to_string(),
            TenValue::Fifty => "L".to_string(),
            TenValue::Sixty => "LX".to_string(),
            TenValue::Seventy => "LXX".to_string(),
            TenValue::Eighty => "LXXX".to_string(),
            TenValue::Ninty => "XC".to_string(),
        },
        Token::Hundred(hundred_value) => match hundred_value {
            HundredValue::One => "C".to_string(),
            HundredValue::Two => "CC".to_string(),
            HundredValue::Three => "CCC".to_string(),
            HundredValue::Four => "CD".to_string(),
            HundredValue::Five => "D".to_string(),
            HundredValue::Six => "DC".to_string(),
            HundredValue::Seven => "DCC".to_string(),
            HundredValue::Eight => "DCCC".to_string(),
            HundredValue::Nine => "CM".to_string(),
        },
        Token::Thousand(thousand_value) => match thousand_value {
            ThousandValue::One => "M".to_string(),
            ThousandValue::Two => "MM".to_string(),
            ThousandValue::Three => "MMM".to_string(),
        },
        Token::EOF => ";".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_value() {
        let test_cases = vec![
            (Token::Unit(UnitValue::One), "I".to_string()),
            (Token::Unit(UnitValue::Two), "II".to_string()),
            (Token::Unit(UnitValue::Three), "III".to_string()),
            (Token::Unit(UnitValue::Four), "IV".to_string()),
            (Token::Unit(UnitValue::Five), "V".to_string()),
            (Token::Unit(UnitValue::Six), "VI".to_string()),
            (Token::Unit(UnitValue::Seven), "VII".to_string()),
            (Token::Unit(UnitValue::Eight), "VIII".to_string()),
            (Token::Unit(UnitValue::Nine), "IX".to_string()),
            (Token::Ten(TenValue::Ten), "X".to_string()),
            (Token::Ten(TenValue::Twenty), "XX".to_string()),
            (Token::Ten(TenValue::Thirty), "XXX".to_string()),
            (Token::Ten(TenValue::Fourty), "XL".to_string()),
            (Token::Ten(TenValue::Fifty), "L".to_string()),
            (Token::Ten(TenValue::Sixty), "LX".to_string()),
            (Token::Ten(TenValue::Seventy), "LXX".to_string()),
            (Token::Ten(TenValue::Eighty), "LXXX".to_string()),
            (Token::Ten(TenValue::Ninty), "XC".to_string()),
            (Token::Hundred(HundredValue::One), "C".to_string()),
            (Token::Hundred(HundredValue::Two), "CC".to_string()),
            (Token::Hundred(HundredValue::Three), "CCC".to_string()),
            (Token::Hundred(HundredValue::Four), "CD".to_string()),
            (Token::Hundred(HundredValue::Five), "D".to_string()),
            (Token::Hundred(HundredValue::Six), "DC".to_string()),
            (Token::Hundred(HundredValue::Seven), "DCC".to_string()),
            (Token::Hundred(HundredValue::Eight), "DCCC".to_string()),
            (Token::Hundred(HundredValue::Nine), "CM".to_string()),
            (Token::Thousand(ThousandValue::One), "M".to_string()),
            (Token::Thousand(ThousandValue::Two), "MM".to_string()),
            (Token::Thousand(ThousandValue::Three), "MMM".to_string()),
        ];

        for (input, expected) in test_cases {
            let result = token_value(input);

            assert_eq!(result, expected);
        }
    }
}
