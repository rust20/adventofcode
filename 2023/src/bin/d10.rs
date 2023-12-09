#![feature(test)]

use std::time::Instant;

extern crate test;

fn part1(inp: &str) {
    let sum = inp.len();
    println!("part 1: {}", sum);
}
fn part2(inp: &str) {
    let sum = inp.len();
    println!("part 2: {}", sum);
}

fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        // "input1.txt",
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d10/".to_string();
        let reader = std::fs::read_to_string(path+fi).expect("read input");

        let start1 = Instant::now();
        part1(&reader);
        let start2 = Instant::now();
        part2(&reader);
        let finish = start2.elapsed();

        println!(
            "time1 {:?}, time2 {:?}",
            start2 - start1,
            finish
        );
    }
}

#[bench]
fn pt2(b: &mut test::Bencher) {
    let fi = "tests/d10/input2.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");

    b.iter(|| part2(&reader))
}
