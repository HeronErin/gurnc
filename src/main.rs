#![allow(unused)]
mod compiler;
const HELLO : &str = "

int x = 4;

";

fn main() {
    compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
}
