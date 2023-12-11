#![feature(test)]

use std::time::Instant;

extern crate test;

fn part1(inp: &str) {
    let map: Vec<char> = inp.chars().collect();
    let width = map.iter().position(|&x| x == '\n').unwrap();
    let height = map.len() / width - 1;

    let mut cols = vec![true; width];
    let mut rows = vec![true; height];
    let mut pos = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y * (width + 1) + x] == '#' {
                pos.push((x as i64, y as i64));
                cols[x] = false;
                rows[y] = false;
            }
        }
    }

    let mut x = 0;
    let cols: Vec<i32> = cols
        .iter()
        .map(|&v| {
            if v {
                x += 1
            }
            x
        })
        .collect();
    let mut y = 0;
    let rows: Vec<i32> = rows
        .iter()
        .map(|&v| {
            if v {
                y += 1
            }
            y
        })
        .collect();

    let scale = 2;

    pos.iter_mut().for_each(|p| {
        let (mut x, mut y): (i64, i64) = *p;
        let count = cols[x as usize];
        x += count as i64 * (scale - 1);
        let count = rows[y as usize];
        y += count as i64 * (scale - 1);
        *p = (x, y);
    });

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
            if map[y * (width + 1) + x] == '#' {
                pos.push((x as i64, y as i64));
                cols[x] = false;
                rows[y] = false;
            }
        }
    }

    let mut x = 0;
    let cols: Vec<i32> = cols
        .iter()
        .map(|&v| {
            if v {
                x += 1
            }
            x
        })
        .collect();
    let mut y = 0;
    let rows: Vec<i32> = rows
        .iter()
        .map(|&v| {
            if v {
                y += 1
            }
            y
        })
        .collect();

    let scale = 1000000;

    pos.iter_mut().for_each(|p| {
        let (mut x, mut y): (i64, i64) = *p;
        let count = cols[x as usize];
        x += count as i64 * (scale - 1);
        let count = rows[y as usize];
        y += count as i64 * (scale - 1);
        *p = (x, y);
    });

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
