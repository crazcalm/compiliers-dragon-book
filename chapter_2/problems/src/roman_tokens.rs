enum UnitValue {
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

enum TenValue {
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

enum HundredValue {
    OneHundred,
    TwoHundred,
    ThreeHundred,
    FourHundred,
    FiveHundred,
    SixHundred,
    SevenHundred,
    EightHundred,
    NineHundred,
}

enum ThousandValue {
    OneThousand,
    TwoThousand,
    ThreeThousand,
}

enum Token {
    Unit(UnitValue),
    Ten(TenValue),
    Hundred(HundredValue),
    Thousand(ThousandValue),
}

fn token_value(token: Token) -> String {
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
            HundredValue::OneHundred => "C".to_string(),
            HundredValue::TwoHundred => "CC".to_string(),
            HundredValue::ThreeHundred => "CCC".to_string(),
            HundredValue::FourHundred => "CD".to_string(),
            HundredValue::FiveHundred => "D".to_string(),
            HundredValue::SixHundred => "DC".to_string(),
            HundredValue::SevenHundred => "DCC".to_string(),
            HundredValue::EightHundred => "DCCC".to_string(),
            HundredValue::NineHundred => "CM".to_string(),
        },
        Token::Thousand(thousand_value) => match thousand_value {
            ThousandValue::OneThousand => "M".to_string(),
            ThousandValue::TwoThousand => "MM".to_string(),
            ThousandValue::ThreeThousand => "MMM".to_string(),
        },
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
            (Token::Hundred(HundredValue::OneHundred), "C".to_string()),
            (Token::Hundred(HundredValue::TwoHundred), "CC".to_string()),
            (
                Token::Hundred(HundredValue::ThreeHundred),
                "CCC".to_string(),
            ),
            (Token::Hundred(HundredValue::FourHundred), "CD".to_string()),
            (Token::Hundred(HundredValue::FiveHundred), "D".to_string()),
            (Token::Hundred(HundredValue::SixHundred), "DC".to_string()),
            (
                Token::Hundred(HundredValue::SevenHundred),
                "DCC".to_string(),
            ),
            (
                Token::Hundred(HundredValue::EightHundred),
                "DCCC".to_string(),
            ),
            (Token::Hundred(HundredValue::NineHundred), "CM".to_string()),
            (Token::Thousand(ThousandValue::OneThousand), "M".to_string()),
            (
                Token::Thousand(ThousandValue::TwoThousand),
                "MM".to_string(),
            ),
            (
                Token::Thousand(ThousandValue::ThreeThousand),
                "MMM".to_string(),
            ),
        ];

        for (input, expected) in test_cases {
            let result = token_value(input);

            assert_eq!(result, expected);
        }
    }
}
