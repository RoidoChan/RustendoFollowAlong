use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let file_name = env::args().nth(1).unwrap();

    let mut file = fs::File::open("LoZ.z64").unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf);
    println!("{}", file_name);
}
