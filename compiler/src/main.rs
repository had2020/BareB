use std::{env, path::Path};

fn main() {
    if env::args().len() > 1 {
        let arg0: String = env::args().nth(1).expect("Failed to parse CLI args!");
        let contents: Vec<u8> = std::fs::read(Path::new(&arg0)).expect("Failed to read file!");
        for i in contents {
            // TODO remove
            print!("{}", i as char);
        }
    } else {
        println!("Enter filename as arg!");
    }
}
