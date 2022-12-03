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

    let to_rank = |x| -> usize {
        (match x {
            97..=122 => x - 97,
            65..=90 => x - 39,
            _ => 0,
        }) as usize
    };

    let mut sum = 0;

    for i in input.into_iter() {
        let mut check_right = vec![false; 52];
        let mut check_left = vec![false; 52];
        let mut dup = 0;

        let mut tmp = 0 as char;

        let comp_size = i.len() / 2 as usize;
        let k: Vec<char> = i.chars().collect();

        for ch_index in 0..comp_size {
            let item_left = k[ch_index] as i32;
            let item_right = k[ch_index + comp_size] as i32;

            let rank_left = to_rank(item_left);
            let rank_right = to_rank(item_right);

            check_left[rank_left] = true;
            check_right[rank_right] = true;

            if check_left[rank_right] {
                dup = rank_right;
                tmp = char::from_u32(item_right as u32).unwrap();
                break;
            }
            if check_right[rank_left] {
                dup = rank_left;
                tmp = char::from_u32(item_left as u32).unwrap();
                break;
            }
        }

        sum += dup;
        if comp_size != 0 {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn part2() {
    let input_raw = get_input();
    let input = input_raw.split("\n");

    let to_rank = |x| -> usize {
        (match x {
            97..=122 => x - 97,
            65..=90 => x - 39,
            _ => 0,
        }) as usize
    };

    let mut sum = 0;

    for i in input.into_iter() {
        let mut check_right = vec![false; 52];
        let mut check_left = vec![false; 52];
        let mut dup = 0;

        let mut tmp = 0 as char;

        let comp_size = i.len() / 2 as usize;
        let k: Vec<char> = i.chars().collect();

        for ch_index in 0..comp_size {
            let item_left = k[ch_index] as i32;
            let item_right = k[ch_index + comp_size] as i32;

            let rank_left = to_rank(item_left);
            let rank_right = to_rank(item_right);

            check_left[rank_left] = true;
            check_right[rank_right] = true;

            if check_left[rank_right] {
                dup = rank_right;
                tmp = char::from_u32(item_right as u32).unwrap();
                break;
            }
            if check_right[rank_left] {
                dup = rank_left;
                tmp = char::from_u32(item_left as u32).unwrap();
                break;
            }
        }

        sum += dup;
        if comp_size != 0 {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
