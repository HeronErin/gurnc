

#[derive(Clone)]
pub enum StringTypeBitmask{
    Char = 1, // Single quotes
    TemplatedString = 2, // backticks 
    TypicalString = 4, // Double quotes

    IsMultiLine = 8, // """ OR ```


    IsRaw = 16, // r"
    IsIndentationCorrect = 32, // I"
    IsDoubleIndentationCorrect = 64, //  II"
    IsBytes = 128 // b"
}
impl core::fmt::Debug for StringTypeBitmask{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.clone() as u16).fmt(f)
    }
}

#[derive(Debug)]
pub struct StringLiteral{
    pub string_type_bitmask : StringTypeBitmask, // Bitmask, no enum
    pub string_text_contents : String,
}

impl StringLiteral{
    pub fn new(input : String) -> Self{
        
        todo!()
    }
}