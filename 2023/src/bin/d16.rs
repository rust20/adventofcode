use std::{collections::HashMap, time::Instant};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    UP = 0b0010,
    RI = 0b0001,
    DO = 0b0100,
    LE = 0b1000,
}

pub fn part1(inp: &str) -> i64 {
    let inp = inp.bytes().collect::<Vec<_>>();
    let width = inp.iter().position(|&v| v == b'\n').unwrap();

    let mut stack = Vec::new();
    let mut visited = vec![0; inp.len()];

    stack.push((0, Dir::RI));

    let mut sum = 0;
    while !stack.is_empty() {
        let (curr, dir): (usize, _) = stack.pop().unwrap();
        if curr.leading_zeros() == 0
            || curr >= inp.len()
            || curr % (width + 1) == width
            || visited[curr] & dir as usize > 0
        {
            continue;
        }
        sum += (visited[curr] == 0) as i32;
        visited[curr] |= dir as usize;

        match (inp[curr], dir) {
            (b'-', Dir::RI) | (b'.', Dir::RI) | (b'/', Dir::UP) | (b'\\', Dir::DO) => {
                stack.push((curr + 1, Dir::RI));
            }
            (b'|', Dir::DO) | (b'.', Dir::DO) | (b'/', Dir::LE) | (b'\\', Dir::RI) => {
                stack.push((curr + (width + 1), Dir::DO));
            }
            (b'-', Dir::LE) | (b'.', Dir::LE) | (b'/', Dir::DO) | (b'\\', Dir::UP) => {
                stack.push((curr - 1, Dir::LE));
            }
            (b'|', Dir::UP) | (b'.', Dir::UP) | (b'/', Dir::RI) | (b'\\', Dir::LE) => {
                stack.push((curr - (width + 1), Dir::UP));
            }
            (b'|', Dir::LE | Dir::RI) => {
                stack.push((curr - (width + 1), Dir::UP));
                stack.push((curr + (width + 1), Dir::DO));
            }
            (b'-', Dir::UP | Dir::DO) => {
                stack.push((curr - 1, Dir::LE));
                stack.push((curr + 1, Dir::RI));
            }
            _ => {}
        }
    }

    // let sum = visited.iter().filter(|&&v| v > 0).count();
    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let inp = inp.bytes().collect::<Vec<_>>();
    let width = inp.iter().position(|&v| v == b'\n').unwrap();
    let height = inp.iter().filter(|&&v| v == b'\n').count();

    let mut max = 0;

    let v = |start: usize, dir: Dir| {
        let mut stack = Vec::new();
        let mut visited = vec![0; inp.len()];

        stack.push((start, dir));

        let mut sum = 0;
        while !stack.is_empty() {
            let (curr, dir) = stack.pop().unwrap();

            if curr.leading_zeros() == 0
                || curr % (width + 1) == width
                || curr >= inp.len()
                || visited[curr] & dir as usize > 0
            {
                continue;
            }

            sum += (visited[curr] == 0) as i32;
            visited[curr] |= dir as usize;

            match (inp[curr], dir) {
                (b'-', Dir::RI) | (b'.', Dir::RI) | (b'/', Dir::UP) | (b'\\', Dir::DO) => {
                    stack.push((curr + 1, Dir::RI))
                }
                (b'|', Dir::DO) | (b'.', Dir::DO) | (b'/', Dir::LE) | (b'\\', Dir::RI) => {
                    stack.push((curr + (width + 1), Dir::DO))
                }
                (b'-', Dir::LE) | (b'.', Dir::LE) | (b'/', Dir::DO) | (b'\\', Dir::UP) => {
                    stack.push((curr - 1, Dir::LE))
                }
                (b'|', Dir::UP) | (b'.', Dir::UP) | (b'/', Dir::RI) | (b'\\', Dir::LE) => {
                    stack.push((curr - (width + 1), Dir::UP))
                }
                (b'|', Dir::LE | Dir::RI) => {
                    stack.push((curr - (width + 1), Dir::UP));
                    stack.push((curr + (width + 1), Dir::DO));
                }
                (b'-', Dir::UP | Dir::DO) => {
                    stack.push((curr - 1, Dir::LE));
                    stack.push((curr + 1, Dir::RI));
                }
                _ => {}
            }
        }

        // let sum = visited.iter().filter(|&&v| v > 0).count();
        sum as i64
    };

    let padding = (width + 1) * (height - 1);
    for i in 0..width {
        max = max.max(v(i, Dir::DO));
        max = max.max(v(padding + i, Dir::UP));
    }
    for i in 0..height {
        max = max.max(v(i * (width + 1), Dir::RI));
        max = max.max(v((i + 1) * (width + 1) - 1, Dir::LE));
    }

    max as i64
}

fn traverse(
    memo: &mut Vec<i32>,
    map: &mut (&mut Vec<u8>, &Vec<u8>, usize),
    next: (usize, Dir),
) -> i64 {
    let (curr, dir) = next;
    let (visited, inp, width) = map;
    let width = *width;

    if curr.leading_zeros() == 0
        || curr >= inp.len()
        || curr % (width + 1) == width
        || visited[curr] & dir as u8 > 0
    {
        return 0;
    }
    // let tz = curr + (dir as u8).trailing_zeros() as usize;
    // let memo_val = memo[tz];
    // if memo_val > 0 {
    //     return memo_val as i64;
    // }
    let energieze = (visited[curr] == 0) as i64;
    visited[curr] |= dir as u8;

    let res = energieze
        + match (inp[curr], dir) {
            (b'-', Dir::RI) | (b'.', Dir::RI) | (b'/', Dir::UP) | (b'\\', Dir::DO) => {
                traverse(memo, map, (curr + 1, Dir::RI))
            }
            (b'|', Dir::DO) | (b'.', Dir::DO) | (b'/', Dir::LE) | (b'\\', Dir::RI) => {
                traverse(memo, map, (curr + (width + 1), Dir::DO))
            }
            (b'-', Dir::LE) | (b'.', Dir::LE) | (b'/', Dir::DO) | (b'\\', Dir::UP) => {
                traverse(memo, map, (curr - 1, Dir::LE))
            }
            (b'|', Dir::UP) | (b'.', Dir::UP) | (b'/', Dir::RI) | (b'\\', Dir::LE) => {
                traverse(memo, map, (curr - (width + 1), Dir::UP))
            }
            (b'|', Dir::LE | Dir::RI) => {
                traverse(memo, map, (curr - (width + 1), Dir::UP))
                    + traverse(memo, map, (curr + (width + 1), Dir::DO))
            }
            (b'-', Dir::UP | Dir::DO) => {
                traverse(memo, map, (curr - 1, Dir::LE)) + traverse(memo, map, (curr + 1, Dir::RI))
            }
            _ => unreachable!(),
        };

    // memo[tz] = res as i32;

    res
}

fn part1_2(inp: &str) -> i64 {
    let inp = inp.bytes().collect::<Vec<_>>();
    let width = inp.iter().position(|&v| v == b'\n').unwrap();

    let mut memo = vec![0; inp.len()];
    let sum = traverse(
        &mut memo,
        &mut (&mut vec![0; inp.len()], &inp, width),
        (0, Dir::RI),
    );

    sum as i64
}

fn part2_2(inp: &str) -> i64 {
    let inp = inp.bytes().collect::<Vec<_>>();
    let width = inp.iter().position(|&v| v == b'\n').unwrap();
    let height = inp.iter().filter(|&&v| v == b'\n').count();

    let mut max = 0;

    let padding = (width + 1) * (height - 1);
    for i in 0..width {
        let mut memo = vec![0; inp.len()];
        max = max.max(traverse(
            &mut memo,
            &mut (&mut vec![0; inp.len()], &inp, width),
            (i, Dir::DO),
        ));

        let mut memo = vec![0; inp.len()];
        max = max.max(traverse(
            &mut memo,
            &mut (&mut vec![0; inp.len()], &inp, width),
            (padding + i, Dir::UP),
        ));
    }
    for i in 0..height {
        let mut memo = vec![0; inp.len()];
        max = max.max(traverse(
            &mut memo,
            &mut (&mut vec![0; inp.len()], &inp, width),
            (i * (width + 1), Dir::RI),
        ));

        let mut memo = vec![0; inp.len()];
        max = max.max(traverse(
            &mut memo,
            &mut (&mut vec![0; inp.len()], &inp, width),
            ((i + 1) * (width + 1) - 1, Dir::LE),
        ));
    }

    max as i64
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d16/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        let start1_2 = Instant::now();
        println!("part1 {}", part1_2(&reader));
        let start2_2 = Instant::now();
        println!("part2 {}", part2_2(&reader));
        let finish_2 = start2_2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
        println!("v2: time1 {:?}, time2 {:?}", start2_2 - start1_2, finish_2);
    }
}
