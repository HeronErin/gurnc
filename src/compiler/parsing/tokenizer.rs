use core::panic;
use std::error::Error;
use std::str::Chars;
use std::string;

// TODO: Implement this shit
pub struct StringLiteral;
use super::super::operators::*;
use super::number_parser::*;

#[derive(Debug, Clone)]
pub enum TokenData {
    Keyword(Keyword),
    TextCluster(String),
    NumberLiteral(NumberLiteral),
    // StringLiteral(StringLiteral),
    Whitespace(String),
    Operator(Operator),
    Semicolon,
    Colon,

    // Starting char, token contents
    Bracket(
        u8,
        Option<Vec<Token> /*  Only None on pattern match constants*/>,
    ), // Anytype of thing that could nest code, including '(', '{', and "[".
}
impl PartialEq for TokenData{
    fn eq(&self, other: &Self) -> bool {
        match self{
            TokenData::Keyword(k) => match other {
                TokenData::Keyword(k2) => k == k2,
                _ => false
            },
            TokenData::TextCluster(text) => match other {
                TokenData::TextCluster(text2) => text == text2,
                _ => false  
            },
            TokenData::NumberLiteral(nl) => {
                match other {
                    TokenData::NumberLiteral(nl2) => nl == nl2,
                    _ => false
                }
            },
            TokenData::Whitespace(_) => matches!(other, TokenData::Whitespace(_)),
            TokenData::Operator(opr) => match other {
                TokenData::Operator(opr2) => opr == opr2,
                _=> false
            },
            TokenData::Semicolon => matches!(other, TokenData::Semicolon),
            TokenData::Colon => matches!(other, TokenData::Colon),
            TokenData::Bracket(c, _) => match other {
                TokenData::Bracket(c2, _) => *c == *c2,
                _ => false
            },
        }
    }
}


#[derive(Debug, Clone)]
pub struct Token {
    pub index: usize,
    pub length: usize,
    pub data: TokenData,
}
impl PartialEq for Token{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[inline]
fn is_opening_bracket(chr: u8) -> bool {
    chr == b'(' || chr == b'[' || chr == b'{' || chr == b'<'
}
#[inline]
fn opening_to_closing(chr: u8) -> u8 {
    match chr {
        b'(' => b')',
        b'{' => b'}',
        b'[' => b']',
        b'<' => b'>',
        _ => 0,
    }
}
#[inline]
fn count_whitespace_indexes(s: &str) -> usize {
    let mut ws = 0;
    for c in s.chars() {
        if !c.is_ascii_whitespace() {
            return ws;
        }
        ws += 1;
    }
    ws
}

fn detect_text_cluster(s: &str) -> Option<usize> {
    if 0 == s.len() {
        return None;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    if (first.is_whitespace() || first.is_ascii_punctuation() || first.is_ascii_digit()) {
        return None;
    }
    let mut count = 1;
    for char in chars {
        if char.is_whitespace() || char.is_ascii_punctuation() {
            break;
        }
        count += 1;
    }

    Some(count)
}

use super::keywords::*;
use crate::compiler::errors::*;

pub fn tokenize_text(text: String) -> Result<Vec<Token>, ParsingError> {
    let str = text.as_str();
    let mut index = 0;
    let mut waiting_for_ending: u8 = 0;

    // Opening index, char, old tokenStack, waiting_for_ending
    let mut bracketStack: Vec<(usize, u8, Vec<Token>, u8)> = Vec::new();

    let mut tokenStack = Vec::new();

    let mut canBePreUnary = true;

    let mut isAfterWhitespace = false;

    while (index < text.len()) {
        let current = *text[index..].as_bytes().first().unwrap();
        if (current.is_ascii_whitespace()) {
            let amount = count_whitespace_indexes(&text[index..]);
            tokenStack.push(Token {
                index: index,
                length: amount,
                data: TokenData::Whitespace(text[index..index + amount].to_string()),
            });

            index += amount;
            isAfterWhitespace = true;
            continue;
        }

        if (is_opening_bracket(current)) {
            let closing = opening_to_closing(current);

            bracketStack.push((index, current, tokenStack, waiting_for_ending));

            tokenStack = Vec::new();
            waiting_for_ending = closing;

            canBePreUnary = true;
            index += 1;
        } else if (current == waiting_for_ending) {
            let (start, opening, mut oldStack, old_wait) =
                bracketStack.pop().ok_or(ParsingError::BracketCountError)?;

            oldStack.push(Token {
                index: start,
                length: index - start,
                data: TokenData::Bracket(opening, Some(tokenStack)),
            });
            tokenStack = oldStack;
            waiting_for_ending = old_wait;
            index += 1;
        } else if (current == b',') {
            tokenStack.push(Token {
                index: index,
                length: 1,
                data: TokenData::Colon,
            });
            index += 1;
            canBePreUnary = true;
            continue;
        } else if (current == b';') {
            tokenStack.push(Token {
                index: index,
                length: 1,
                data: TokenData::Semicolon,
            });
            index += 1;
            canBePreUnary = true;
            continue;
        } else if let Some((length, numberLiteral)) = NumberLiteral::new(&text[index..]) {
            debug_assert_ne!(length, 0);

            tokenStack.push(Token {
                index,
                length,
                data: TokenData::NumberLiteral(numberLiteral),
            });

            index += length;
        } else if let Some((length, opr)) = operator_test(
            &text[index..],
            !canBePreUnary,
            canBePreUnary,
            !canBePreUnary,
        ) {
            debug_assert_ne!(length, 0);
            tokenStack.push(Token {
                index,
                length,
                data: TokenData::Operator((opr)),
            });
            index += length;
            canBePreUnary = true;
            continue;
        } else if let Some(length) = detect_text_cluster(&text[index..]) {
            debug_assert_ne!(length, 0);
            let cluster = &text[index..index + length].to_lowercase();
            let possible_keyword = Keyword::try_from_string(&cluster);
            if let Some(keyword) = possible_keyword {
                tokenStack.push(Token {
                    index,
                    length,
                    data: TokenData::Keyword(keyword),
                })
            } else {
                tokenStack.push(Token {
                    index,
                    length,
                    data: TokenData::TextCluster(text[index..index + length].to_string()),
                })
            }

            index += length;
        } else {
            println!("Unknown {} at {}", current, index);
            return Err(ParsingError::UnknownTokenizationError);
            index += 1;
        }

        isAfterWhitespace = false;
        canBePreUnary = false;
    }
    if (bracketStack.len() != 0) {
        return Err(ParsingError::BracketCountError);
    }
    Ok(tokenStack)
}
