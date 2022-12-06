use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}
fn to_bin_rep(x: char) -> usize {
    let val = x as i32;
    let val_convereted = match val {
        97..=122 => val - 97,
        65..=90 => val - 39,
        _ => 0,
    };
    (1 << val_convereted) as usize
}

fn part1(input_raw: String) -> i32 {
    let input = input_raw.chars();
    let mut cursor = VecDeque::new();

    let mut res = 1;

    for i in input.into_iter() {
        let binrep = to_bin_rep(i);
        cursor.push_back(binrep);
        if cursor.len() > 4 {
            cursor.pop_front().unwrap();
        }
        let a = cursor
            .iter()
            .fold(0, |acc, val| -> usize { acc | val })
            .count_ones();
        if a == 4 {
            break;
        }
        res += 1;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_case_1() {
        let input: String = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let result = part1(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_case_2() {
        let input: String = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let result = part1(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part1_case_3() {
        let input: String = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let result = part1(input);
        assert_eq!(result, 11);
    }
}

fn part2(input_raw: String) -> i32 {
    let input = input_raw.chars();
    let mut cursor = VecDeque::new();

    0
}

fn main() {
    let input_raw = get_input();

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);

    println!("{} {}", res1, res2);
}
