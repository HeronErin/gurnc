pub enum Operator{
    Assign,

    Add,
    AddEq,

    SubEq,
    Sub,

    Mut, // Maybe div, maybe deref
    MultEq,

    Div,
    DivEq,

    Mod,
    ModEq,

    BitwiseAnd,
    BitwiseAndEq,
    
    Xor,
    XorEq,

    BitwiseOr,
    BitwiseOrEq,

    BitwiseShiftRight, // <<
    BitwiseShiftLeft, // >>

    BitwiseShiftRightEq, // <<=
    BitwiseShiftLeftEq,// >>=

    BitwiseUnsignedShiftRight, // <<<
    BitwiseUnsignedShiftLeft, // >>>

    BitwiseUnsignedShiftRightEq, // <<<=
    BitwiseUnsignedShiftLeftEq,// >>>=

    Not,

    LogicalAnd,
    LogicalAndEq,

    LogicalOr,
    LogicalOrEq,

    EqualityCheck,
    
    GreaterThan,
    GreaterThanEq,
    LesserThan,
    LesserThanEq,

    Reference, // & when used as unary
    ConversionPipe, // |>
    OptionalOperator, // ?
    ErrorOperator // !
}
const BINARY_OPERATORS : [(&'static str, Operator); 35] = [
    ("=", Operator::Assign),

    ("+", Operator::Add),
    ("+=", Operator::AddEq),

    ("-", Operator::Sub),
    ("-=", Operator::SubEq),

    ("*", Operator::Mut),
    ("*=", Operator::MultEq),

    ("/", Operator::Div),
    ("/=", Operator::DivEq),

    ("%", Operator::Mod),
    ("%=", Operator::ModEq),

    ("&", Operator::BitwiseAnd),
    ("&=", Operator::BitwiseAndEq),

    ("^", Operator::Xor),
    ("^=", Operator::XorEq),

    ("|", Operator::BitwiseOr),
    ("|=", Operator::BitwiseOrEq),

    (">>", Operator::BitwiseShiftRight),
    (">>=", Operator::BitwiseShiftRightEq),

    ("<<", Operator::BitwiseShiftLeft),
    ("<<=", Operator::BitwiseShiftLeftEq),

    (">>>", Operator::BitwiseUnsignedShiftRight),
    (">>>=", Operator::BitwiseUnsignedShiftRightEq),

    ("<<<", Operator::BitwiseUnsignedShiftLeft),
    ("<<<=", Operator::BitwiseUnsignedShiftLeftEq),

    ("&&", Operator::LogicalAnd),
    ("&&=", Operator::LogicalAndEq),

    ("||", Operator::LogicalOr),
    ("||=", Operator::LogicalOrEq),

    ("==", Operator::EqualityCheck),

    (">", Operator::GreaterThan),
    (">=", Operator::GreaterThanEq),
    ("<=", Operator::LesserThanEq),

    ("<", Operator::LesserThan),

    ("|>", Operator::ConversionPipe),
];

const UNARY_OPERATIONS : &[(&'static str, Operator)] = &[

];