#![allow(unused)]
mod compiler;
const HELLO : &str = "

stringify!(foo) x = 4;";

fn main() {
    // Either()
    let ts = compiler::parsing::tokenizer::tokenize_text(HELLO.to_string()).unwrap();
    println!("{:?}", ts);
}
