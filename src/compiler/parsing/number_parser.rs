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
        let first_char = number.chars().next()?;
        
        // First must be zero to specify a base, or a decimal number
        if !first_char.is_numeric(){
            return None;
        }

        let detected_base = NUMBER_PREFIX_DATA
            .iter()
            .filter(|prefix| current_string[..prefix.0.len()].eq_ignore_ascii_case(prefix.0))
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

        if self.is_negative {
            ret = T::zero().sub(ret);
        }

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
