use std::env;
use read_input::prelude::*;

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let mut block = String::from("// =");
    let mut whitespace = String::from("// ");

    for i in 1..input.len() { 
        block.push_str(&"=".to_string());
    }
    
    println!("\n{0}\n{1}{2}\n{3}\n", block, whitespace, input, block);
}
