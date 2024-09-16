#![allow(unused)]
mod compiler;

use compiler::parsing::pattern_constants::*;
use compiler::parsing::*;

const HELLO : &str = "&[T] foo(comptime u8 t, u8 t) where T implements int => 1 |> T;";

fn main() {
    // Either()
    let ts = compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
    // println!("{:?}", ts);
    println!("{:?}", test_tokens_against(FUNCTION_DECLARATION, &ts));
    
}
