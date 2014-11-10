fn main() {
    println!("Start of program");
    println!("");

    let program = getProgram();

    let tape = [0u8, ..640000];
    let mut pointer = 0u;
    let mut position = 0u;
    let programLength = program.chars().count();
    while position < programLength {
        position = processInstruction(program, tape, &pointer, position);
    }
    
    println!("");
    println!("End of program.");    
}

fn getProgram () -> &'static str {
    "+ + * - /"
}

fn processInstruction (program : &str, mut tape : &[u8], mut pointer : &uint, pos : uint) -> uint {
    let instruction = getInstruction(program, pos);
    
    match instruction {
        '>' => *pointer += 1u,
        '<' => *pointer -= 1u,
        '+' => tape[*pointer] += 1u8,
        '-' => tape[*pointer] -= 1u8,
        '.' => print!("Monumental error"),
        ',' => panic!("Not implemented operation"),
        '[' => panic!("Not implemented operation"),
        ']' => panic!("Not implemented operation"),
        _ => {/* Ignore */}
    }
    
    pos + 1u
}

fn getInstruction(program : &str, pos : uint) -> char {
    match program.chars().nth(pos) {
        Some(c) => c,
        None => panic!("At the disco")
    }
}

