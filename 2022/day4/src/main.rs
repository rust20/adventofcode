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

    let mut sum: i32 = 0;

    for pair in input.into_iter() {
        if pair == "" {
            continue;
        }
        let pairs: Vec<Vec<i32>> = pair
            .split(",")
            .map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();

        if (pairs[0][0] <= pairs[1][0] && pairs[1][1] <= pairs[0][1])
            || (pairs[1][0] <= pairs[0][0] && pairs[0][1] <= pairs[1][1])
        {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn part2() {
    let input_raw = get_input();
    let input = input_raw.split("\n");

    let mut sum: i32 = 0;

    for pair in input.into_iter() {
        if pair == "" {
            continue;
        }
        let pairs: Vec<Vec<i32>> = pair
            .split(",")
            .map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();

        if !(pairs[0][1] < pairs[1][0] || pairs[0][0] > pairs[1][1]) {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
