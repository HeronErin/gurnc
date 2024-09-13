#![allow(unused)]
mod compiler;

use compiler::parsing::pattern_constants::*;
use compiler::parsing::pattern_matcher::*;

const HELLO : &str = "i32 x() where foo{}";

fn main() {
    // Either()
    let ts = compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
    
    println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));
    
}
