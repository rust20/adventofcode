use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

#[derive(Clone, Eq, PartialEq)]
enum Dir {
    ZE = 0,
    RI = 0b0001,
    UP = 0b0010,
    DO = 0b0100,
    LE = 0b1000,
}

pub fn part1(inp: &str) -> i64 {
    let map: Vec<_> = inp.bytes().collect();
    let sum = inp.len();
    let width = inp.chars().position(|v| v == '\n').unwrap();

    let mut visited = vec![(i32::MAX, Dir::ZE); inp.len()];
    let mut stack = VecDeque::new();
    stack.push_back((0, 0, 0, Dir::RI));

    while !stack.is_empty() {
        let (curr, c_loss, dist, dir): (usize, _, _, _) = stack.pop_front().unwrap();

        if curr.leading_zeros() == 0
            || curr >= inp.len()
            || curr % (width + 1) == width
            || visited[curr].0 < (map[curr] - b'0') as i32 + c_loss
        {
            continue;
        }

        let loss = (map[curr] - b'0') as i32 + c_loss;

        for i in 0..map.len() {
            if curr == i {
                print!("#");
            } else if visited[i].1 != Dir::ZE {
                print!("{}", match visited[i].1 {
                    Dir::RI => ">",
                    Dir::UP => "^",
                    Dir::DO => "V",
                    Dir::LE => "<",
                    Dir::ZE => unreachable!(),
                });
            } else {
                print!("{}", map[i] as char);
            }
        }
        println!();
        // std::thread::sleep(Duration::from_millis(1));

        visited[curr] = (loss, dir.clone());
        let n_dist = if dist == 2 { 0 } else { dist + 1 };

        // stack.push_back((curr + 1, loss, n_dist, Dir::RI));
        // stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
        // stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
        // stack.push_back((curr - 1, loss, n_dist, Dir::LE));
        match (dist == 2, dir) {
            (true, Dir::RI) => {
                // stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                // stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (false, Dir::RI) => {
                stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                // stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (true, Dir::UP) => {
                stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                // stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                // stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (false, Dir::UP) => {
                stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                // stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (true, Dir::DO) => {
                stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                // stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                // stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (false, Dir::DO) => {
                stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                // stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (true, Dir::LE) => {
                // stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            (false, Dir::LE) => {
                // stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                // stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            _ => unreachable!(),
        }
    }
    println!("{:?}", inp.chars().collect::<Vec<char>>()[inp.len() - 2]);
    visited[inp.len() - 2].0 as i64
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
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d17/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
