use std::{fmt::Debug, num::ParseIntError};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NumberBase {
    Binary = 2,
    Decimal = 10,
    Octal = 8,
    Hexadecimal = 16,
}
impl NumberBase {
    #[inline]
    pub fn is_valid(&self, c: char) -> bool {
        match self {
            Self::Binary => matches!(c, '0'..='1'),
            Self::Octal => matches!(c, '0'..='7'),
            Self::Decimal => matches!(c, '0'..='9'),
            Self::Hexadecimal => matches!(c.to_ascii_lowercase(), '0'..='9' | 'a'..='f'),
            // _ => panic!("Unknown base: {:?}", self),
        }
    }
}



#[derive(Clone)]
pub struct NumberLiteral {
    pub text_content: String,
    pub text_filtered: String,
    pub is_negative: bool,
    pub has_decimal: bool,
    pub detected_base: Option<NumberBase>,
    pub number_type: Option<Primitive>,
}
impl Debug for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.text_content.fmt(f)
    }
}
impl PartialEq for NumberLiteral {
    fn eq(&self, other: &Self) -> bool {
        // Simplist case
        if self.text_content == other.text_content {
            return true;
        }
        if (self.is_negative != other.is_negative){return false;}
        let tp = self.number_type.clone().unwrap_or(Primitive::I32);
        let otp = other.number_type.clone().unwrap_or(Primitive::I32);
        
        if (tp.is_signed() != otp.is_signed()){return false;}
        if (tp.is_unsigned() != otp.is_unsigned()){return false;}
        if (tp.is_float() != otp.is_float()){return false;}
        if (tp.is_float()){
            let parsed_self = self.parse_int::<f64>();
            let parsed_other = other.parse_int::<f64>();
            if parsed_self.is_err() || parsed_other.is_err(){return false;}
            return parsed_self.unwrap() == parsed_other.unwrap();
        }
        if (tp.is_unsigned()){
            let parsed_self = self.parse_int::<u128>();
            let parsed_other = other.parse_int::<u128>();
            if parsed_self.is_err() || parsed_other.is_err(){return false;}
            return parsed_self.unwrap() == parsed_other.unwrap();
        }else{
            let parsed_self = self.parse_int::<i128>();
            let parsed_other = other.parse_int::<i128>();
            if parsed_self.is_err() || parsed_other.is_err(){return false;}
            return parsed_self.unwrap() == parsed_other.unwrap();
        }
    }
}

use num_traits::Num;

use crate::compiler::objects::gurn_objects::Primitive;
impl NumberLiteral {
    pub fn DUMMY() -> Self {
        Self {
            text_content: "DUMMY_NUMBER".to_string(),
            text_filtered: "DUMMY_NUMBER".to_string(),
            is_negative: false,
            has_decimal: false,
            detected_base: None,
            number_type: None,
        }
    }
    pub fn new(number: &str) -> Option<(usize, Self)> {
        let mut size = 0;
        let is_negative = number.starts_with('-');
        let mut current_string = if !is_negative {
            number
        } else {
            size += 1;
            &number[1..]
        };
        let first_char = current_string.chars().next()?;

        // First must be zero to specify a base, or a decimal number
        if !first_char.is_numeric() {
            return None;
        }

        let detected_base = NUMBER_PREFIX_DATA
            .iter()
            .filter(|prefix| {
                current_string.len() >= prefix.0.len()
                    && current_string[..prefix.0.len()].eq_ignore_ascii_case(prefix.0)
            })
            .map(|prefix| prefix.1.clone())
            .next();
        if let Some(base) = detected_base.as_ref() {
            current_string = &current_string[2..];
            size += 2;
        }
        let base = detected_base.clone().unwrap_or(NumberBase::Decimal);

        let mut filtered = (if is_negative { "-" } else { "" }).to_string();
        let mut highest_valid: usize = usize::MAX;
        let mut next_invalid = usize::MAX;

        let mut hasSeenPeriod = false;

        for char in current_string.char_indices().filter(|c| c.1 != '_') {
            if '.' == char.1 {
                hasSeenPeriod = true;
            } else if !base.is_valid(char.1) {
                next_invalid = char.0;
                break;
            }

            filtered.push(char.1);
            highest_valid = char.0;
        }
        // Found nothing
        if highest_valid == usize::MAX {
            return None;
        }

        // We are at the end of the buffer
        if next_invalid == usize::MAX {
            size += highest_valid + 1;
            return Some((
                size,
                Self {
                    text_content: number[..size].to_string(),
                    text_filtered: filtered,
                    is_negative,
                    detected_base,
                    number_type: None,
                    has_decimal: hasSeenPeriod,
                },
            ));
        }
        size += next_invalid;
        current_string = &current_string[next_invalid..];

        let number_type = NUMBER_SUFFIX_DATA
            .iter()
            .filter(|suffix| {
                current_string.len() >= suffix.0.len()
                    && current_string[..suffix.0.len()].eq_ignore_ascii_case(suffix.0)
            })
            .map(|prefix| (prefix.0.len(), prefix.1.clone()))
            .next();
        if let Some(ty) = number_type.as_ref() {
            size += ty.0;
        }
        Some((
            size,
            Self {
                text_content: number[..size].to_string(),
                text_filtered: filtered,
                is_negative,
                detected_base,
                number_type: number_type.map(|t| t.1),
                has_decimal: hasSeenPeriod,
            },
        ))
    }

    pub fn parse_int<T: Num>(&self) -> Result<T, T::FromStrRadixErr> {
        let mut ret = T::from_str_radix(
            self.text_filtered.as_str(),
            self.detected_base.clone().unwrap_or(NumberBase::Decimal) as u32,
        )?;

        Ok(ret)
    }
}

const NUMBER_SUFFIX_DATA: [(&'static str, Primitive); 16] = [
    ("i8", Primitive::I8),
    ("i16", Primitive::I16),
    ("i32", Primitive::I32),
    ("i64", Primitive::I64),
    ("i128", Primitive::I128),
    ("isize", Primitive::ISIZE),
    ("u8", Primitive::U8),
    ("u16", Primitive::U16),
    ("u32", Primitive::U32),
    ("u64", Primitive::U64),
    ("u128", Primitive::U128),
    ("usize", Primitive::USIZE),
    ("f32", Primitive::F32),
    ("f64", Primitive::F64),
    ("f128", Primitive::F128),
    ("real", Primitive::REAL),
];

const NUMBER_PREFIX_DATA: [(&'static str, NumberBase); 4] = [
    ("0b", NumberBase::Binary),
    ("0d", NumberBase::Decimal),
    ("0o", NumberBase::Octal),
    ("0x", NumberBase::Hexadecimal),
];

impl Primitive {
    pub fn to_size(&self) -> Option<usize> {
        match self {
            Primitive::I8 | Primitive::U8 => Some(1),
            Primitive::I16 | Primitive::U16 => Some(2),
            Primitive::I32 | Primitive::U32 | Primitive::F32 => Some(4),
            Primitive::I64 | Primitive::U64 | Primitive::F64 => Some(8),
            Primitive::I128 | Primitive::U128 | Primitive::F128 => Some(16),
            Primitive::ISIZE | Primitive::USIZE => None,
            Primitive::REAL => Some(8),
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Primitive::F32 | Primitive::F64 | Primitive::F128 => true,
            _ => false,
        }
    }
    pub fn is_unsigned(&self) -> bool {
        match self {
            Primitive::U8 | Primitive::U16 | Primitive::U32 | Primitive::U64 | Primitive::U128 |Primitive::USIZE => true,
            _ => false
        }
    }
    pub fn is_signed_int(&self) -> bool {
        match self {
            Primitive::I8 | Primitive::I16 | Primitive::I32 | Primitive::I64 | Primitive::I128 |Primitive::ISIZE => true,
            _ => false
        }
    }
    pub fn is_signed(&self) -> bool {
        self.is_float() || self.is_signed_int()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_base_is_valid_binary() {
        let base = NumberBase::Binary;
        assert!(base.is_valid('0'));
        assert!(base.is_valid('1'));
        assert!(!base.is_valid('2'));
        assert!(!base.is_valid('a'));
    }

    #[test]
    fn test_number_base_is_valid_octal() {
        let base = NumberBase::Octal;
        assert!(base.is_valid('0'));
        assert!(base.is_valid('7'));
        assert!(!base.is_valid('8'));
        assert!(!base.is_valid('a'));
    }

    #[test]
    fn test_number_base_is_valid_decimal() {
        let base = NumberBase::Decimal;
        assert!(base.is_valid('0'));
        assert!(base.is_valid('9'));
        assert!(!base.is_valid('a'));
        assert!(!base.is_valid(' '));
    }

    #[test]
    fn test_number_base_is_valid_hexadecimal() {
        let base = NumberBase::Hexadecimal;
        assert!(base.is_valid('0'));
        assert!(base.is_valid('9'));
        assert!(base.is_valid('a'));
        assert!(base.is_valid('f'));
        assert!(base.is_valid('A'));
        assert!(base.is_valid('F'));
        assert!(!base.is_valid('g'));
        assert!(!base.is_valid(' '));
    }

    #[test]
    fn test_number_literal_new_decimal() {
        let (size, literal) = NumberLiteral::new("1234").unwrap();
        assert_eq!(size, 4);
        assert_eq!(literal.text_content, "1234");
        assert_eq!(literal.text_filtered, "1234");
        assert!(!literal.is_negative);
        assert!(!literal.has_decimal);
        assert_eq!(literal.detected_base, None);
        assert_eq!(literal.number_type, None);
    }

    #[test]
    fn test_number_literal_new_binary() {
        let (size, literal) = NumberLiteral::new("0b1010").unwrap();
        assert_eq!(size, 6);
        assert_eq!(literal.text_content, "0b1010");
        assert_eq!(literal.text_filtered, "1010");
        assert!(!literal.is_negative);
        assert!(!literal.has_decimal);
        assert_eq!(literal.detected_base, Some(NumberBase::Binary));
        assert_eq!(literal.number_type, None);
    }

    #[test]
    fn test_number_literal_new_octal() {
        let (size, literal) = NumberLiteral::new("0o755").unwrap();
        assert_eq!(size, 5);
        assert_eq!(literal.text_content, "0o755");
        assert_eq!(literal.text_filtered, "755");
        assert!(!literal.is_negative);
        assert!(!literal.has_decimal);
        assert_eq!(literal.detected_base, Some(NumberBase::Octal));
        assert_eq!(literal.number_type, None);
    }

    #[test]
    fn test_number_literal_new_hexadecimal() {
        let (size, literal) = NumberLiteral::new("0x1a3f").unwrap();
        assert_eq!(size, 6);
        assert_eq!(literal.text_content, "0x1a3f");
        assert_eq!(literal.text_filtered, "1a3f");
        assert!(!literal.is_negative);
        assert!(!literal.has_decimal);
        assert_eq!(literal.detected_base, Some(NumberBase::Hexadecimal));
        assert_eq!(literal.number_type, None);
    }

    #[test]
    fn test_number_literal_new_negative() {
        let (size, literal) = NumberLiteral::new("-1234 ").unwrap();
        assert_eq!(size, 5);
        assert_eq!(literal.text_content, "-1234");
        assert_eq!(literal.text_filtered, "-1234");
        assert!(literal.is_negative);
        assert!(!literal.has_decimal);
        assert_eq!(literal.detected_base, None);
        assert_eq!(literal.number_type, None);
    }

    #[test]
    fn test_number_literal_new_with_decimal() {
        let (size, literal) = NumberLiteral::new("3.14").unwrap();
        assert_eq!(size, 4);
        assert_eq!(literal.text_content, "3.14");
        assert_eq!(literal.text_filtered, "3.14");
        assert!(!literal.is_negative);
        assert!(literal.has_decimal);
        assert_eq!(literal.detected_base, None);
        assert_eq!(literal.number_type, None);
    }

    #[test]
    fn test_number_literal_new_with_suffix() {
        let (size, literal) = NumberLiteral::new("255u8").unwrap();
        assert_eq!(size, 5);
        assert_eq!(literal.text_content, "255u8");
        assert_eq!(literal.text_filtered, "255");
        assert!(!literal.is_negative);
        assert!(!literal.has_decimal);
        assert_eq!(literal.detected_base, None);
        assert_eq!(literal.number_type, Some(Primitive::U8));
    }

    #[test]
    fn test_number_literal_parse_int_i32() {
        let literal = NumberLiteral {
            text_content: "123".to_string(),
            text_filtered: "123".to_string(),
            is_negative: false,
            has_decimal: false,
            detected_base: Some(NumberBase::Decimal),
            number_type: Some(Primitive::I32),
        };
        let value: i32 = literal.parse_int().unwrap();
        assert_eq!(value, 123);
    }

    #[test]
    fn test_number_literal_parse_int_u8() {
        let literal = NumberLiteral {
            text_content: "255u8".to_string(),
            text_filtered: "255".to_string(),
            is_negative: false,
            has_decimal: false,
            detected_base: Some(NumberBase::Decimal),
            number_type: Some(Primitive::U8),
        };
        let value: u8 = literal.parse_int().unwrap();
        assert_eq!(value, 255);
    }

    #[test]
    fn test_number_literal_parse_int_negative() {
        let literal = NumberLiteral {
            text_content: "-123".to_string(),
            text_filtered: "-123".to_string(),
            is_negative: true,
            has_decimal: false,
            detected_base: Some(NumberBase::Decimal),
            number_type: None,
        };
        let value: i32 = literal.parse_int().unwrap();
        assert_eq!(value, -123);
    }

    #[test]
    fn test_number_literal_parse_int_invalid() {
        let literal = NumberLiteral {
            text_content: "0x1g".to_string(),
            text_filtered: "1g".to_string(),
            is_negative: false,
            has_decimal: false,
            detected_base: Some(NumberBase::Hexadecimal),
            number_type: None,
        };
        assert!(literal.parse_int::<i32>().is_err());
    }

    #[test]
    fn test_number_type_to_size() {
        assert_eq!(Primitive::I8.to_size(), Some(1));
        assert_eq!(Primitive::U8.to_size(), Some(1));
        assert_eq!(Primitive::I16.to_size(), Some(2));
        assert_eq!(Primitive::U16.to_size(), Some(2));
        assert_eq!(Primitive::I32.to_size(), Some(4));
        assert_eq!(Primitive::U32.to_size(), Some(4));
        assert_eq!(Primitive::F32.to_size(), Some(4));
        assert_eq!(Primitive::I64.to_size(), Some(8));
        assert_eq!(Primitive::U64.to_size(), Some(8));
        assert_eq!(Primitive::F64.to_size(), Some(8));
        assert_eq!(Primitive::I128.to_size(), Some(16));
        assert_eq!(Primitive::U128.to_size(), Some(16));
        assert_eq!(Primitive::F128.to_size(), Some(16));
        assert_eq!(Primitive::ISIZE.to_size(), None);
        assert_eq!(Primitive::USIZE.to_size(), None);
        assert_eq!(Primitive::REAL.to_size(), Some(8));
    }

    #[test]
    fn test_number_type_is_float() {
        assert!(Primitive::F32.is_float());
        assert!(Primitive::F64.is_float());
        assert!(Primitive::F128.is_float());
        assert!(!Primitive::I8.is_float());
        assert!(!Primitive::U8.is_float());
    }
}
