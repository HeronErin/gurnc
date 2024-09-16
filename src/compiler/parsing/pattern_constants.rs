use super::{pattern_matcher::*, tokenizer::{Token, TokenData}, type_parser::type_size_function, Keyword};
use Match::*;

const TYPE_GLOB : Match<'static> = GlobWithSizer(type_size_function);

pub const IF_STATEMENT : &[Match] = &[
    Of(&[TokenData::Keyword(Keyword::If)]),
    IgnoreWhitespace,
    Bracket(b'(', &[Glob]),
    IgnoreWhitespace,
    Either(
        &[Glob, Of(&[TokenData::Semicolon])], // Single line
        &[Bracket(b'{', &[Glob])] // Scoped if
    )
];

pub const FUNCTION_DECLARATION : &[Match] = &[
    // Function decorators
    PossibleWhitespaceSeparated(&[
        Of(&[TokenData::AtSign]),
        OfType(&[TokenData::TextCluster(None)]),
        IgnoreWhitespace,
        Optional(&[
            Bracket(b'(', &[Glob])
        ])
    ]),
    IgnoreWhitespace,
    PossibleWhitespaceSeparated(&[
        OfType(&[TokenData::Keyword(Keyword::DUMMY)]),
    ]),
    IgnoreWhitespace,
    
    // Return type
    TYPE_GLOB,

    IgnoreWhitespace,
    
    // Function Name
    OfType(&[TokenData::TextCluster(None)]),
    
    IgnoreWhitespace,

    // Generics
    Optional(&[
        Of(&[TokenData::Operator(crate::compiler::operators::Operator::LesserThan)]),
        Glob,
        Of(&[TokenData::Operator(crate::compiler::operators::Operator::GreaterThan)]),
        IgnoreWhitespace
    ]),


    // Args
    Bracket(b'(', &[
        PossibleCommaSeparated(&[
            IgnoreWhitespace,
            PossibleWhitespaceSeparated(&[
                OfType(&[TokenData::Keyword(Keyword::DUMMY)]),
            ]),
            IgnoreWhitespace,
            TYPE_GLOB,
            Whitespace,
            OfType(&[TokenData::TextCluster(None)]),
            IgnoreWhitespace
        ])
    ]),
    Optional(&[
        IgnoreWhitespace,
        Of(&[TokenData::Keyword(Keyword::Where)]),
        Glob,
    ]),
    IgnoreWhitespace,

    Either(&[
        Of(&[TokenData::Bracket(b'{', None)])
    ], &[
        Of(&[TokenData::Operator(crate::compiler::operators::Operator::EqualsArrow)]),
        Glob,
        Of(&[TokenData::Semicolon])
    ])


    
];

pub const TEST : &[Match] = &[
    Of(&[TokenData::Keyword(Keyword::If)]),
    IgnoreWhitespace,
    Optional(&[
        Of(&[TokenData::Keyword(Keyword::Where)]),
        Glob
    ]),
    Of(&[TokenData::Semicolon])
];