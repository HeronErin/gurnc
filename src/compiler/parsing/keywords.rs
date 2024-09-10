macro_rules! genKeyword {
    ($($element:ident, $text: literal), *) => {
        const KEYWORDS : &[&'static str] = &[$($text, )*];


        enum Keyword{
            $(
                $element,
            )*
        }
        impl Keyword{
            pub fn try_from_string(x : &str) ->Option<Self>{
                if x.len() == 0 { return None }
                let mut new_string = x.to_string();

                unsafe{
                    let bts = new_string.as_bytes_mut();
                    bts[0] = bts[0].to_ascii_uppercase();
                }

                $(
                    if new_string.starts_with($text){
                        return Some(Self::$element)
                    }
                )*
                None
            }
        }
    };
}

genKeyword!(
    If, "if", 
    ElseIf, "else if",
    Else, "else",
    While, "while",
    Do, "do",
    For, "for",
    Match, "match",
    Var, "Var",

    Trait, "trait",
    Struct, "struct",
    Enum, "enum",
    Import, "import",

    Pub, "pub",
    Private, "private",

    Where, "where",
    Const, "const",
    Defer, "defer",

    Static, "static",
    Inline, "inline",

    Runtime, "runtime",
    Comptime, "comptime",
    Pure, "pure"
);
