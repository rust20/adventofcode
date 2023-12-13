use std::{collections::HashMap, time::Instant};

#[allow(dead_code)]
fn consume_debug(pat: &str, broks: &[usize], curr: String) -> i64 {
    if broks.is_empty() {
        if pat.is_empty() {
            println!("match: {}", curr);
            return 1;
        } else {
            return pat.chars().all(|v| v != '#') as i64;
        }
    }

    let b_seq = broks[0];
    if pat.len() < b_seq {
        return 0;
    }

    let res = if pat[0..b_seq].chars().all(|v| v != '.') {
        let mut s = 0;
        if &pat[0..1] == "?" {
            s = consume_debug(&pat[1..], &broks, curr.to_string() + ".")
        }

        let n = &String::from_utf8(vec![b'#'; b_seq]).unwrap();

        if pat.len() == b_seq {
            s + consume_debug(&pat[b_seq..], &broks[1..], curr.to_string() + n)
        } else if !(&pat[b_seq..=b_seq] == "#") {
            s + consume_debug(&pat[b_seq + 1..], &broks[1..], curr.to_string() + n + ".")
        } else {
            s
        }
    } else {
        consume_debug(&pat[1..], &broks, curr.to_string() + ".")
    };

    res
}

fn consume(memo: &mut HashMap<(usize, usize), i64>, pat: &str, broks: &[usize]) -> i64 {
    let key = &(pat.len(), broks.len());
    if memo.contains_key(key) {
        return memo[key];
    }

    if broks.is_empty() {
        if pat.is_empty() {
            return 1;
        } else {
            return pat.chars().all(|v| v != '#') as i64;
        }
    }

    let b_seq = broks[0];
    if pat.len() < b_seq {
        return 0;
    }

    let res = if pat[0..b_seq].chars().all(|v| v != '.') {
        let mut s = 0;
        if &pat[0..1] == "?" {
            s = consume(memo, &pat[1..], &broks)
        }

        if pat.len() == b_seq {
            s + consume(memo, &pat[b_seq..], &broks[1..])
        } else if !(&pat[b_seq..=b_seq] == "#") {
            s + consume(memo, &pat[b_seq + 1..], &broks[1..])
        } else {
            s
        }
    } else if &pat[0..1] == "#" {
        0
    } else {
        consume(memo, &pat[1..], &broks)
    };

    memo.insert(*key, res);
    res
}

pub fn part1(inp: &str) -> i64 {
    let mut sum = 0;
    for line in inp.lines() {
        let tmp: Vec<&str> = line.split(" ").collect();
        let springs = tmp[0];
        let brokens: Vec<usize> = tmp[1].split(',').map(|v| v.parse().unwrap()).collect();

        let mut hs = HashMap::new();
        let n = consume(&mut hs, springs, &brokens[..]);
        // let n = consume_debug(springs, &brokens[..], "".to_string());
        // println!("got {}  {}", n, line);

        sum += n
    }

    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let mut sum = 0;
    for line in inp.lines() {
        let tmp: Vec<&str> = line.split(" ").collect();
        let mut springs = tmp[0].to_string();
        let mut brokens: Vec<usize> = tmp[1].split(',').map(|v| v.parse().unwrap()).collect();

        springs = vec![springs; 5].join("?");

        let dup = brokens.clone();
        for _ in 0..4 {
            brokens.append(&mut dup.clone())
        }

        // println!("{} {:?}", springs, brokens);

        let mut hs = HashMap::new();
        let n = consume(&mut hs, &springs, &brokens[..]);
        // println!("got {}", n);
        sum += n
    }

    sum as i64
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        // "input2.txt",
        // "input3.txt",
    ];

    for fi in inputs {
        let path = "tests/d12/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
