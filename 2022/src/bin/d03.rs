use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("tests/d03/input1.txt").expect("Unable to open file");
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
                break;
            }
            if check_right[rank_left] {
                dup = rank_left;
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

    let all_group: Vec<&str> = input.collect();
    let mut group: usize = 0;

    let group_n: usize = 3;

    while group < all_group.len() - 1 {
        let mut check = vec![vec![false; group_n]; 52];

        for ch_index in 0..group_n {
            for x in all_group[group + ch_index].chars().into_iter() {
                check[to_rank(x as u32)][ch_index] = true;
            }
        }

        for i in 0..52 {
            if check[i].iter().fold(true, |acc, item| acc && *item) {
                sum += i + 1;
                break;
            }
        }

        group += group_n;
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
