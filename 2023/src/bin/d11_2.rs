#![feature(test)]

use std:: time::Instant;

extern crate test;

fn part1(inp: &str) {
    let start1  = Instant::now();

    let map: Vec<Vec<char>> = inp.lines().map(|v| v.chars().collect()).collect();
    let width = map[0].len();
    let height = map.len();

    println!("{:?}", start1.elapsed());

    let mut cols = vec![true; width];
    let mut rows = vec![true; height];
    let mut pos = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                cols[x] = false;
                rows[y] = false;
            }
        }
    }

    let mut yp = 0;
    for y in 0..height {
        if rows[y] {
            yp += 1;
            continue;
        }
        let mut xp = 0;
        for x in 0..width {
            if cols[x] {
                xp += 1;
                continue;
            }
            if map[y][x] == '#' {
                pos.push(((x + xp) as i64, (y + yp) as i64));
            }
        }
    }

    let mut sum = 0;
    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            let (x1, y1) = pos[i];
            let (x2, y2) = pos[j];
            let dist = (x1 - x2).abs() + (y1 - y2).abs();
            sum += dist;
        }
    }

    println!("part 1: {}", sum);
}
fn part2(inp: &str) {
    let map: Vec<char> = inp.chars().collect();
    let width = map.iter().position(|&x| x == '\n').unwrap();
    let height = map.len() / width - 1;

    let mut cols = vec![true; width];
    let mut rows = vec![true; height];
    let mut pos = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y * (width+1) + x] == '#' {
                cols[x] = false;
                rows[y] = false;
            }
        }
    }

    let scale = 1000000;

    let mut yp = 0;
    for y in 0..height {
        if rows[y] {
            yp += 1;
            continue;
        }
        let mut xp = 0;
        for x in 0..width {
            if cols[x] {
                xp += 1;
                continue;
            }
            if map[y * (width+1) + x] == '#' {
                let xp = xp * (scale-1);
                let yp = yp * (scale-1);
                pos.push(((x + xp) as i64, (y + yp) as i64));
            }
        }
    }

    let mut sum = 0;
    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            let (x1, y1) = pos[i];
            let (x2, y2) = pos[j];
            let dist = (x1 - x2).abs() + (y1 - y2).abs();
            sum += dist;
        }
    }

    println!("part 2: {}", sum);
}

fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d11/".to_string();
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

