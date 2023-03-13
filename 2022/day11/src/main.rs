use regex::{RegexSet, Regex};
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

#[derive(Clone)]
enum Operation {
    None,
    MultiplySelf,
    AddSelf,
    Multiply(u64),
    Add(u64),
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    target_throw_true: u64,
    target_throw_false: u64,
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
    let mut monkeys = Vec::new();
    let mut curr_monkey = Monkey::default();

    let parser = vec![
        r"^$",
        r"Monkey (\d+):",
        r"\s+Starting items: (.*)",
        r"\s+Operation: new = old (\+|\*) (.*)",
        r"\s+Test: divisible by (.*)",
        r"\s+If true: throw to monkey (\d+)",
        r"\s+If false: throw to monkey (\d+)",
    ];

    let set = RegexSet::new(&parser).unwrap();
    let matchers: Vec<Regex> = parser.clone().into_iter().map(|x| {Regex::new(x).unwrap()}).collect();
    for input in input_raw.split("\n").into_iter() {
        // Iterate over and collect all of the matches.
        let matches = set.matches(input).into_iter();
        matches.for_each(|x| match x {
            0 => {
                monkeys.push(curr_monkey.clone());
            }
            1 => {
                curr_monkey = Monkey::default();
            }
            2 => {
                let matches = matchers[x].captures(input).unwrap();
                let items = matches.get(1).unwrap().as_str().split(", ");
                curr_monkey.items = items.map(|x| x.parse::<u64>().unwrap()).collect();
            }
            3 => {
                let matches = matchers[x].captures(input).unwrap();
                let operator = matches.get(1).unwrap().as_str();
                let value = matches.get(2).unwrap().as_str();
                curr_monkey.operation = match (operator, value) {
                    ("*", "old") => Operation::MultiplySelf,
                    ("+", "old") => Operation::AddSelf,
                    ("*", _) => Operation::Multiply(value.parse::<u64>().unwrap()),
                    ("+", _) => Operation::Add(value.parse::<u64>().unwrap()),
                    _ => {Operation::None}
                };
            }
            4 => {
                let matches = matchers[x].captures(input).unwrap();
                curr_monkey.test_divisible = matches.get(1).unwrap().as_str().parse::<u64>().unwrap();
            }
            5 => {
                let matches = matchers[x].captures(input).unwrap();
                curr_monkey.target_throw_true = matches.get(1).unwrap().as_str().parse::<u64>().unwrap();
            }
            6 => {
                let matches = matchers[x].captures(input).unwrap();
                curr_monkey.target_throw_false = matches.get(1).unwrap().as_str().parse::<u64>().unwrap();
            }
            _ => {}
        });
    }
    monkeys
}

fn part1(input_raw: String) -> u64 {
    let input = input_raw.chars();


    let mut monkeys = get_monkeys(input.as_str().to_string());
    let mut monkey_inspect_count = vec![0; monkeys.len()];

    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let monkey = monkeys[monkey_idx].clone();
            for item in monkey.items.iter() {
                
                let item_worry_level: u64 = match monkey.operation {
                    Operation::None => *item,
                    Operation::MultiplySelf => item * item,
                    Operation::AddSelf => item * 2,
                    Operation::Multiply(val) => item * val,
                    Operation::Add(val) => item + val,
                } / 3;
                if item_worry_level % monkey.test_divisible == 0 {
                    monkeys[monkey.target_throw_true as usize].items.push(item_worry_level);
                } else {
                    monkeys[monkey.target_throw_false as usize].items.push(item_worry_level);
                }
            }
            monkey_inspect_count[monkey_idx] += monkey.items.len();
            monkeys[monkey_idx].items = Vec::new();
        }
    }

    monkey_inspect_count.sort();


    let res = monkey_inspect_count[monkeys.len() - 1] * monkey_inspect_count[monkeys.len() - 2];
    res as u64
}

fn part2(input_raw: String) -> u64 {
    let input = input_raw.chars();

    let mut monkeys = get_monkeys(input.as_str().to_string());
    let mut monkey_inspect_count = vec![0; monkeys.len()];

    let cap: u64 = monkeys.clone().iter().map(|x| x.test_divisible).product();

    for _round in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let monkey = monkeys[monkey_idx].clone();
            for item in monkey.items.iter() {

                let test = cap;
                
                let item_worry_level: u64 = match monkey.operation {
                    Operation::None => *item,
                    Operation::MultiplySelf => (item % test) * (item % test),
                    Operation::AddSelf => (item % test) * 2,
                    Operation::Multiply(val) => (item % test) * (val % test),
                    Operation::Add(val) => (item % test) + (val % test),
                } % test;

                if item_worry_level % monkey.test_divisible == 0 {
                    monkeys[monkey.target_throw_true as usize].items.push(item_worry_level);
                } else {
                    monkeys[monkey.target_throw_false as usize].items.push(item_worry_level);
                }
            }
            monkey_inspect_count[monkey_idx] += monkey.items.len();
            monkeys[monkey_idx].items = Vec::new();
        }
    }

    monkey_inspect_count.sort();

    let res = monkey_inspect_count[monkeys.len() - 1] as u64 * monkey_inspect_count[monkeys.len() - 2] as u64;
    res as u64
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
    #[test]
    fn test_part2_case_1() {
        assert_eq!(part2(get_input("input_test.txt")), 2713310158);
    }
}
