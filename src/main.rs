#![allow(unused)]
mod compiler;
const HELLO : &str = "

stringify!(foo)";

fn main() {
    let ts = compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
    println!("{:?}", ts);
}
