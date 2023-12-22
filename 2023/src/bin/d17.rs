use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Dir {
    RI = 0,
    UP,
    DO,
    LE,
}

pub fn part1(inp: &str) -> i64 {
    const MAX_STRAIGHT: usize = 3;
    const DIRS: usize = 4;

    let w = inp.chars().position(|v| v == '\n').unwrap() + 1;

    let target = inp.len() - 1;
    // let mut paths = vec![vec![]; DIRS * MAX_STRAIGHT * inp.len()];
    let mut min_cost = vec![i32::MAX; DIRS * MAX_STRAIGHT * inp.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, (0, Dir::RI))));
    queue.push(Reverse((0, 0, (0, Dir::DO))));

    while !queue.is_empty() {
        let (prev_loss, dist, (curr, dir)): (_, _, (usize, _)) =
            queue.pop().unwrap().0;

        if curr.leading_zeros() == 0
            || curr >= inp.len()
            || curr % w == w - 1
            || curr == target
            || MAX_STRAIGHT <= dist
        {
            continue;
        }

        let loc = curr * DIRS * MAX_STRAIGHT + dir as usize * MAX_STRAIGHT + dist;
        let loss = (inp.bytes().nth(curr).unwrap() - b'0') as i32 + prev_loss;

        if min_cost[loc] <= loss {
            continue;
        }
        min_cost[loc] = loss;

        // path.push((curr, dir));
        // paths[loc] = path.clone();

        match dir {
            Dir::RI => {
                queue.push(Reverse((loss, dist + 1, (curr + 1, Dir::RI))));
                queue.push(Reverse((loss, 0, (curr - w, Dir::UP))));
                queue.push(Reverse((loss, 0, (curr + w, Dir::DO))));
            }
            Dir::UP => {
                queue.push(Reverse((loss, 0, (curr + 1, Dir::RI))));
                queue.push(Reverse((loss, dist + 1, (curr - w, Dir::UP))));
                queue.push(Reverse((loss, 0, (curr - 1, Dir::LE))));
            }
            Dir::DO => {
                queue.push(Reverse((loss, 0, (curr + 1, Dir::RI))));
                queue.push(Reverse((loss, dist + 1, (curr + w, Dir::DO))));
                queue.push(Reverse((loss, 0, (curr - 1, Dir::LE))));
            }
            Dir::LE => {
                queue.push(Reverse((loss, 0, (curr - w, Dir::UP))));
                queue.push(Reverse((loss, 0, (curr + w, Dir::DO))));
                queue.push(Reverse((loss, dist + 1, (curr - 1, Dir::LE))));
            }
        }
    }

    let last = inp.len() - 2;
    let start = last * DIRS * MAX_STRAIGHT;
    let end = start + DIRS * MAX_STRAIGHT;

    let min = min_cost[start..end].iter().min().unwrap();
    // let pos = min_cost[start..end]
    //     .iter()
    //     .zip(paths[start..end].iter())
    //     .position(|(v1, v2)| v1 == min && !v2.is_empty())
    //     .unwrap();
    // let path_taken: HashMap<usize, Dir> = HashMap::from_iter(paths[start + pos].clone());
    //
    // for i in 0..inp.len() {
    //     print!(
    //         "{}",
    //         match path_taken.get(&i) {
    //             None => format!("{}", inp.chars().nth(i).unwrap()),
    //             Some(Dir::RI) => ">".to_string().red(),
    //             Some(Dir::UP) => "^".to_string().red(),
    //             Some(Dir::DO) => "V".to_string().red(),
    //             Some(Dir::LE) => "<".to_string().red(),
    //         }
    //     );
    // }
    // println!();

    *min as i64 - (inp.bytes().nth(0).unwrap() - b'0') as i64
}

pub fn part2(inp: &str) -> i64 {
    const MAX_STRAIGHT: usize = 10;
    const MIN_DIST: usize = 4;
    const DIRS: usize = 4;

    let map: Vec<_> = inp.bytes().collect();
    let w = inp.chars().position(|v| v == '\n').unwrap() + 1;

    let target = inp.len() - 1;
    let mut visited = vec![i32::MAX; DIRS * MAX_STRAIGHT * map.len()];
    // let mut paths = vec![vec![]; DIRS * MAX_STRAIGHT * map.len()];

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, (0, Dir::RI))));
    queue.push(Reverse((0, 0, (0, Dir::DO))));

    let is_valid = |pos: usize| {
        !(pos.leading_zeros() == 0 || pos >= inp.len() || pos % w == w - 1 || pos == target)
    };

    while !queue.is_empty() {
        let (c_loss, dist, (curr, dir)): (_, _, (usize, _)) = queue.pop().unwrap().0;

        if curr.leading_zeros() == 0
            || curr >= inp.len()
            || curr % w == w - 1
            || MAX_STRAIGHT <= dist
        {
            continue;
        }

        let loc = curr * DIRS * MAX_STRAIGHT + dir as usize * MAX_STRAIGHT + dist;
        let loss = (map[curr] - b'0') as i32 + c_loss;

        if visited[loc] <= loss {
            continue;
        }

        visited[loc] = loss;

        // path.push((curr, dir));
        // paths[loc] = path.clone();

        let dist_compare = MIN_DIST - 1;

        match dir {
            Dir::RI => {
                queue.push(Reverse((loss, dist + 1, (curr + 1, Dir::RI))));
                if dist >= dist_compare && is_valid(curr - w * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr - w, Dir::UP))));
                }
                if dist >= dist_compare && is_valid(curr + w * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr + w, Dir::DO))));
                }
            }
            Dir::UP => {
                queue.push(Reverse((loss, dist + 1, (curr - w, Dir::UP))));
                if dist >= dist_compare && is_valid(curr + 1 * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr + 1, Dir::RI))));
                }
                if dist >= dist_compare && is_valid(curr - 1 * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr - 1, Dir::LE))));
                }
            }
            Dir::DO => {
                queue.push(Reverse((loss, dist + 1, (curr + w, Dir::DO))));
                if dist >= dist_compare && is_valid(curr + 1 * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr + 1, Dir::RI))));
                }
                if dist >= dist_compare && is_valid(curr - 1 * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr - 1, Dir::LE))));
                }
            }
            Dir::LE => {
                queue.push(Reverse((loss, dist + 1, (curr - 1, Dir::LE))));
                if dist >= dist_compare && is_valid(curr - w * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr - w, Dir::UP))));
                }
                if dist >= dist_compare && is_valid(curr + w * MIN_DIST) {
                    queue.push(Reverse((loss, 0, (curr + w, Dir::DO))));
                }
            }
        }
    }

    let last = map.len() - 2;
    let start = last * DIRS * MAX_STRAIGHT;
    let end = (last + 1) * DIRS * MAX_STRAIGHT;

    let min = visited[start..end].iter().min().unwrap();
    // let pos = visited[start..end]
    //     .iter()
    //     .zip(paths[start..end].iter())
    //     .position(|(v1, v2)| v1 == min && !v2.is_empty())
    //     .unwrap();
    // let path_taken: HashMap<usize, Dir> = HashMap::from_iter(paths[start + pos].clone());
    //
    // for i in 0..map.len() {
    //     print!(
    //         "{}",
    //         match path_taken.get(&i) {
    //             None => format!("{}", map[i] as char),
    //             Some(Dir::RI) => ">".to_string().red(),
    //             Some(Dir::UP) => "^".to_string().red(),
    //             Some(Dir::DO) => "V".to_string().red(),
    //             Some(Dir::LE) => "<".to_string().red(),
    //         }
    //     );
    // }
    // println!();

    *min as i64 - (map[0] - b'0') as i64
}

trait Something {
    fn red(self) -> Self;
}
impl Something for String {
    fn red(self) -> Self {
        "\x1b[93m".to_string() + &self + "\x1b[0m"
    }
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input1_1.txt",
        "input2.txt",
        "input3.txt",
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
