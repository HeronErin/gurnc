#![allow(unused)]
mod ast;
pub mod tokenizer;
mod number_parser;
mod string_parser;
pub use ast::*;

mod keywords;
pub use keywords::*;

pub enum ParseStage{
    Text,
    DefinitionScan,
    
}
pub enum ParseStageData{
    Text(String)
}