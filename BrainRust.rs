use std::io;
use std::io::File;
use std::io::fs::PathExtensions;
use std::os;
use std::str;

// +    Increases element under pointer
// -    Decrases element under pointer
// >    Increases pointer
// <    Decreases pointer
// [    Starts loop, flag under pointer
// ]    Indicates end of loop
// .    Outputs ASCII code under pointer
// ,    Reads char and stores ASCII under ptr

fn main(){
    let args = os::args();
    let program = readFile();
    let file_path = Path::new(&args[1]);
    if args.len() == 1 {
        execute_default_program();
    } else {
        if !file_path.exists() {
            panic!("File {} does not exists", file_path.display());
        } else {
            let program = read_file(&file_path);
        }
    }
}

fn read_file(file_path : &Path) -> &str {
    match File::open(file_path).read_to_end() {
        Ok(file_content) => match str::from_utf8(file_content.as_slice()) {
            Some(str_file) => return str_file,
            None => panic!("invalid utf-8 file : {}", file_path.display())
        },
        Err(e) => panic!("invalid file : {}. {}", file_path.display(), e)
    }

}
