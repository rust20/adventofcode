use std::time::Instant;
extern crate test;

pub fn part1(inp: &str) {
    let mut sum = 0;

    for line in inp.lines() {
        let mut splitted = line
            .split(" ")
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut diffs = Vec::new();
        while !splitted.iter().all(|v| *v == 0) {
            for i in 0..(splitted.len() - 1) {
                splitted[i] = splitted[i + 1] - splitted[i];
            }
            diffs.push(splitted.pop().unwrap());
        }
        sum += diffs.iter().sum::<i32>();
    }
    println!("part 1: {}", sum);
}
pub fn part2(inp: &str) {
    let mut sum = 0;

    for line in inp.lines() {
        let mut history = line
            .split(" ")
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut diffs = Vec::new();
        while !history.iter().all(|v| *v == 0) {
            diffs.push(history[0]);

            for i in 0..(history.len() - 1) {
                history[i] = history[i + 1] - history[i];
            }

            history.pop();
        }
        sum += diffs.iter().rev().fold(0, |acc, v| v - acc);
    }

    println!("part 2: {}", sum);
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt"
    ];

    for fi in inputs {
        let path = "tests/d9/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        part1(&reader);
        let start2 = Instant::now();
        part2(&reader);
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
