use std::{collections::HashMap, time::Instant};

pub fn part1(inp: &str) -> i64 {
    let sum = inp.len();
    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let sum = inp.len();
    sum as i64
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        // "input1.txt",
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d21/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
