use std::error::Error;
use std::str::Chars;
use std::string;

// TODO: Implement this shit
pub struct StringLiteral;
use super::super::operators::*;
use super::number_parser::*;

pub enum TokenData {
    Keyword(String),
    TextCluster(String),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    Whitespace(String),
    Operator(Operator),
    Semicolon,
    Colon,

    Macro(MacroCall),

    // Starting char, token contents
    Bracket(u8, Vec<Token>), // Anytype of thing that could nest code, including '(', '{', and "["
}

pub struct MacroCall {
    pub name: String,
    pub argument_tokens: Vec<Token>,
}
pub struct Token {
    pub index: usize,
    pub length: usize,
    pub data: TokenData,
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
        println!("chr {}", current);
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
                data: TokenData::Bracket(opening, tokenStack),
            });
            tokenStack = oldStack;
            waiting_for_ending = old_wait;
            index += 1;
        } else if let Some((length, _, _)) = quick_number_check(&text[index..]) {
            let literal = NumberLiteral::new(text[index..index + length].to_string());
            tokenStack.push(Token {
                index,
                length,
                data: TokenData::NumberLiteral(literal),
            });

            index += length;
        } else if let Some((length, opr)) = operator_test(
            &text[index..],
            !canBePreUnary,
            canBePreUnary,
            !canBePreUnary,
        ) {
            tokenStack.push(Token {
                index,
                length,
                data: TokenData::Operator((opr)),
            });
            index += length;
        }else{         
            println!("Unknown {}", index);
            index += 1;
        }
        
        isAfterWhitespace = false;
        canBePreUnary = false;
    }

    todo!()
}
