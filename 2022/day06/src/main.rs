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
    let distinct_chars = 4;

    let mut res = 1;

    for i in input.into_iter() {
        let binrep = to_bin_rep(i);
        cursor.push_back(binrep);
        if cursor.len() > distinct_chars {
            cursor.pop_front().unwrap();
        }
        let a = cursor
            .iter()
            .fold(0, |acc, val| -> usize { acc | val })
            .count_ones();
        if a == distinct_chars as u32 {
            break;
        }
        res += 1;
    }
    return res;
}

fn part2(input_raw: String) -> i32 {
    let input = input_raw.chars();
    let mut cursor = VecDeque::new();
    let distinct_chars = 14;

    let mut res = 1;

    for i in input.into_iter() {
        let binrep = to_bin_rep(i);
        cursor.push_back(binrep);
        if cursor.len() > distinct_chars {
            cursor.pop_front().unwrap();
        }
        let a = cursor
            .iter()
            .fold(0, |acc, val| -> usize { acc | val })
            .count_ones();
        if a == distinct_chars as u32 {
            break;
        }
        res += 1;
    }
    return res;
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

    #[test]
    fn test_part2_case_1() {
        let input: String = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let result = part2(input);
        assert_eq!(result, 19);
    }
    #[test]
    fn test_part2_case_2() {
        let input: String = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let result = part2(input);
        assert_eq!(result, 23);
    }
    #[test]
    fn test_part2_case_3() {
        let input: String = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let result = part2(input);
        assert_eq!(result, 23);
    }
    #[test]
    fn test_part2_case_4() {
        let input: String = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let result = part2(input);
        assert_eq!(result, 29);
    }
}
