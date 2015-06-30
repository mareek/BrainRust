use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str;

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
    match args.next() {
        None => execute_default_program(),
        Some(arg) => {
            execute(read_file(&Path::new(&arg)));
        }
    }
}

fn read_file(file_path : &Path) -> String {
    let mut buffer = String::new();
    match File::open(file_path) {
        Ok(mut file) => match file.read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => panic!("realy invalid file : {}. {}", file_path.display(), e)
        },
        Err(e) => panic!("invalid file : {}. {}", file_path.display(), e)
    }
}

fn execute_default_program () {
    execute(String::from("[-]>[-]<>+++++++[<+++++++>-]<+++.--."));
}

fn execute(program : String) {
    Program::new(&program).execute(&mut [0u8; 640000], &mut 0usize);
}

trait CodeBlock {
    fn execute(&self, tape : &mut[u8], pointer : &mut usize);
} 

impl CodeBlock for char {
    fn execute(&self, tape : &mut[u8], pointer : &mut usize) {
        match *self {
            '>' => *pointer += 1,
            '<' => *pointer -= 1,
            '+' => tape[*pointer] += 1u8,
            '-' => tape[*pointer] -= 1u8,
            '.' => output(tape, *pointer),
            ',' => input(tape, *pointer),
            _ => {/* Ignore */}
        }
    }
}

struct Program {
    blocks: Vec<Box<CodeBlock>>
}

impl Program {
    fn new(program: &String) -> Program {
        let valid_instructions = ['+', '-', '>', '<', '[', ']', '.', ','];
        let instructions = program.chars().filter(|&c| valid_instructions.iter().any(|&i| i==c)).collect::<Vec<char>>();
        Program { 
            blocks: Vec::new()
        }
    }
}

impl CodeBlock for Program {
    fn execute(&self, tape : &mut[u8], pointer : &mut usize) {
        for block in &self.blocks {
            block.execute(tape, pointer);
        }
    }
}

struct Loop {
    blocks: Vec<Box<CodeBlock>>
}

impl Loop {
    fn new(instructions: &Vec<char>, currentInstruction: &mut usize) -> Loop {
        panic!("At the disco")
    }
}

impl CodeBlock for Loop {
    fn execute(&self, tape : &mut[u8], pointer : &mut usize) {
        while tape[*pointer] != 0u8 {
            for block in &self.blocks {
                block.execute(tape, pointer);
            }
        }
    }
} 

fn output(tape : &[u8], pointer : usize) {
    let value = tape[pointer];
    match str::from_utf8(&[value]) {
        Ok(c) => print!("{}", c),
        Err(_) => print_error(format!("incorrect utf-8 value : {}", value).trim())
    }    
}

fn input(tape : &mut[u8], pointer : usize) {
    let buffer : &mut[u8] = &mut [0u8];
    match io::stdin().read(buffer) {
        Ok(_) => tape[pointer] = buffer[0],
        Err(e) => panic!("no input : {}", e)
    }
}

fn print_error(error_message : &str) {
    println!("");
    println!("Error, {}", error_message);
    println!("");
}