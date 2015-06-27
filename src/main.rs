use std::env;
use std::fs::File;
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
    match args.next() {
        None => execute_default_program(),
        Some(arg) => {
            execute(read_file(&Path::new(&arg)).trim());
        }
    }
}

fn read_file<'a>(file_path : &Path) -> String {
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
    execute("[-]>[-]<>+++++++[<+++++++>-]<+++.--.");
}

fn execute(program : &str)-> String {
    let executable = Program::new(program);
    panic!("At the disco!\n{}", program)
}

trait CodeBlock {
    fn execute(&self);
} 

struct Program {
    raw: Vec<char>,
    instructions: Vec<char>,
    blocks: Vec<Box<CodeBlock>>
}

impl Program {
    pub fn new(program: &str) -> Program {
        Program { 
            raw: Vec::new(),
            instructions: Vec::new(),
            blocks: Vec::new()
        }
    }
}

impl CodeBlock for Program {
    fn execute(&self) {
        for block in &self.blocks {
            block.execute();
        }
    }
}