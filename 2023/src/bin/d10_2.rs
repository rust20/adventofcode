#![feature(test)]

use std::time::Instant;

extern crate test;

#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn part1(inp: &str) {
    let s1 = Instant::now();

    let mut map: Vec<char> = inp.chars().collect();
    let map_w = map.iter().position(|v| *v == '\n').unwrap();
    let pos = map.iter().position(|v| *v == 'S').unwrap();

    let s2 = Instant::now();
    let mut memo = vec![-1; inp.len()];
    let mut stack = Vec::new();

    let s3 = Instant::now();
    let s_pipe = if matches!(map[pos + 1], '-' | 'J' | '7') {
        ('-', Dir::R)
    } else if matches!(map[pos + map_w + 1], '|' | 'J' | 'L') {
        ('|', Dir::R)
    } else {
        ('L', Dir::R)
    };

    memo[pos] = 0;
    map[pos] = s_pipe.0;
    stack.push((pos, s_pipe.1));
    // stack.push((pos, s_pipe.2));

    let visit = |stack: &mut Vec<_>, memo: &mut Vec<_>, p: usize, val, dir| {
        println!("gonna visit {:?} {:?} {}", p, dir, val);

        let n: isize = match dir {
            Dir::D => map_w as isize + 1,
            Dir::U => map_w as isize - 1,
            Dir::L => -1,
            Dir::R => 1,
        };

        if (p % map_w).checked_add_signed(n).is_none() {
            return;
        }

        if let Some(n) = p.checked_add_signed(n) {
            if memo[n] > val + 1 || memo[n] == -1 {
                memo[n] = val + 1;
                stack.push((n, dir));
            }
        }
    };

    let s4 = Instant::now();
    let mut t = 0;
    while !stack.is_empty() {
        let (p, dir) = stack.pop().unwrap();
        let val = memo[p];

        println!("visiting {:?} {:?} {} {}", p, dir, val, map[p]);

        match (map[p], dir) {
            ('-', Dir::R) | ('L', Dir::D) | ('F', Dir::U) => {
                visit(&mut stack, &mut memo, p, val, Dir::R)
            }
            ('-', Dir::L) | ('J', Dir::D) | ('7', Dir::U) => {
                visit(&mut stack, &mut memo, p, val, Dir::L)
            }
            ('|', Dir::D) | ('7', Dir::R) | ('F', Dir::L) => {
                visit(&mut stack, &mut memo, p, val, Dir::D)
            }
            ('|', Dir::U) | ('J', Dir::R) | ('L', Dir::L) => {
                visit(&mut stack, &mut memo, p, val, Dir::U)
            }
            _ => {}
        }
        t += 1;
    }
    println!("{}", t / 2);

    for i in (0..map.len()).step_by(map_w+1) {
        println!("{:?}", &map[i..=i+map_w]);
    }
    println!();

    let s5 = Instant::now();
    let sum = memo
        .iter()
        .max()
        .unwrap();

    let s6 = Instant::now();

    println!(
        "p1: {:?}, p2: {:?}, p3: {:?}, p4: {:?}, p5: {:?}",
        s2 - s1,
        s3 - s2,
        s4 - s3,
        s5 - s4,
        s6 - s5,
    );
    println!("part 1: {}", sum);
}

fn part2(inp: &str) {
    let sum = inp.len();
    println!("part 2: {}", sum);
}

fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        // "input1.txt",
        "input2.txt",
        "input3.txt",
        "input4.txt",
        "input5.txt",
        "input6.txt",
        "input7.txt",
    ];

    for fi in inputs {
        let path = "tests/d10/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        part1(&reader);
        let start2 = Instant::now();
        part2(&reader);
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}

#[bench]
fn pt2(b: &mut test::Bencher) {
    let fi = "tests/d10/input1.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");

    b.iter(|| part1(&reader))
}
