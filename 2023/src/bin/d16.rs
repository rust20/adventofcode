use std::time::Instant;

// const UP: isize = 0b0010; // up
// const RI: isize = 0b0001; // right
// const DO: isize = 0b0100; // down
// const LE: isize = 0b1001; // left

#[derive(Clone, Copy)]
enum Dir {
    UP = 0b0010,
    RI = 0b0001,
    DO = 0b0100,
    LE = 0b1000,
}

pub fn part1(inp: &str) -> i64 {
    let inp = inp.bytes().collect::<Vec<_>>();
    let width = inp.iter().position(|&v| v == b'\n').unwrap() as isize + 1;
    let height = inp.iter().filter(|&&v| v == b'\n').count() as isize;

    let mut stack = Vec::new();
    let mut visited = vec![0 as isize; inp.len()];

    stack.push((0 as isize, Dir::RI));

    while !stack.is_empty() {
        let (curr, dir) = stack.pop().unwrap();
        if curr < 0
            || curr % width == width - 1
            || curr / width == height
            || curr >= inp.len() as isize
            || visited[curr as usize] & dir as isize > 0
        {
            continue;
        }
        visited[curr as usize] |= dir as isize;

        match (inp[curr as usize], dir) {
            (b'-', Dir::RI) | (b'.', Dir::RI) | (b'/', Dir::UP) | (b'\\', Dir::DO) => {
                stack.push((curr + 1, Dir::RI));
            }
            (b'|', Dir::DO) | (b'.', Dir::DO) | (b'/', Dir::LE) | (b'\\', Dir::RI) => {
                stack.push((curr + width, Dir::DO));
            }
            (b'-', Dir::LE) | (b'.', Dir::LE) | (b'/', Dir::DO) | (b'\\', Dir::UP) => {
                stack.push((curr - 1, Dir::LE));
            }
            (b'|', Dir::UP) | (b'.', Dir::UP) | (b'/', Dir::RI) | (b'\\', Dir::LE) => {
                stack.push((curr - width, Dir::UP));
            }
            (b'|', Dir::LE | Dir::RI) => {
                stack.push((curr - width, Dir::UP));
                stack.push((curr + width, Dir::DO));
            }
            (b'-', Dir::UP | Dir::DO) => {
                stack.push((curr - 1, Dir::LE));
                stack.push((curr + 1, Dir::RI));
            }
            _ => {}
        }
    }

    let sum = visited.iter().filter(|&&v| v > 0).count();
    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let inp = inp.bytes().collect::<Vec<_>>();
    let wid = inp.iter().position(|&v| v == b'\n').unwrap() as isize + 1;
    let hei = inp.iter().filter(|&&v| v == b'\n').count() as isize;

    let mut max = 0;

    let v = |start: isize, dir: Dir| {
        let mut stack = Vec::new();
        let mut visited = vec![0 as isize; inp.len()];

        stack.push((start, dir));

        while !stack.is_empty() {
            let (curr, dir) = stack.pop().unwrap();
            if curr < 0
                || curr % wid == wid - 1
                || curr / wid == hei
                || curr >= inp.len() as isize
                || visited[curr as usize] & dir as isize > 0
            {
                continue;
            }
            visited[curr as usize] |= dir as isize;

            match (inp[curr as usize], dir) {
                (b'-', Dir::RI) | (b'.', Dir::RI) | (b'/', Dir::UP) | (b'\\', Dir::DO) => {
                    stack.push((curr + 1, Dir::RI))
                }
                (b'|', Dir::DO) | (b'.', Dir::DO) | (b'/', Dir::LE) | (b'\\', Dir::RI) => {
                    stack.push((curr + wid, Dir::DO))
                }
                (b'-', Dir::LE) | (b'.', Dir::LE) | (b'/', Dir::DO) | (b'\\', Dir::UP) => {
                    stack.push((curr - 1, Dir::LE))
                }
                (b'|', Dir::UP) | (b'.', Dir::UP) | (b'/', Dir::RI) | (b'\\', Dir::LE) => {
                    stack.push((curr - wid, Dir::UP))
                }
                (b'|', Dir::LE | Dir::RI) => {
                    stack.push((curr - wid, Dir::UP));
                    stack.push((curr + wid, Dir::DO));
                }
                (b'-', Dir::UP | Dir::DO) => {
                    stack.push((curr - 1, Dir::LE));
                    stack.push((curr + 1, Dir::RI));
                }
                _ => {}
            }
        }

        let sum = visited.iter().filter(|&&v| v > 0).count();
        sum as i64
    };
    for i in 0..wid - 1 {
        let padding = wid * (hei - 1);
        max = max.max(v(i, Dir::DO));
        max = max.max(v(padding + i, Dir::UP));
    }
    for i in 0..hei {
        max = max.max(v(i * wid, Dir::RI));
        max = max.max(v((i + 1) * wid - 1, Dir::LE));
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

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
