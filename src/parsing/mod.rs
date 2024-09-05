#![allow(unused)]
mod ast;
pub mod tokenizer;
mod number_parser;
pub use ast::*;

pub enum ParseStage{
    Text,
    DefinitionScan,
    
}
pub enum ParseStageData{
    Text(String)
}