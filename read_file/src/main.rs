use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    // Sample from https://www.youtube.com/watch?v=nQqraiMymcU
    let mut file = File::open("info.txt").expect("Can't open the file.");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Can't read file.");

    println!("File content:\n\n{}", buffer);

    // Now the sample from CLion docs - the main returns a value
    let mut f = File::open("info.txt")?;
    let mut buffer2 = String::new();

    f.read_to_string(&mut buffer2)?;

    println!("File content 2:\n\n{}", buffer2);

    Ok(())
}
