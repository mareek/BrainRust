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
        '.' => print!("{}", std::str::from_utf8([tape[*pointer]])),
        ',' => panic!("Not implemented operation"),
        '[' => panic!("Not implemented operation"),
        ']' => panic!("Not implemented operation"),
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
