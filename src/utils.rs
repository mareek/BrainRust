use std::fs::File;
use std::io::Read;
use std::path::Path;

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