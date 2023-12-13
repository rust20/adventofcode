use std::collections::HashSet;

pub fn part1(inp: &str) {
    let mut sum = 0;

    for line in inp.lines() {
        let mut hs = HashSet::new();

        let set: Vec<&str> = (&line[line.find(":").unwrap() + 1..])
            .split(" | ")
            .collect();
        set[0].split(" ").for_each(|val| {
            if let Ok(p) = val.parse::<i32>() {
                hs.insert(p);
            };
        });
        let win_num = set[1].split(" ").fold(0, |acc, val| {
            acc + if let Ok(p) = val.parse::<i32>() {
                if hs.get(&p).is_some() {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        });

        if win_num > 0 {
            let t = 1 << (win_num - 1);
            sum += t
        }
    }

    println!("part1: {}", sum)
}
pub fn part2(inp: &str) {

    let lines = inp.lines().collect::<Vec<&str>>();
    let mut wins = Vec::new();

    for line in lines {
        let mut hs = HashSet::new();

        let set: Vec<&str> = (&line[line.find(":").unwrap() + 1..])
            .split(" | ")
            .collect();
        set[0].split(" ").for_each(|val| {
            if let Ok(p) = val.parse::<i32>() {
                hs.insert(p);
            };
        });
        let win_num = set[1].split(" ").fold(0, |acc, val| {
            acc + if let Ok(p) = val.parse::<i32>() {
                if hs.get(&p).is_some() {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        });
    
        wins.push(win_num);
    }

    let n = wins.len();
    let mut card = vec![1;n];

    for i in 1..=n {
        let v = card[i-1];
        let w = wins[i-1];
        card[i..i+w].iter_mut().for_each(|val| {*val += v;});
    }
    let sum: u32 = card.iter().sum();

    println!("part2: {}", sum)
}

#[allow(dead_code)]
fn main() {
    let file_input = "tests/d4/input1.txt";
    let reader = std::fs::read_to_string(file_input).expect("read input");

    part1(&reader);
    part2(&reader);
}
