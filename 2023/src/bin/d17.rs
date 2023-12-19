use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Dir {
    RI = 0,
    UP = 1,
    DO = 2,
    LE = 3,
}

pub fn part1(inp: &str) -> i64 {
    let map: Vec<_> = inp.bytes().collect();
    let w = inp.chars().position(|v| v == '\n').unwrap() + 1;

    let mut visited = vec![i32::MAX; 16 * map.len()];
    let mut stack = BinaryHeap::new();
    stack.push(Reverse((0, 0, 0, Dir::RI)));

    while !stack.is_empty() {
        let (c_loss, curr, dist, dir): (_, usize, _, _) = stack.pop().unwrap().0;

        if curr.leading_zeros() == 0
            || curr >= inp.len()
            || curr % w == w - 1
            || dist == 3
            || curr == map.len() - 1
        {
            continue;
        }

        let loc = curr * 16 + dir as usize * 4 + dist;
        let loss = (map[curr] - b'0') as i32 + c_loss;


        if visited[loc] <= (map[curr] - b'0') as i32 + c_loss {
            continue;
        }

        visited[loc] = loss;

        match dir {
            Dir::RI => {
                stack.push(Reverse((loss, curr + 1, dist + 1, Dir::RI)));
                stack.push(Reverse((loss, curr - w, 0, Dir::UP)));
                stack.push(Reverse((loss, curr + w, 0, Dir::DO)));
            }
            Dir::UP => {
                stack.push(Reverse((loss, curr + 1, 0, Dir::RI)));
                stack.push(Reverse((loss, curr - w, dist + 1, Dir::UP)));
                stack.push(Reverse((loss, curr - 1, 0, Dir::LE)));
            }
            Dir::DO => {
                stack.push(Reverse((loss, curr + 1, 0, Dir::RI)));
                stack.push(Reverse((loss, curr + w, dist + 1, Dir::DO)));
                stack.push(Reverse((loss, curr - 1, 0, Dir::LE)));
            }
            Dir::LE => {
                stack.push(Reverse((loss, curr - w, 0, Dir::UP)));
                stack.push(Reverse((loss, curr + w, 0, Dir::DO)));
                stack.push(Reverse((loss, curr - 1, dist + 1, Dir::LE)));
            }
            _ => unreachable!(),
        }
    }

    // let allpath = visited[map.len() - 2].2.clone();

    // for pt in 0..allpath.len() {
    //     println!("res: {}", visited[map.len()-2].0[pt]);
    //     let path_taken: HashMap<usize, Dir> = HashMap::from_iter(allpath[pt].clone());
    //     for i in 0..map.len() {
    //         print!(
    //             "{}",
    //             match path_taken.get(&i) {
    //                 None => format!("{}", map[i] as char),
    //                 Some(Dir::RI) => ">".to_string(),
    //                 Some(Dir::UP) => "^".to_string(),
    //                 Some(Dir::DO) => "V".to_string(),
    //                 Some(Dir::LE) => "<".to_string(),
    //                 _ => "!".to_string(),
    //             }
    //         );
    //     }
    //     println!();
    // }

    // let path_taken: HashMap<usize, Dir> = HashMap::from_iter(visited[map.len() - 2].2.clone());
    // for i in 0..map.len() {
    //     print!(
    //         "{}",
    //         match path_taken.get(&i) {
    //             None => &String::from_utf8([map[i]].to_vec()).unwrap(),
    //             Some(Dir::RI) => ">",
    //             Some(Dir::UP) => "^",
    //             Some(Dir::DO) => "V",
    //             Some(Dir::LE) => "<",
    //             _ => "!",
    //         }
    //     );
    // }
    // println!();
    // std::thread::sleep(Duration::from_millis(1));

    let last = map.len() - 2;
    *visited[last * 16..(last + 1) * 16].iter().min().unwrap() as i64 - (map[0] - b'0') as i64
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
        "input1.txt",
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
