use std::io;
use std::io::Read;
use std::io::Write;
use std::str::Chars;

pub trait CodeBlock {
    fn execute(&self, tape: &mut [u8], pointer: &mut usize);
}

pub struct Program {
    blocks: Vec<Box<CodeBlock>>
}

impl Program {
    pub fn new(program: &String) -> Program {
        let mut instructions = program.chars();
        let mut blocks = Vec::new();
        while let Some(instruction) = instructions.next() {
            blocks.push(get_code_block(&mut instructions, instruction));
        }
        Program { blocks: blocks }
    }

    pub fn new_command_line(program: &String) -> Program {
        Program::new(program)
    }
}

impl CodeBlock for Program {
    fn execute(&self, tape: &mut [u8], pointer: &mut usize) {
        for block in &self.blocks {
            block.execute(tape, pointer);
        }
    }
}

struct Loop {
    blocks: Vec<Box<CodeBlock>>,
}

impl Loop {
    fn new(instructions: &mut Chars) -> Loop {
        let mut blocks = Vec::new();
        while let Some(instruction) = instructions.next() {
            if instruction == ']' {
                break;
            } else {
                blocks.push(get_code_block(instructions, instruction));
            }
        }

        Loop { blocks: blocks }
    }
}

impl CodeBlock for Loop {
    fn execute(&self, tape: &mut [u8], pointer: &mut usize) {
        while tape[*pointer] != 0u8 {
            for block in &self.blocks {
                block.execute(tape, pointer);
            }
        }
    }
}

struct Instruction {
    value: char
}

impl Instruction {
    fn new(value: char) -> Instruction {
        Instruction { value: value }
    }

    fn input(&self, tape: &mut [u8], pointer: &mut usize) {
        let mut stdin = io::stdin();
        stdin.read(&mut tape[*pointer..*pointer+1]).unwrap();
    }

    fn output(&self, tape: &mut [u8], pointer: &mut usize) {
        let mut stdout = io::stdout();
        stdout.write(&mut tape[*pointer..*pointer+1]).unwrap(); 
        stdout.flush().unwrap() ;
    }
}

impl CodeBlock for Instruction {
    fn execute(&self, tape: &mut [u8], pointer: &mut usize) {
        match self.value {
            '>' => *pointer += 1,
            '<' => *pointer -= 1,
            '+' => tape[*pointer] += 1u8,
            '-' => tape[*pointer] -= 1u8,
            '.' => self.output(tape, pointer),
            ',' => self.input(tape, pointer),
            _ => { /* Ignore */ }
        }
    }
}

fn get_code_block(instructions: &mut Chars, instruction: char) -> Box<CodeBlock> {
    if instruction == '[' {
        Box::new(Loop::new(instructions))
    } else {
        Box::new(Instruction::new(instruction))
    }
}
