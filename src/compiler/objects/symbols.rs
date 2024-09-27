
pub struct ResolvedSymbol{

}
pub enum KnownSymbolUsage{
    Function,
    Macro,
    Type,
    Variable
}

pub struct SymbolContext{
    usage : Option<KnownSymbolUsage>
}
pub struct UnresolvedSymbol{
    name : String,

}