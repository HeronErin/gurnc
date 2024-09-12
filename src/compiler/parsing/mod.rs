#![allow(unused)]
mod ast;
pub mod tokenizer;
mod number_parser;
mod string_parser;
pub use ast::*;

mod keywords;
mod pattern_matcher;
mod pattern_constants;
pub use keywords::*;

pub enum ParseStage{
    Text,
    DefinitionScan,
    
}
pub enum ParseStageData{
    Text(String),
    Tokens(Vec<tokenizer::Token>),
}