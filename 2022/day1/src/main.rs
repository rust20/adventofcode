use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn main() {
    let mut data = String::new();
    let f = File::open("../input.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");

    println!("{}", data);

    let a = get_current_working_dir();
    println!("{}", a);
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
