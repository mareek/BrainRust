mod utils;
mod interpreter;
mod fuzzer;

extern crate rand;

use std::env;
use std::path::Path;
use interpreter::CodeBlock;
use interpreter::Program;

// +    Increases element under pointer
// -    Decrases element under pointer
// >    Increases pointer
// <    Decreases pointer
// [    Starts loop, flag under pointer
// ]    Indicates end of loop
// .    Outputs ASCII code under pointer
// ,    Reads char and stores ASCII under ptr

fn main(){
    let mut args = env::args();
    args.next();
    match args.next() {
        None => execute_default_program(),
        Some(arg) => execute(utils::read_file(&Path::new(&arg)))
    }
    
    println!("");    
}

fn execute_default_program () {
    execute(String::from("[-]>[-]<>+++++++[<+++++++>-]<+++.--."));
}

fn execute(program : String) {
    Program::new_command_line(&program).execute(&mut [0u8; 640000], &mut 0usize);
}
