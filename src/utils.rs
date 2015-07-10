use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str;

pub fn read_file(file_path : &Path) -> String {
    let mut buffer = String::new();
    match File::open(file_path) {
        Ok(mut file) => match file.read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => panic!("realy invalid file : {}. {}", file_path.display(), e)
        },
        Err(e) => panic!("invalid file : {}. {}", file_path.display(), e)
    }
}

pub fn output(value : u8) {
    match str::from_utf8(&[value]) {
        Ok(c) => print!("{}", c),
        Err(_) => print_error(format!("incorrect utf-8 value : {}", value).trim())
    }    
}

pub fn input() -> u8 {
    let mut buffer = String::new();    
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.into_bytes()[0]
}

fn print_error(error_message : &str) {
    println!("");
    println!("Error, {}", error_message);
    println!("");
}
