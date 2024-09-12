use super::{pattern_matcher::*, tokenizer::{Token, TokenData}, Keyword};
use Match::*;


const IF_STATEMENT : &[Match] = &[
    Of(&[TokenData::Keyword(Keyword::If)]),
    OfType(&[TokenData::Bracket(b'(', None)]),
    Either(
        &[Glob, Of(&[TokenData::Semicolon])], // Single line
        &[OfType(&[TokenData::Bracket(b'{', None)])] // Scoped if
    )
];