#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParsingError{
    BracketCountError,
    UnknownTokenizationError
}