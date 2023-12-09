use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("tests/d01/input1.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn solve1() {
    let data = get_input();
    let arr = data.split("\n");

    let mut max = 0;
    let mut curr = 0;

    for splitted in arr {
        if splitted.is_empty() {
            max = if max < curr { curr } else { max };

            curr = 0;
            continue;
        }
        curr += splitted.parse::<i32>().unwrap();
    }
    if curr > max {
        max = curr;
    }
    println!("{}", max);
}

fn solve2() {
    let data = get_input();
    let arr = data.split("\n").map(|x| x.parse::<i32>());

    let mut pos1 = 0;
    let mut pos2 = 0;
    let mut pos3 = 0;

    let mut curr = 0;

    let mut rerank = |prev| {
        if pos3 > prev {
            return;
        };
        pos3 = prev;
        if pos2 > pos3 {
            return;
        };
        (pos2, pos3) = (pos3, pos2);
        if pos1 < pos2 {
            (pos1, pos2) = (pos2, pos1);
        };
    };

    for splitted in arr {
        if let Ok(val) = splitted {
            curr += val;
            continue;
        }
        rerank(curr);
        curr = 0
    }
    let total = pos1 + pos2 + pos3;

    println!("{}", total);
}

fn main() {
    solve1();
    solve2();
}
