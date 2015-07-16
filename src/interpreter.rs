use std::io::Read;
use std::str::Chars;
use utils;

pub trait CodeBlock {
    fn execute(&self, tape : &mut[u8], pointer: &mut usize);
} 

pub struct Program {
    blocks: Vec<Box<CodeBlock>>
}

impl Program {
    pub fn new(program: &String, output : fn(u8), input : fn() ->u8) -> Program {
        let mut instructions = program.chars();
        let mut blocks = Vec::new();
        while let Some(instruction) = instructions.next() {
            blocks.push(get_code_block(&mut instructions, instruction, output, input));
        }
        Program { 
            blocks: blocks
        }
    }

    pub fn new_command_line(program: &String) -> Program {
        Program::new(program, utils::output, utils::input)
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
    fn new(instructions: &mut Chars, output : fn(u8), input : fn() ->u8) -> Loop {
        let mut blocks = Vec::new();
        while  let Some(instruction) = instructions.next() {
            if instruction == ']' {
                break;
            } else {
                blocks.push(get_code_block(instructions, instruction, output, input));
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

struct Instruction {
    value : char,
    output : fn(u8),
    input : fn() -> u8
}

impl Instruction {
    fn new(value: char,  output : fn(u8), input : fn() -> u8) -> Instruction {
        Instruction {
            value: value,
            output: output, 
            input: input 
        }        
    }
}

impl CodeBlock for Instruction {
    fn execute(&self, tape : &mut[u8], pointer: &mut usize) {
        match self.value {
            '>' => *pointer += 1,
            '<' => *pointer -= 1,
            '+' => tape[*pointer] += 1u8,
            '-' => tape[*pointer] -= 1u8,
            '.' => self.output(tape[*pointer]),
            ',' => tape[*pointer] = self.input(),
            _ => {/* Ignore */}
        }
    }
}

fn get_code_block(instructions: &mut Chars, instruction: char, output : fn(u8), input : fn() ->u8) -> Box<CodeBlock> {
    if instruction == '[' {
        Box::new(Loop::new(instructions, output, input))
    } else {
        Box::new(Instruction::new(instruction, output, input)) 
    }
}
