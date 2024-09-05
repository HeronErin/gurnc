use std::error::Error;

// TODO: Implement this shit
pub struct StringLiteral;
use super::number_parser::*;

pub enum TokenData {
    Keyword(String),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
}
pub struct Token {
    pub index: i32,
    pub length: i32,
    pub data: TokenData,
}
