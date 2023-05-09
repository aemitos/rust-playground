use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("info.txt").expect("Can't open the file.");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Can't read file.");

    println!("File content:\n\n{}", buffer);
}
