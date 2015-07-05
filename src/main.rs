mod utils;

use std::env;
use std::io::Read;
use std::path::Path;

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
    Program::new(&program).execute(&mut [0u8; 640000], &mut 0usize);
}

trait CodeBlock {
    fn execute(&self, tape : &mut[u8], pointer: &mut usize);
} 

impl CodeBlock for char {
    fn execute(&self, tape : &mut[u8], pointer: &mut usize) {
        match *self {
            '>' => *pointer += 1,
            '<' => *pointer -= 1,
            '+' => tape[*pointer] += 1u8,
            '-' => tape[*pointer] -= 1u8,
            '.' => utils::output(tape, *pointer),
            ',' => utils::input(tape, *pointer),
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
        
        let mut blocks = Vec::new();
        let mut current_instruction = 0;
        while current_instruction < instructions.len() {
            blocks.push(get_code_block(&instructions, &mut current_instruction));
            current_instruction += 1;            
        }
        Program { 
            blocks: blocks
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
    fn new(instructions: &Vec<char>, current_instruction: &mut usize) -> Loop {
        let mut blocks = Vec::new();
        *current_instruction += 1;
        while *current_instruction < instructions.len() && instructions[*current_instruction] != ']' {
            blocks.push(get_code_block(instructions, current_instruction));
            *current_instruction += 1;            
        }
        
        Loop {
            blocks: blocks
        }        
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

fn get_code_block(instructions: &Vec<char>, current_instruction: &mut usize) -> Box<CodeBlock> {
    if instructions[*current_instruction] == '[' {
        Box::new(Loop::new(instructions, current_instruction))
    } else {
        Box::new(instructions[*current_instruction]) 
    }
}
