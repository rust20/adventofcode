use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    RI = 0,
    UP = 1,
    DO = 2,
    LE = 3,
}

const INFI: i64 = i64::MAX;

pub fn part1(inp: &str) -> i64 {
    let map: Vec<_> = inp.bytes().collect();
    let w = inp.chars().position(|v| v == '\n').unwrap() + 1;

    let mut visited = vec![false; map.len()];
    let mut dist = vec![INFI; map.len()];
    let mut path = vec![-1; map.len()];
    let mut hs = HashSet::new();

    let target = map.len() - 1;

    let mut curr: usize = 0;
    let mut dir = Dir::RI;
    let mut s_dist = 0;
    dist[curr] = 0;
    path[curr] = 0;

    loop {
        visited[curr] = true;
        let children = match dir {
            Dir::RI => [(curr + 1, Dir::RI), (curr - w, Dir::UP), (curr + w, Dir::DO)],
            Dir::UP => [(curr + 1, Dir::RI), (curr - w, Dir::UP), (curr - 1, Dir::LE)],
            Dir::DO => [(curr + 1, Dir::RI), (curr + w, Dir::DO), (curr - 1, Dir::LE)],
            Dir::LE => [(curr - w, Dir::UP), (curr + w, Dir::DO), (curr - 1, Dir::LE)],
        };
        for child in children {
            visited[child.0]
            
        }
    }

    let mut stack = VecDeque::new();
    stack.push_back((0, 0, 0, Dir::RI));

    let mut min = i32::MAX;

    let mut n: u64 = 0;
    while !stack.is_empty() {
        let (curr, c_loss, dist, dir): (usize, _, _, _) = stack.pop_back().unwrap();
        // println!("{:?}", (curr, c_loss, dist, &dir));

        if curr.leading_zeros() == 0
            || curr >= inp.len()
            || curr % w == w - 1
            || visited[curr].0[dir as usize] <= (map[curr] - b'0') as i32 + c_loss
            || dist == 3
        // || curr == target
        {
            continue;
        }

        n += 1;

        if 0 * (n % (1 << 30)) == 1 {
            println!("{}", n);
            for i in 0..map.len() {
                // if curr == i {
                //     print!("#");
                // } else
                if visited[i].1 != Dir::ZE {
                    print!(
                        "{}",
                        match visited[i].1 {
                            Dir::RI => ">",
                            Dir::UP => "^",
                            Dir::DO => "V",
                            Dir::LE => "<",
                            Dir::ZE => unreachable!(),
                        }
                    );
                } else {
                    print!("{}", map[i] as char);
                }
            }
            println!();
            std::thread::sleep(Duration::from_millis(1000));
        }

        let loss = (map[curr] - b'0') as i32 + c_loss;

        visited[curr].0[dir as usize] = loss;
        visited[curr].1 = dir.clone();

        if curr == target {
            // min = min.min((map[curr] - b'0') as i32 + c_loss)
            min = min.min(c_loss)
        }

        // stack.push_back((curr + m_dir.0, loss, 0, Dir::RI));
        // stack.push_back((curr + m_dir.1, loss, 0, Dir::UP));
        // stack.push_back((curr + m_dir.2, loss, 0, Dir::DO));
        // stack.push_back((curr + m_dir.3, loss, 0, Dir::LE));
        match dir {
            Dir::RI => {
                stack.push_back((curr + 1, loss, dist + 1, Dir::RI));
                stack.push_back((curr - w, loss, 0, Dir::UP));
                stack.push_back((curr + w, loss, 0, Dir::DO));
                // stack.push_back((curr - 1, loss, n_dist, Dir::LE));
            }
            Dir::UP => {
                stack.push_back((curr + 1, loss, 0, Dir::RI));
                stack.push_back((curr - w, loss, dist + 1, Dir::UP));
                // stack.push_back((curr + (width + 1), loss, n_dist, Dir::DO));
                stack.push_back((curr - 1, loss, 0, Dir::LE));
            }
            Dir::DO => {
                stack.push_back((curr + 1, loss, 0, Dir::RI));
                // stack.push_back((curr - (width + 1), loss, n_dist, Dir::UP));
                stack.push_back((curr + w, loss, dist + 1, Dir::DO));
                stack.push_back((curr - 1, loss, 0, Dir::LE));
            }
            Dir::LE => {
                // stack.push_back((curr + 1, loss, n_dist, Dir::RI));
                stack.push_back((curr - w, loss, 0, Dir::UP));
                stack.push_back((curr + w, loss, 0, Dir::DO));
                stack.push_back((curr - 1, loss, dist + 1, Dir::LE));
            }
            _ => unreachable!(),
        }
    }

    for i in 0..map.len() {
        // if curr == i {
        //     print!("#");
        // } else
        if visited[i].1 != Dir::ZE {
            print!(
                "{}",
                match visited[i].1 {
                    Dir::RI => ">",
                    Dir::UP => "^",
                    Dir::DO => "V",
                    Dir::LE => "<",
                    Dir::ZE => unreachable!(),
                }
            );
        } else {
            print!("{}", map[i] as char);
        }
    }
    println!();
    std::thread::sleep(Duration::from_millis(1));

    // println!("{:?}", inp.chars().collect::<Vec<char>>()[inp.len() - 2]);
    // visited[inp.len() - 2].0 as i64 + (map[inp.len() - 2] - b'0' - (map[0] - b'0')) as i64
    // visited[inp.len() - 2].0 as i64 - (map[inp.len() - 2] - b'0' + (map[0] - b'0')) as i64
    // *visited[map.len() - 2].0.iter().min().unwrap() as i64 - (map[0] - b'0') as i64 - (map[inp.len()-2] - b'0') as i64
    *visited[map.len() - 2].0.iter().min().unwrap() as i64 - (map[0] - b'0') as i64
    // visited[inp.len() - 2].0 as i64
    // min as i64 - (map[0] - b'0') as i64
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
