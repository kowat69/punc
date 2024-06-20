mod tokenization;
mod parsing;
mod generation;

use generation::*;
use tokenization::*;
use parsing::*;
use std::process::Command;
use std::env;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("require 2 arguments.");
        return;
    }
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file.");
    let input = contents.as_str();

    let tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    println!("Token###############");
    for token in tokens.iter(){
        println!("{:?} {}", token.kind, token.value);
    }
    let parser = Parser::new(tokens, input);
    let nodes = parser.run();

    println!("\nNode################");
    for node in nodes.iter(){
        node.debug();
    }
    let generator = Generator::new(&nodes, input);
    let codes = generator.gen();

    let output = format!("bits 64
        defalut rel
        global main
        section .text
        main:
        ret
        ");
    println!("{}", output);
}
