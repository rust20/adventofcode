use std::time::Instant;

fn correct(rows: &String, seq: &Vec<usize>) -> bool {
    let mut target = Vec::new();
    let mut current = 0;
    for i in rows.chars() {
        if i == '#' {
            current += 1;
        }
        if i == '.' && current != 0 {
            target.push(current);
            current = 0;
        }
    }

    if current != 0 {
        target.push(current);
    }

    seq.eq(&target)
}

fn generate(rows: &str) -> Vec<String> {
    let mut result = vec!["".to_string()];

    for i in rows.chars() {
        if i == '?' {
            let mut clones = result.clone();
            clones.iter_mut().for_each(|v| v.push('.'));
            result.iter_mut().for_each(|v| v.push('#'));

            result.append(&mut clones);
        } else {
            result.iter_mut().for_each(|v| v.push(i));
        }
    }

    result
}

pub fn part1(inp: &str) -> i64 {
    let mut sum = 0;
    for line in inp.lines() {
        let tmp: Vec<&str> = line.split(" ").collect();
        let springs = tmp[0];
        let brokens: Vec<usize> = tmp[1]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        // println!("{} {:?}", tmp[0], brokens);
        let possible_things = generate(springs);
        // println!("{}", possible_things.len());

        let mut n = 0;
        for t in possible_things {
            if correct(&t, &brokens) {
                println!("{} is correct", t);
                n += 1;
            }
        }

        println!("got {} line {}", n, line);

        sum += n
    }

    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let sum = inp.len();
    sum as i64
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        // "input1.txt",
        // "input2.txt",
        "input3.txt",
    ];

    for fi in inputs {
        let path = "tests/d12/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        let p1 = part1(&reader);
        let start2 = Instant::now();
        let p2 = part2(&reader);
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
        println!("part 1: {}", p1);
        println!("part 2: {}", p2);
    }
}
