use std::{env, path::Path};

fn main() {
    if env::args().len() > 1 {
        let filepath: String = env::args().nth(1).expect("Failed to specfiy filepath!");
        let arch: String = env::args().nth(2).expect("Failed to specfiy arch!");
        let contents: Vec<u8> = std::fs::read(Path::new(&filepath)).expect("Failed to read file!");
        let mut tok_buf: [u8; 64] = [0_u8; 64];
        let mut tok_buf_ptr: usize = 0;
        for i in contents {
            // cargo run /Users/hadrian/Developer/GitRepos/BareB/tests/hello_world.bb BBareVM
            print!("{} ", i as char); // debug
            if i != (' ' as u8) {
                tok_buf[tok_buf_ptr] = i;
                tok_buf_ptr += 1;
            } else {
                todo!("check token");
                if
                tok_buf = [0_u8; 64];
            }
        }
    } else {
        println!("Enter filename as arg!");
    }
}
