#![allow(unused)]
mod ast;
pub mod tokenizer;
mod number_parser;
mod string_parser;
pub use ast::*;

mod keywords;
mod pattern_matcher;
pub use pattern_matcher::*;
pub mod type_parser;

pub mod pattern_constants;
pub use keywords::*;

pub enum ParseStage{
    Text,
    DefinitionScan,
    
}
pub enum ParseStageData{
    Text(String),
    Tokens(Vec<tokenizer::Token>),
}