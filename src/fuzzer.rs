use rand;
use rand::Rng;

fn generate_single_program() -> String {
    let corpus = ['<', '>', '+', '-', '[', ']', '.'];
    let mut rng = rand::thread_rng();
    let mut program = String::new();
    for _i in 0..rng.gen_range(10, 100) {
        program.push(*rng.choose(&corpus).unwrap());
    }
    
    program
}