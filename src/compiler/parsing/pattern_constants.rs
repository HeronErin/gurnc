use super::{
    pattern_matcher::*,
    tokenizer::{Token, TokenData},
    type_parser::type_size_function,
};
use Match::*;
use crate::compiler::keywords::Keyword;


const TYPE_GLOB: Match<'static> = GlobWithSizer(type_size_function);



macro_rules! basic_control_flow {
    ($name : ident, $keyword : ident, $condition : expr) => {
        pub const $name: &[Match] = &[
            Optional(&[Of(&[TokenData::Keyword(Keyword::Comptime)])]),
            IgnoreWhitespace,
            Of(&[TokenData::Keyword(Keyword::$keyword)]),
            IgnoreWhitespace,
            Bracket(b'(', $condition),
            IgnoreWhitespace,
            Either(
                &[Glob, Of(&[TokenData::Semicolon])], // Single line
                &[Bracket(b'{', &[Glob])],            // Scoped if
            ),
        ];
    };
}
basic_control_flow!(IF_STATEMENT, If, &[Glob]);
basic_control_flow!(ELSE_STATEMENT, Else, &[Glob]);
basic_control_flow!(ELSE_IF_STATEMENT, ElseIf, &[Glob]);
basic_control_flow!(FOR_LOOP, For, &[
    Optional(&[TYPE_GLOB]),
    OfType(&[TokenData::TextCluster(None)]),
    Of(&[TokenData::Semicolon]),
    Glob,
]);



pub const DO_WHILE_LOOP: &[Match] = &[
    Of(&[TokenData::Keyword(Keyword::Do)]),
    Bracket(b'{', &[Glob]),
    Of(&[TokenData::Keyword(Keyword::While)]),
    IgnoreWhitespace,
    Bracket(b'(', &[Glob]),
    IgnoreWhitespace,
    Of(&[TokenData::Semicolon]),
];

pub const FUNCTION_DECLARATION: &[Match] = &[
    // Function decorators
    PossibleWhitespaceSeparated(&[
        Of(&[TokenData::AtSign]),
        OfType(&[TokenData::TextCluster(None)]),
        IgnoreWhitespace,
        Optional(&[Bracket(b'(', &[Glob])]),
    ]),
    IgnoreWhitespace,
    PossibleWhitespaceSeparated(&[OfType(&[TokenData::Keyword(Keyword::DUMMY)])]),
    IgnoreWhitespace,
    // Return type
    TYPE_GLOB,
    IgnoreWhitespace,
    // Function Name
    OfType(&[TokenData::TextCluster(None)]),
    IgnoreWhitespace,
    // Generics
    Optional(&[
        Of(&[TokenData::Operator(
            crate::compiler::operators::Operator::LesserThan,
        )]),
        Glob,
        Of(&[TokenData::Operator(
            crate::compiler::operators::Operator::GreaterThan,
        )]),
        IgnoreWhitespace,
    ]),
    // Args
    Bracket(
        b'(',
        &[PossibleCommaSeparated(&[
            IgnoreWhitespace,
            PossibleWhitespaceSeparated(&[OfType(&[TokenData::Keyword(Keyword::DUMMY)])]),
            IgnoreWhitespace,
            TYPE_GLOB,
            Whitespace,
            OfType(&[TokenData::TextCluster(None)]),
            IgnoreWhitespace,
        ])],
    ),
    Optional(&[
        IgnoreWhitespace,
        Of(&[TokenData::Keyword(Keyword::Where)]),
        Glob,
    ]),
    IgnoreWhitespace,
    Either(
        &[Of(&[TokenData::Bracket(b'{', None)])],
        &[
            Of(&[TokenData::Operator(
                crate::compiler::operators::Operator::EqualsArrow,
            )]),
            Glob,
            Of(&[TokenData::Semicolon]),
        ],
    ),
];

pub const TEST: &[Match] = &[
    Of(&[TokenData::Keyword(Keyword::If)]),
    IgnoreWhitespace,
    Optional(&[Of(&[TokenData::Keyword(Keyword::Where)]), Glob]),
    Of(&[TokenData::Semicolon]),
];
