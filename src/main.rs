use std::process::Command;

use std::env;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file.");
    let input = contents.as_str();

    let output = format!("bits 64
        defalut rel
        global main
        section .text
        main:
        mov rax, {}
        ret
        ", input);
    println!("{}", output);
}
