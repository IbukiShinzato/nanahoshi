use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("ELF file required");

    let data = fs::read(path).expect("failed to read file");

    println!("{:02x}", data[0]);
    println!("{:02x}", data[1]);
    println!("{:02x}", data[2]);
    println!("{:02x}", data[3]);
}
