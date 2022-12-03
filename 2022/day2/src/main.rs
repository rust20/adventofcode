use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn main() {
    let input_raw = get_input();
    let input = input_raw.split("\n");

    let mut strategy = HashMap::new();

    // A X ROCK
    // B Y PAPER
    // C Z SCISSOR

    strategy.insert("A X", 3 + 1);
    strategy.insert("A Y", 6 + 2);
    strategy.insert("A Z", 0 + 3);

    strategy.insert("B X", 0 + 1);
    strategy.insert("B Y", 3 + 2);
    strategy.insert("B Z", 6 + 3);

    strategy.insert("C X", 6 + 1);
    strategy.insert("C Y", 0 + 2);
    strategy.insert("C Z", 3 + 3);

    let mut sum = 0;
    for i in input.into_iter() {
        if let Some(point) = strategy.get(i) {
            sum += point
        }
    }

    println!("{}", sum);
}
