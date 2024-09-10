#[derive(Clone, Copy)]
pub enum Operator {
    Assign,

    Add,
    AddEq,

    SubEq,
    Sub,

    Mut,
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
    BitwiseShiftLeft,  // >>

    BitwiseShiftRightEq, // <<=
    BitwiseShiftLeftEq,  // >>=

    BitwiseUnsignedShiftRight, // <<<
    BitwiseUnsignedShiftLeft,  // >>>

    BitwiseUnsignedShiftRightEq, // <<<=
    BitwiseUnsignedShiftLeftEq,  // >>>=

    Not,

    LogicalAnd,
    LogicalAndEq,

    LogicalOr,
    LogicalOrEq,

    EqualityCheck,
    NotEqualityCheck,

    GreaterThan,
    GreaterThanEq,
    LesserThan,
    LesserThanEq,

    Dereference,      // * when used as unary
    Reference,        // & when used as unary
    ConversionPipe,   // |>
    OptionalOperator, // ?
    ErrorOperator,    // !
}
const BINARY_OPERATORS: [(&'static str, Operator); 36] = [
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
    ("!=", Operator::NotEqualityCheck),
    (">", Operator::GreaterThan),
    (">=", Operator::GreaterThanEq),
    ("<=", Operator::LesserThanEq),
    ("<", Operator::LesserThan),
    ("|>", Operator::ConversionPipe),
];


const UNARY_PRE: [(&'static str, Operator); 3] = [
    ("*", Operator::Dereference),       // *ptr
    ("&", Operator::Reference),         // &u8
    ("~", Operator::Not),               // ~1u8 == 254
];
const UNARY_POST : [(&'static str, Operator); 2] = [
    ("?", Operator::OptionalOperator), // [].get(4)?
    ("!", Operator::ErrorOperator),    // file.read(99)!;
];



pub fn operator_test(str : &str, potential_binary : bool, potential_pre_unary : bool, potential_post_unary : bool) -> Option<(usize, Operator)>{
    if (potential_binary){
        if let Some(bin) = BINARY_OPERATORS.iter().filter(|opr| str.starts_with(opr.0)).next(){
            return Some((bin.0.len(), bin.1));
        }
    }
    if (potential_pre_unary){
        if let Some(bin) = UNARY_PRE.iter().filter(|opr| str.starts_with(opr.0)).next(){
            return Some((bin.0.len(), bin.1));
        }
    }
    if (potential_post_unary){
        if let Some(bin) = UNARY_POST.iter().filter(|opr| str.starts_with(opr.0)).next(){
            return Some((bin.0.len(), bin.1));
        }
    }
    None
}
