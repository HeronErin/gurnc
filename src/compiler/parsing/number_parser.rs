use std::num::ParseIntError;

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
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NumberType {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISIZE,
    U8,
    U16,
    U32,
    U64,
    U128,
    USIZE,
    F32,
    F64,
    F128,
    REAL,
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub text_content: String,
    pub text_filtered: String,
    pub is_negative: bool,
    pub has_decimal : bool,
    pub detected_base: Option<NumberBase>,
    pub number_type: Option<NumberType>,
}
use num_traits::Num;
impl NumberLiteral {
    pub fn new(number: &str) -> Option<(usize, Self)> {
        let mut size = 0;
        let is_negative = number.starts_with('-');
        let mut current_string = if !is_negative {
            number
        } else {
            size+=1; &number[1..]
        };
        let first_char = current_string.chars().next()?;
        
        // First must be zero to specify a base, or a decimal number
        if !first_char.is_numeric(){
            return None;
        }

        let detected_base = NUMBER_PREFIX_DATA
            .iter()
            .filter(|prefix| current_string.len() >= prefix.0.len() && current_string[..prefix.0.len()].eq_ignore_ascii_case(prefix.0))
            .map(|prefix| prefix.1.clone())
            .next();
        if let Some(base) = detected_base.as_ref() {
            current_string = &current_string[2..];
            size+=2;
        }
        let base = detected_base.clone().unwrap_or(NumberBase::Decimal);
        
        let mut filtered = (if is_negative {"-"} else {""}).to_string();
        let mut highest_valid: usize = usize::MAX;
        let mut next_invalid = usize::MAX;

        let mut hasSeenPeriod = false;
        
        for char in current_string.char_indices().filter(|c| c.1 != '_'){


            if '.' == char.1{
                hasSeenPeriod = true;
            }
            else if !base.is_valid(char.1){
                next_invalid = char.0;
                break;
            }
            

            filtered.push(char.1);
            highest_valid = char.0;
        }
        // Found nothing
        if highest_valid == usize::MAX{return None;}
        
        // We are at the end of the buffer
        if next_invalid == usize::MAX{
            size += highest_valid + 1;
            return Some((size, Self {
                text_content: number[..size].to_string(),
                text_filtered: filtered,
                is_negative,
                detected_base,
                number_type: None,
                has_decimal: hasSeenPeriod
            }))
        }
        size += next_invalid;
        current_string = &current_string[next_invalid..];


        let number_type = NUMBER_SUFFIX_DATA
            .iter()
            .filter(|suffix| 
                current_string.len() >= suffix.0.len() && current_string[..suffix.0.len()].eq_ignore_ascii_case(suffix.0)
            )
            .map(|prefix| (prefix.0.len(), prefix.1.clone()))
            .next();
        if let Some(ty) = number_type.as_ref() {
            size += ty.0;
        }
        Some((size, Self {
            text_content: number[..size].to_string(),
            text_filtered: filtered,
            is_negative,
            detected_base,
            number_type: number_type.map(|t| t.1),
            has_decimal: hasSeenPeriod
        }))
        
    }

    pub fn parse_int<T: Num>(&self) -> Result<T, T::FromStrRadixErr> {
        let mut ret = T::from_str_radix(
            self.text_filtered.as_str(),
            self.detected_base.clone().unwrap_or(NumberBase::Decimal) as u32,
        )?;

        Ok(ret)
    }
}

const NUMBER_SUFFIX_DATA: [(&'static str, NumberType); 16] = [
    ("i8", NumberType::I8),
    ("i16", NumberType::I16),
    ("i32", NumberType::I32),
    ("i64", NumberType::I64),
    ("i128", NumberType::I128),
    ("isize", NumberType::ISIZE),
    ("u8", NumberType::U8),
    ("u16", NumberType::U16),
    ("u32", NumberType::U32),
    ("u64", NumberType::U64),
    ("u128", NumberType::U128),
    ("usize", NumberType::USIZE),
    ("f32", NumberType::F32),
    ("f64", NumberType::F64),
    ("f128", NumberType::F128),
    ("real", NumberType::REAL),
];

const NUMBER_PREFIX_DATA: [(&'static str, NumberBase); 4] = [
    ("0b", NumberBase::Binary),
    ("0d", NumberBase::Decimal),
    ("0o", NumberBase::Octal),
    ("0x", NumberBase::Hexadecimal),
];

impl NumberType {
    pub fn to_size(&self) -> Option<usize> {
        match self {
            NumberType::I8 | NumberType::U8 => Some(1),
            NumberType::I16 | NumberType::U16 => Some(2),
            NumberType::I32 | NumberType::U32 | NumberType::F32 => Some(4),
            NumberType::I64 | NumberType::U64 | NumberType::F64 => Some(8),
            NumberType::I128 | NumberType::U128 | NumberType::F128 => Some(16),
            NumberType::ISIZE | NumberType::USIZE => None,
            NumberType::REAL => Some(8),
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            NumberType::F32 | NumberType::F64 | NumberType::F128 => true,
            _ => false,
        }
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
        assert_eq!(literal.number_type, Some(NumberType::U8));
    }


    #[test]
    fn test_number_literal_parse_int_i32() {
        let literal = NumberLiteral {
            text_content: "123".to_string(),
            text_filtered: "123".to_string(),
            is_negative: false,
            has_decimal: false,
            detected_base: Some(NumberBase::Decimal),
            number_type: Some(NumberType::I32),
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
            number_type: Some(NumberType::U8),
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
        assert_eq!(NumberType::I8.to_size(), Some(1));
        assert_eq!(NumberType::U8.to_size(), Some(1));
        assert_eq!(NumberType::I16.to_size(), Some(2));
        assert_eq!(NumberType::U16.to_size(), Some(2));
        assert_eq!(NumberType::I32.to_size(), Some(4));
        assert_eq!(NumberType::U32.to_size(), Some(4));
        assert_eq!(NumberType::F32.to_size(), Some(4));
        assert_eq!(NumberType::I64.to_size(), Some(8));
        assert_eq!(NumberType::U64.to_size(), Some(8));
        assert_eq!(NumberType::F64.to_size(), Some(8));
        assert_eq!(NumberType::I128.to_size(), Some(16));
        assert_eq!(NumberType::U128.to_size(), Some(16));
        assert_eq!(NumberType::F128.to_size(), Some(16));
        assert_eq!(NumberType::ISIZE.to_size(), None);
        assert_eq!(NumberType::USIZE.to_size(), None);
        assert_eq!(NumberType::REAL.to_size(), Some(8));
    }

    #[test]
    fn test_number_type_is_float() {
        assert!(NumberType::F32.is_float());
        assert!(NumberType::F64.is_float());
        assert!(NumberType::F128.is_float());
        assert!(!NumberType::I8.is_float());
        assert!(!NumberType::U8.is_float());
    }
}
