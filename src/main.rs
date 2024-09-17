#![allow(unused)]
mod compiler;

use std::collections::HashMap;

use compiler::parsing::pattern_constants::*;
use compiler::parsing::*;

const HELLO : &str = "Type get_half_word() => halfWordSize;";

fn main() {
    
    // Either()
    let ts = compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
    // println!("{:?}", ts);
    println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));
    
}
