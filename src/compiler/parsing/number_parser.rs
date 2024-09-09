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
            _ => panic!("Unknown base: {:?}", self),
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
pub struct NumberLiteral {
    pub text_content: String,
    pub text_filtered: String,
    pub is_negative: bool,
    pub detected_base: Option<NumberBase>,
    pub number_type: Option<NumberType>,
}
use num_traits::Num;
impl NumberLiteral {
    pub fn new(number: String) -> Self {
        let no_underscore: String = number.as_str().chars().filter(|c| *c != '_').collect();
        let is_negative = no_underscore.starts_with('-');
        let mut no_negative = if !is_negative {
            no_underscore
        } else {
            no_underscore.strip_prefix("-").unwrap().to_string()
        };

        let detected_base = NUMBER_PREFIX_DATA
            .iter()
            .filter(|prefix| no_negative.starts_with(prefix.0))
            .map(|prefix| prefix.1.clone())
            .next();
        if let Some(base) = detected_base.as_ref() {
            no_negative = no_negative[2..].to_string();
        }
        let number_type = NUMBER_SUFFIX_DATA
            .iter()
            .filter(|suffix| no_negative.ends_with(suffix.0))
            .map(|prefix| (prefix.0.len(), prefix.1.clone()))
            .next();
        if let Some(ty) = number_type.as_ref() {
            no_negative = no_negative[..no_negative.len() - ty.0].to_string();
        }
        Self {
            text_content: number,
            text_filtered: no_negative,
            is_negative,
            detected_base,
            number_type: number_type.map(|t| t.1),
        }
    }

    pub fn parse_int<T: Num>(&self) -> Result<T, T::FromStrRadixErr> {
        let mut ret = T::from_str_radix(
            self.text_filtered.as_str(),
            self.detected_base.clone().unwrap_or(NumberBase::Decimal) as u32,
        )?;

        if self.is_negative {
            ret = T::zero().sub(ret);
        }

        Ok(ret)
    }
}
pub fn quick_number_check(mut maybe_number: &str) -> Option<(usize, NumberBase, Option<NumberType>)> {
    let mut ret = 0;
    if (maybe_number.starts_with('-')) {
        maybe_number = maybe_number.get(1..).unwrap();
        ret += 1;
    }
    let mut base = NumberBase::Decimal;
    for prefix in NUMBER_PREFIX_DATA.iter() {
        if !maybe_number.starts_with(prefix.0) {
            continue;
        }
        ret += prefix.0.len();
        maybe_number = maybe_number.get(prefix.0.len()..).unwrap();
        base = prefix.1.clone();

        break;
    }
    let no_underscore = maybe_number
        .char_indices()
        .into_iter()
        .filter(|c| c.1 != '_');
    let mut max_valid_digit = 0;
    for (index, char) in no_underscore {
        if !base.is_valid(char) {
            break;
        }
        max_valid_digit = index;
    }
    ret += max_valid_digit;
    maybe_number = maybe_number.get(max_valid_digit..).unwrap();
    let suffix = NUMBER_SUFFIX_DATA
        .iter()
        .filter(|suffix| maybe_number.starts_with(suffix.0))
        .next();
    if let Some(suffix) = suffix{
        ret += suffix.0.len();
    }

    Some((ret, base, suffix.map(|s| s.1.clone())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse_integers() {
        // Decimal tests
        assert_eq!(
            Ok(123),
            NumberLiteral::new("123".to_string()).parse_int::<i32>()
        );
        assert_eq!(
            Ok(-123),
            NumberLiteral::new("-123".to_string()).parse_int::<i32>()
        );
        assert_eq!(
            Ok(255),
            NumberLiteral::new("255u8".to_string()).parse_int::<u8>()
        );

        // Binary tests
        assert_eq!(
            Ok(5),
            NumberLiteral::new("0b101".to_string()).parse_int::<i32>()
        );
        assert_eq!(
            Ok(-2),
            NumberLiteral::new("-0b10".to_string()).parse_int::<i32>()
        );

        // Octal tests
        assert_eq!(
            Ok(8),
            NumberLiteral::new("0o10".to_string()).parse_int::<i32>()
        );
        assert_eq!(
            Ok(-8),
            NumberLiteral::new("-0o10".to_string()).parse_int::<i32>()
        );

        // Hexadecimal tests
        assert_eq!(
            Ok(26),
            NumberLiteral::new("0x1A".to_string()).parse_int::<i32>()
        );
        assert_eq!(
            Ok(-26),
            NumberLiteral::new("-0x1A".to_string()).parse_int::<i32>()
        );
        assert_eq!(
            Ok(16),
            NumberLiteral::new("0x10u32".to_string()).parse_int::<u32>()
        );
    }

    #[test]
    pub fn test_parse_floats() {
        // Float tests
        let f = NumberLiteral::new("-0.1".to_string()).parse_int::<f32>();
        assert_eq!(-0.1, f.unwrap());
        let f = NumberLiteral::new("3.14".to_string()).parse_int::<f32>();
        assert_eq!(3.14, f.unwrap());
    }

    #[test]
    pub fn test_invalid_parse() {
        // Invalid parsing
        assert!(NumberLiteral::new("invalid".to_string())
            .parse_int::<i32>()
            .is_err());
        assert!(NumberLiteral::new("0xG".to_string())
            .parse_int::<i32>()
            .is_err());
        assert!(NumberLiteral::new("-0o8".to_string())
            .parse_int::<i32>()
            .is_err()); // Octal digit out of range
    }

    #[test]
    pub fn test_with_suffixes() {
        // Suffix parsing
        assert_eq!(
            Ok(255),
            NumberLiteral::new("255u8".to_string()).parse_int::<u8>()
        );
        assert_eq!(
            Ok(-127),
            NumberLiteral::new("-127i8".to_string()).parse_int::<i8>()
        );
        assert_eq!(
            Ok(1024),
            NumberLiteral::new("1024u16".to_string()).parse_int::<u16>()
        );
    }

    #[test]
    pub fn test_negative_numbers() {
        // Ensure negatives are parsed correctly
        assert_eq!(
            Ok(-1),
            NumberLiteral::new("-1".to_string()).parse_int::<i8>()
        );
        assert_eq!(
            Ok(-10),
            NumberLiteral::new("-0xa".to_string()).parse_int::<i8>()
        );
        assert_eq!(
            Ok(-10),
            NumberLiteral::new("-0xai8".to_string()).parse_int::<i8>()
        );
    }

    #[test]
    pub fn test_text_content_and_filtering() {
        let literal = NumberLiteral::new("0xFFu8".to_string());
        assert_eq!(literal.text_content, "0xFFu8");
        assert_eq!(literal.text_filtered, "FF");
        assert_eq!(literal.detected_base, Some(NumberBase::Hexadecimal));
        assert_eq!(literal.number_type, Some(NumberType::U8));
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
