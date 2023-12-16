use std::{collections::HashMap, time::Instant};

pub fn part1(inp: &str) -> i64 {
    let mut map: Vec<Vec<char>> = inp.lines().map(|l| l.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 'O' {
                let mut cursor = i;
                while cursor > 0 && map[cursor - 1][j] == '.' {
                    map[cursor][j] = '.';
                    cursor -= 1;
                    map[cursor][j] = 'O';
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..height {
        let n = map[i].iter().filter(|&&v| v == 'O').count();
        sum += n * (height - i);
    }

    sum as i64
}

type Map = Vec<Vec<char>>;

pub fn part2(inp: &str) -> i64 {
    let mut map: Map = inp.lines().map(|l| l.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();

    let cycles = 1000000000;

    let mut hs: HashMap<Map, _> = HashMap::new();
    let mut hs_val: HashMap<i64, _> = HashMap::new();


    // TODO: use vec of positions of O as the hashmap key(?)

    let mut x = 0;
    for i in 0..cycles {
        if let Some(&n) = hs.get(&map) {
            x = n;
            break;
        }
        hs_val.insert(i, map.clone());
        hs.insert(map.clone(), i);

        // north
        for h in 0..height {
            for w in 0..width {
                if map[h][w] == 'O' {
                    let mut cursor = h;
                    while cursor > 0 && map[cursor - 1][w] == '.' {
                        map[cursor][w] = '.';
                        cursor -= 1;
                        map[cursor][w] = 'O';
                    }
                }
            }
        }

        // west
        for w in 0..width {
            for h in 0..height {
                if map[h][w] == 'O' {
                    let mut cursor = w;
                    while cursor > 0 && map[h][cursor - 1] == '.' {
                        map[h][cursor] = '.';
                        cursor -= 1;
                        map[h][cursor] = 'O';
                    }
                }
            }
        }
        // south
        for h in (0..height).rev() {
            for w in 0..width {
                if map[h][w] == 'O' {
                    let mut cursor = h;
                    while cursor < height - 1 && map[cursor + 1][w] == '.' {
                        map[cursor][w] = '.';
                        cursor += 1;
                        map[cursor][w] = 'O';
                    }
                }
            }
        }
        // east
        for w in (0..width).rev() {
            for h in 0..height {
                if map[h][w] == 'O' {
                    let mut cursor = w;
                    while cursor < width - 1 && map[h][cursor + 1] == '.' {
                        map[h][cursor] = '.';
                        cursor += 1;
                        map[h][cursor] = 'O';
                    }
                }
            }
        }

    }

    let variants = hs_val.len() as i64 - x;

    let c = (cycles - x) % variants as i64 + x;
    let map = hs_val.get(&c).unwrap().clone();

    let mut total_load = 0_i64;
    for h in 0..height {
        let n = map[h].iter().filter(|&&v| v == 'O').count();
        total_load += (n * (height - h)) as i64;
    }

    total_load as i64
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d14/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
