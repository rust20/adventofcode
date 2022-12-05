use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn part1() {
    let input_raw = get_input();
    let input = input_raw.split("\n");

    let input_r1: Vec<&str> = input.collect();

    let mut tmp = input_r1.split(|x| x.is_empty());
    let crates: Vec<Vec<char>> = tmp
        .next()
        .unwrap()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let moves = tmp.next().unwrap();

    let mut col: usize = 0;

    let crates_index = crates.last().unwrap();

    let col_length = crates_index.len();
    let row_length = crates.len();

    let max_crate = crates_index
        .into_iter()
        .filter(|x| **x != ' ')
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let mut crate_stack = vec![Vec::new(); max_crate as usize];

    let mut crate_i: usize = 0;

    // generate stack of crates
    while col < col_length {
        if crates.last().unwrap()[col] == ' ' {
            col += 1;
            continue;
        }

        let mut row: usize = row_length - 1;
        while row > 0 {
            row -= 1;
            let curr_char = crates[row][col];
            if curr_char == ' ' {
                continue;
            }
            crate_stack[crate_i].push(curr_char);
        }
        crate_i += 1;
        col += 1;
    }

    for i in 0..crate_stack.len() {
        println!("{} {:?}", i + 1, crate_stack[i]);
    }

    // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for instruction in moves {
        let cap = re.captures(instruction).unwrap();
        println!("move: {} from: {} to: {}", &cap[1], &cap[2], &cap[3]);
        let move_times = cap[1].parse::<i32>().unwrap();
        let from = cap[2].parse::<usize>().unwrap();
        let to = cap[3].parse::<usize>().unwrap();

        (0..move_times).for_each(|_| {
            let item = crate_stack[from - 1].pop().unwrap();
            crate_stack[to - 1].push(item);
        });
    }

    let res: String = crate_stack.iter().map(|x| x.last().unwrap()).collect();

    println!("{}", res);

    println!("{:?}", crate_stack);
    // println!("{:?}", crates);
    // println!("{:?}", moves);
}

fn main() {
    part1();
}
