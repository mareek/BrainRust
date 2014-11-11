use std::os;
use std::io::fs::PathExtensions;
use std::io::File;
use std::str;

// http://esolangs.org/wiki/Brainfuck
//  >  Increment the pointer.
//  <  Decrement the pointer
//  +  Increment the byte at the pointer.
//  -  Decrement the byte at the pointer.
//  .  Output the byte at the pointer.
//  ,  Input a byte and store it in the byte at the pointer.
//  [  Jump forward past the matching ] if the byte at the pointer is zero.
//  ]  Jump backward to the matching [ unless the byte at the pointer is zero.

fn main() {
    let args = os::args();
    if args.len() == 1 {
        execute_default_program();
    } else {
        let file_path = Path::new(&args[1]);
        if !file_path.exists() {
            panic!("File {} does not exists", file_path.display());
        } else {
            execute_from_file(&file_path);
        }
    }
}

fn execute_from_file(file_path : &Path) {
    match File::open(file_path).read_to_end() {
        Ok(file_content) => match str::from_utf8(file_content.as_slice()) {
            Some(str_file) => execute(str_file),
            None => panic!("invalid utf-8 file : {}", file_path.display())
        },
        Err(e) => panic!("invalid file : {}. {}", file_path.display(), e)
    }
}

fn execute_default_program () {
    execute("[-]>[-]<>+++++++[<+++++++>-]<+++.--.")
}

fn execute (program : &str) {
    let mut tape = [0u8, ..640000];
    let pointer = &mut 0u;
    let position = &mut 0u;
    let program_length = program.chars().count();
    
    while *position < program_length {
        process_instruction(program, tape, pointer, position);
    }
}

fn process_instruction (program : &str, tape : &mut[u8], pointer : &mut uint, position : &mut uint) {
    let instruction = get_instruction(program, *position);
    
    match instruction {
        '>' => *pointer += 1u,
        '<' => *pointer -= 1u,
        '+' => tape[*pointer] += 1u8,
        '-' => tape[*pointer] -= 1u8,
        '.' => output_current_value(tape, *pointer),
        ',' => panic!("Not implemented operation : ,"),
        '[' => process_opening_bracket(tape[*pointer], program, position),
        ']' => process_closing_bracket(tape[*pointer], program, position),
        _ => {/* Ignore */}
    }
    
    *position += 1u
}

fn get_instruction(program : &str, position : uint) -> char {
    match program.chars().nth(position) {
        Some(c) => c,
        None => panic!("Impossible position : {}", position)
    }
}

fn output_current_value(tape : &[u8], pointer : uint) {
    let value = tape[pointer];
    match str::from_utf8([value]) {
        Some(c) => print!("{}", c),
        None => print_error(format!("incorrect utf-8 value : {}", value).as_slice())
    }    
}

fn process_opening_bracket (value : u8, program : &str, position : &mut uint) {
    if value == 0 {
        while get_instruction(program, *position) != ']' {
            *position += 1;
        }
    }
}

fn process_closing_bracket (value : u8, program : &str, position : &mut uint) {
    if value != 0 {
        while get_instruction(program, *position) != '[' {
            *position -= 1;
        }
    }
}

fn print_error(error_message : &str) {
    println!("");
    println!("Error, {}", error_message);
    println!("");
}