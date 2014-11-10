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
    println!("Start of program");
    println!("");

    let program = get_program();

    let mut tape = [0u8, ..640000];
    let pointer = &mut 0u;
    let position = &mut 0u;
    let program_length = program.chars().count();
    
    while *position < program_length {
        process_instruction(program, tape, pointer, position);
    }
    
    println!("");
    println!("End of program.");    
}

fn get_program () -> &'static str {
    "[-]>[-]<>+++++++[<+++++++>-]<+++.--."
}

fn process_instruction (program : &str, tape : &mut[u8], pointer : &mut uint, position : &mut uint) {
    let instruction = get_instruction(program, *position);
    
    match instruction {
        '>' => *pointer += 1u,
        '<' => *pointer -= 1u,
        '+' => tape[*pointer] += 1u8,
        '-' => tape[*pointer] -= 1u8,
        '.' => output_current_value(tape, *pointer),
        ',' => panic!("Not implemented operation"),
        '[' => process_opening_bracket(tape[*pointer], program, position),
        ']' => process_closing_bracket(tape[*pointer], program, position),
        _ => {/* Ignore */}
    }
    
    *position += 1u
}

fn get_instruction(program : &str, pos : uint) -> char {
    match program.chars().nth(pos) {
        Some(c) => c,
        None => panic!("At the disco")
    }
}

fn output_current_value(tape : &[u8], pointer : uint) {
    let value = tape[pointer];
    match std::str::from_utf8([value]) {
        Some(c) => print!("{}", c),
        None => {
            println!("");
            println!("Error, incorrect utf-8 value : {}", value);
            println!("");
        }
    }    
}


//  [  Jump forward past the matching ] if the byte at the pointer is zero.
fn process_opening_bracket (value : u8, program : &str, position : &mut uint) {
    if value == 0 {
        while get_instruction(program, *position) != ']' {
            *position += 1;
        }
    }
}

//  ]  Jump backward to the matching [ unless the byte at the pointer is zero.
fn process_closing_bracket (value : u8, program : &str, position : &mut uint) {
    if value != 0 {
        while get_instruction(program, *position) != '[' {
            *position -= 1;
        }
    }
}

