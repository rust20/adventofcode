#![feature(test)]

use std::time::Instant;

extern crate test;

fn part1(inp: &str) {
    let mut map: Vec<Vec<char>> = inp.lines().map(|v| v.chars().collect()).collect();
    let map_h = map.len();
    let map_w = map[0].len();
    let mut pos = (0, 0);

    for y in 0..map_h {
        for x in 0..map_w {
            if map[y][x] == 'S' {
                pos = (x as i32, y as i32);
            }
        }
    }

    let (x, y) = pos;
    let x = x as usize;
    let y = y as usize;

    let s_pipe = match (
        if x > 0 { map[y][x - 1] } else { '.' },
        map[y + 1][x],
        if y > 0 { map[y - 1][x] } else { '.' },
        map[y][x + 1],
    ) {
        (_, '|' | 'J' | 'L', '|' | 'F' | '7', _) => '|',
        ('-' | 'F' | 'L', _, _, '-' | 'J' | '7') => '-',
        (_, '|' | 'J' | 'L', _, '-' | 'J' | '7') => 'F',
        ('-' | 'F' | 'L', '|' | 'J' | 'L', _, _) => '7',
        ('-' | 'F' | 'L', _, '|' | 'F' | '7', _) => 'J',
        (_, _, '|' | 'F' | '7', '-' | 'J' | '7') => 'L',
        _ => 's',
    };
    map[y][x] = s_pipe;

    let mut memo = vec![vec![-1 as i32; map_w]; map_h];
    let mut stack = Vec::new();
    stack.push(pos);
    memo[pos.1 as usize][pos.0 as usize] = 0;

    let visit = |stack: &mut Vec<_>, memo: &mut Vec<Vec<_>>, x, y, val| {
        if 0 > x || x >= map_w as i32 || 0 > y || y >= map_h as i32 {
            return;
        }
        if memo[y as usize][x as usize] > val + 1 || memo[y as usize][x as usize] == -1 {
            memo[y as usize][x as usize] = val + 1;
            stack.push((x, y));
        }
    };

    while !stack.is_empty() {
        let (x, y): (i32, i32) = stack.pop().unwrap();
        let val = memo[y as usize][x as usize];

        match map[y as usize][x as usize] {
            '.' => {
                memo[y as usize][x as usize] = -1;
            }
            'S' => {
                memo[y as usize][x as usize] = 0;
                let val = 0;

                if y > 0 && matches!(map[y as usize - 1][x as usize], '|' | '7' | 'F') {
                    visit(&mut stack, &mut memo, x, y - 1, val);
                }
                if matches!(map[y as usize + 1][x as usize], '|' | 'J' | 'L') {
                    visit(&mut stack, &mut memo, x, y + 1, val);
                }
                if x > 0 && matches!(map[y as usize][x as usize - 1], '-' | 'F' | 'L') {
                    visit(&mut stack, &mut memo, x + 1, y, val);
                }
                if matches!(map[y as usize][x as usize + 1], '-' | '7' | 'J') {
                    visit(&mut stack, &mut memo, x + 1, y, val);
                }
            }
            'L' => {
                visit(&mut stack, &mut memo, x, y - 1, val);
                visit(&mut stack, &mut memo, x + 1, y, val);
            }
            'J' => {
                visit(&mut stack, &mut memo, x, y - 1, val);
                visit(&mut stack, &mut memo, x - 1, y, val);
            }
            '7' => {
                visit(&mut stack, &mut memo, x, y + 1, val);
                visit(&mut stack, &mut memo, x - 1, y, val);
            }
            'F' => {
                visit(&mut stack, &mut memo, x, y + 1, val);
                visit(&mut stack, &mut memo, x + 1, y, val);
            }
            '-' => {
                visit(&mut stack, &mut memo, x - 1, y, val);
                visit(&mut stack, &mut memo, x + 1, y, val);
            }
            '|' => {
                visit(&mut stack, &mut memo, x, y - 1, val);
                visit(&mut stack, &mut memo, x, y + 1, val);
            }
            _ => {}
        }
    }

    let sum = memo
        .iter()
        .map(|val| val.iter().max().unwrap())
        .max()
        .unwrap();
    println!("part 1: {}", sum);
}

fn part2(inp: &str) {
    let mut map: Vec<Vec<char>> = inp.lines().map(|v| v.chars().collect()).collect();
    let map_h = map.len();
    let map_w = map[0].len();
    let mut pos = (0, 0);

    for y in 0..map_h {
        for x in 0..map_w {
            if map[y][x] == 'S' {
                pos = (x as i32, y as i32);
            }
        }
    }

    let mut memo = vec![vec![0 as i32; map_w]; map_h];
    let mut stack = Vec::new();
    stack.push(pos);
    memo[pos.1 as usize][pos.0 as usize] = 0;

    let visit = |stack: &mut Vec<_>, memo: &mut Vec<Vec<_>>, x, y, _val| {
        if 0 > x || x >= map_w as i32 || 0 > y || y >= map_h as i32 {
            return;
        }
        if memo[y as usize][x as usize] == 0 {
            stack.push((x, y));
        }
    };

    let (x, y) = pos;
    let x = x as usize;
    let y = y as usize;

    let s_pipe = match (
        if x > 0 { map[y][x - 1] } else { '.' },
        map[y + 1][x],
        if y > 0 { map[y - 1][x] } else { '.' },
        map[y][x + 1],
    ) {
        (_, '|' | 'J' | 'L', '|' | 'F' | '7', _) => '|',
        ('-' | 'F' | 'L', _, _, '-' | 'J' | '7') => '-',
        (_, '|' | 'J' | 'L', _, '-' | 'J' | '7') => 'F',
        ('-' | 'F' | 'L', '|' | 'J' | 'L', _, _) => '7',
        ('-' | 'F' | 'L', _, '|' | 'F' | '7', _) => 'J',
        (_, _, '|' | 'F' | '7', '-' | 'J' | '7') => 'L',
        _ => 's',
    };
    map[y][x] = s_pipe;

    while !stack.is_empty() {
        let (x, y): (i32, i32) = stack.pop().unwrap();
        let val = memo[y as usize][x as usize];

        match map[y as usize][x as usize] {
            'L' => {
                memo[y as usize][x as usize] = 1;
                visit(&mut stack, &mut memo, x, y - 1, val);
                visit(&mut stack, &mut memo, x + 1, y, val);
            }
            'J' => {
                memo[y as usize][x as usize] = 1;
                visit(&mut stack, &mut memo, x, y - 1, val);
                visit(&mut stack, &mut memo, x - 1, y, val);
            }
            '7' => {
                memo[y as usize][x as usize] = 1;
                visit(&mut stack, &mut memo, x, y + 1, val);
                visit(&mut stack, &mut memo, x - 1, y, val);
            }
            'F' => {
                memo[y as usize][x as usize] = 1;
                visit(&mut stack, &mut memo, x, y + 1, val);
                visit(&mut stack, &mut memo, x + 1, y, val);
            }
            '-' => {
                memo[y as usize][x as usize] = 1;
                visit(&mut stack, &mut memo, x - 1, y, val);
                visit(&mut stack, &mut memo, x + 1, y, val);
            }
            '|' => {
                memo[y as usize][x as usize] = 1;
                visit(&mut stack, &mut memo, x, y - 1, val);
                visit(&mut stack, &mut memo, x, y + 1, val);
            }
            _ => {}
        }
    }

    let mut is_pipe = false;
    let mut prev = '.';
    let mut sum = 0;
    for y in 0..map_h {
        let mut counter = 0;
        for x in 0..map_w {
            let c = map[y][x];
            let m = memo[y][x];

            if m == 1 && matches!(c, 'F' | '7' | 'J' | 'L') {
                if is_pipe && ((prev == 'F' && c == 'J') || (prev == 'L' && c == '7')) {
                    counter += 1;
                }

                is_pipe = !is_pipe;
                prev = c;
            } else if m == 1 && c == '|' {
                counter += 1;
            } else if m == 0 && counter % 2 == 1 {
                memo[y][x] = 9;
                sum += 1;
            }
        }
    }
    println!("part 2: {}", sum);
}

fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
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
    let fi = "tests/d10/input2.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");

    b.iter(|| part2(&reader))
}
