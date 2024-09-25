#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Primitive {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISIZE,
    U8,
    U16,
    U32,
    U64,
    U128,
    USIZE,
    F32,
    F64,
    F128,
    REAL,
}