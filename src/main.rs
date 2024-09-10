#![allow(unused)]
mod compiler;
const HELLO : &str = "

int x = 0.5_U8";

fn main() {
    let ts = compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
    println!("{:?}", ts);
}
