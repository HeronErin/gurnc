use super::{pattern_matcher::*, tokenizer::{Token, TokenData}, Keyword};
use Match::*;


pub const IF_STATEMENT : &[Match] = &[
    Of(&[TokenData::Keyword(Keyword::If)]),
    IgnoreWhitespace,
    Of(&[TokenData::Bracket(b'(', None)]),
    IgnoreWhitespace,
    Either(
        &[Glob, Of(&[TokenData::Semicolon])], // Single line
        &[Of(&[TokenData::Bracket(b'{', None)])] // Scoped if
    )
];

pub const FUNCTION_DECLARATION : &[Match] = &[
    // Function decorators
    PossibleWhitespaceSeparated(&[
        Of(&[TokenData::AtSign]),
        OfType(&[TokenData::TextCluster(None)]),
        IgnoreWhitespace,
        Optional(&[
            Of(&[TokenData::Bracket(b'(', None)])
        ])
    ]),
    IgnoreWhitespace,
    PossibleWhitespaceSeparated(&[
        OfType(&[TokenData::Keyword(Keyword::DUMMY)]),
    ]),
    IgnoreWhitespace,
    // Return type
    Glob,
    IgnoreWhitespace,
    
    // Function Name
    OfType(&[TokenData::TextCluster(None)]),
    IgnoreWhitespace,

    // Generics
    Optional(&[(Of(&[TokenData::Bracket(b'<', None)]))]),
    IgnoreWhitespace,
    // Args
    Of(&[TokenData::Bracket(b'(', None)]),
    Optional(&[
        IgnoreWhitespace,
        Of(&[TokenData::Keyword(Keyword::Where)]),
        Glob,
    ]),
    IgnoreWhitespace,

    Either(&[
        Of(&[TokenData::Operator(crate::compiler::operators::Operator::EqualsArrow)]),
        Glob,
        Of(&[TokenData::Semicolon])
    ], &[
        Of(&[TokenData::Bracket(b'{', None)])
    ])


    
];