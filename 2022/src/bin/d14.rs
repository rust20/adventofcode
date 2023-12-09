#![allow(dead_code)]

use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufReader, Read};


fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

struct P {
    x: i32,
    y: i32,
}

struct Line {
    p1: P,
    p2: P,
}

enum Bound {
    X(i32, i32),
    Y(i32, i32),
}

struct Constraints {
    l: Line,
    b: Bound,
}

fn parse_lines(input: String) -> Vec<P> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
    }

    RE.captures_iter(&input)
        .map(|x| P {
            x: x[1].parse().unwrap(),
            y: x[2].parse().unwrap(),
        })
        .collect()
}

fn part1(input_raw: String) -> i32 {
    let input = input_raw.split("\n");

    for i in input.into_iter() {
        if i.is_empty() {
            continue;
        }
        parse_lines(i.to_string());
    }
    0
}

// check if y and x satisfies y = f(x) and a >= x >= b or a >= y >= b
// y = mx + c
//

fn part2(input_raw: String) -> i32 {
    let input = input_raw.chars();
    let res = input.count();
    res as i32
}

fn main() {
    let input_raw = get_input("tests/d14/input1.txt");

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);

    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_case_1() {
        assert_eq!(part1(get_input("input_test.txt")), 5);
    }
}
