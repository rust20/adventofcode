use std::fs::File;
use std::io::{BufReader, Read};

fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn part1(input_raw: String) -> i32 {
    let input = input_raw.split("\n");

    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;

    for inst in input.into_iter() {
        if inst.is_empty() {
            continue;
        }

        cycle += 1;

        if (cycle - 20) % 40 == 0 && cycle <= 220 {
            sum += cycle * x;
        }
        if inst != "noop" {
            cycle += 1;
        }
        if (cycle - 20) % 40 == 0 && cycle <= 220 {
            sum += cycle * x;
        }

        if inst != "noop" {
            let a: Vec<&str> = inst.split(" ").collect();
            x += a[1].parse::<i32>().unwrap();
        }
    }

    sum
}

fn part2(input_raw: String) -> i32 {
    let input = input_raw.split("\n");

    let mut cycle = 0;
    let mut x = 1;

    for inst in input.into_iter() {
        if inst.is_empty() {
            continue;
        }

        cycle += 1;
        let mut crt_pos: i32 = (cycle - 1) % 40;

        print!("{}", if (x - crt_pos).abs() < 2 { "#" } else { "." });

        if cycle % 40 == 0 {
            println!("");
        }

        if inst == "noop" {
            continue;
        }
        cycle += 1;
        crt_pos = (cycle - 1) % 40;
        print!("{}", if (x - crt_pos).abs() < 2 { "#" } else { "." });
        if cycle % 40 == 0 {
            println!("");
        }

        let a: Vec<&str> = inst.split(" ").collect();
        x += a[1].parse::<i32>().unwrap();
    }

    0
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
        assert_eq!(part1(get_input("input_test.txt")), 13140);
    }
    #[test]
    fn test_part1_case_2() {
        assert_eq!(part1(get_input("input_test2.txt")), 13140);
    }
}
