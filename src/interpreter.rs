use std::io::Read;
use std::str::Chars;
use utils;

pub trait CodeBlock {
    fn execute(&self, tape : &mut[u8], pointer: &mut usize);
} 

impl CodeBlock for char {
    fn execute(&self, tape : &mut[u8], pointer: &mut usize) {
        match *self {
            '>' => *pointer += 1,
            '<' => *pointer -= 1,
            '+' => tape[*pointer] += 1u8,
            '-' => tape[*pointer] -= 1u8,
            '.' => utils::output(tape[*pointer]),
            ',' => tape[*pointer] = utils::input(),
            _ => {/* Ignore */}
        }
    }
}

pub struct Program {
    blocks: Vec<Box<CodeBlock>>
}

impl Program {
    pub fn new(program: &String) -> Program {
        let mut instructions = program.chars();
        let mut blocks = Vec::new();
        while let Some(instruction) = instructions.next() {
            blocks.push(get_code_block(&mut instructions, instruction, utils::output, utils::input));
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
    fn new(instructions: &mut Chars) -> Loop {
        let mut blocks = Vec::new();
        while  let Some(instruction) = instructions.next() {
            if instruction == ']' {
                break;
            } else {
                blocks.push(get_code_block(instructions, instruction, utils::output, utils::input));
            }
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

fn get_code_block(instructions: &mut Chars, instruction: char, output : fn(u8), input : fn() ->u8) -> Box<CodeBlock> {
    if instruction == '[' {
        Box::new(Loop::new(instructions))
    } else {
        Box::new(instruction) 
    }
}
