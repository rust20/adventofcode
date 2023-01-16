use regex::RegexSet;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

enum Operation {
    None,
    MultiplySelf,
    AddSelf,
    Multiply(i32),
    Add(i32),
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test_divisible: i32,
    target_throw_true: i32,
    target_throw_false: i32,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            operation: Operation::None,
            test_divisible: 0,
            target_throw_true: 0,
            target_throw_false: 0,
        }
    }
}

impl Monkey {}

fn get_monkeys(input_raw: String) -> Vec<Monkey> {
    let monkeys = Vec::new();
    let mut curr_monkey = Monkey::default();

    // let re_index = Regex::new(r"Monkey (\d):").unwrap();
    // let re_items = Regex::new(r"Monkey (\d):").unwrap();
    // let re_op = Regex::new(r"Monkey (\d):").unwrap();
    // let re_test = Regex::new(r"Monkey (\d):").unwrap();
    // let re_test_true = Regex::new(r"Monkey (\d):").unwrap();
    // let re_test_false = Regex::new(r"Monkey (\d):").unwrap();
    //
    let parser = vec![
        r"Monkey (\d+):",
        r"\s+Starting items: (.*)",
        r"\s+Operation: new = old (\+|\*) (.*)",
        r"\s+Test: divisible by (.*)",
        r"\s+If true: throw to monkey (\d+)",
        r"\s+If false: throw to monkey (\d+)",
    ];

    let set = RegexSet::new(&parser).unwrap();
    for input in input_raw.split("\n").into_iter() {
        // Iterate over and collect all of the matches.
        let matches = set.matches(input).into_iter();
        matches.for_each(|x| match x {
            0 => if monkeys.len() == 0 {},
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            _ => {}
        });
    }
    monkeys
}

fn part1(input_raw: String) -> i32 {
    let input = input_raw.chars();

    let res = input.count();
    res as i32
}

fn part2(input_raw: String) -> i32 {
    let input = input_raw.chars();
    let res = input.count();
    res as i32
}

fn main() {
    let input_raw = get_input("input.txt");

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);

    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_case_1() {
        assert_eq!(part1(get_input("input_test.txt")), 10605);
    }
}
