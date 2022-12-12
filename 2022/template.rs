use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
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
    let input_raw = get_input();

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);

    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_case_1() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
    }
}
