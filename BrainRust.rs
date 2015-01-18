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
    let file_path = Path::new(&args[1]);
    let program = read_file(&file_path);
    if args.len() == 1 {
        execute_default_program();
    } else if !file_path.exists() {
        panic!("File {} does not exists", file_path.display());
    } else {
        execute(read_file(&file_path).as_slice());
    }
}

fn read_file(file_path : &Path) -> String {
    match File::open(file_path).read_to_string() {
        Ok(file_content) => file_content,
        Err(e) => panic!("invalid file : {}. {}", file_path.display(), e)
    }
}

fn execute_default_program () {
    execute("[-]>[-]<>+++++++[<+++++++>-]<+++.--.")
}

fn execute(program : &str) {

}
