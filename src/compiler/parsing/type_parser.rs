use crate::{compiler::operators::Operator, consume_whitespace};

use super::tokenizer::{Token, TokenData};

#[inline]
fn is_valid_type_unary(token : &Token) -> bool{
    match token.data {
        
        TokenData::Operator(Operator::Dereference | Operator::Mult) => true,

        TokenData::Operator(Operator::Reference) => true,

        TokenData::Keyword(super::Keyword::Impl | super::Keyword::Const) => true,

        _ => false
    }
}


#[inline]
fn is_single_type_unit(token : &Token) -> bool{
    match token.data{
        TokenData::TextCluster(_) => true,
        TokenData::Bracket(b'[' | b'(', _) => true,
         _ => false
    }
}

// Verifies if a type is valid*, and gives to token slice after it is complete
// NOTE: This will only test if it _looks_ valid
pub fn type_size_function(mut tokens : &[Token]) -> Option<&[Token]>{
    
    loop{
        tokens = consume_whitespace(tokens).1;
        let current = tokens.get(0)?;
        tokens = &tokens[1..];

        if is_valid_type_unary(current){
            continue;
        }else if is_single_type_unit(current){
            break;
        }
        return None;
    }

    // Consume trailing ptr notation
    while let Some(current) = tokens.get(0) {
        if !matches!(current.data, TokenData::Operator(Operator::Dereference | Operator::Mult)){
            break;
        }
        tokens = &tokens[1..];
    }
    
    Some(tokens)
}