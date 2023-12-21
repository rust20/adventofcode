use std::{collections::HashMap, time::Instant};


pub fn part1(inp: &str) -> i64 {

    let mut points = vec![];

    for line in inp.lines() {
        let mut k = line.split(" ");
        let dir = k.next().unwrap();
        let len = k.next().unwrap().parse::<isize>().unwrap();
        let color = k.next().unwrap();

        points.push((dir, len, color));
    }

    points.push(points[0]);
    points.push(points[1]);

    let mut prev = (0_isize, 0_isize);
    let mut points_v2 = vec![];
    for i in 0..points.len() - 2 {
        let trip = &points[i..i + 3];
        let len = trip[1].1;
        let pos = match [trip[0].0, trip[1].0, trip[2].0] {
            ["R", "U", "L"] => (prev.0, prev.1 - (len - 1)),
            ["L", "U", "R"] => (prev.0, prev.1 - (len + 1)),
            ["R", "D", "L"] => (prev.0, prev.1 + (len + 1)),
            ["L", "D", "R"] => (prev.0, prev.1 + (len - 1)),

            ["U", "R", "D"] => (prev.0 + (len+1), prev.1),
            ["D", "R", "U"] => (prev.0 + (len-1), prev.1),
            ["U", "L", "D"] => (prev.0 - (len-1), prev.1),
            ["D", "L", "U"] => (prev.0 - (len+1), prev.1),

            ["R", "U", "R"] | ["L", "U", "L"] => (prev.0, prev.1 - len),
            ["R", "D", "R"] | ["L", "D", "L"] => (prev.0, prev.1 + len),
            ["U", "R", "U"] | ["D", "R", "D"] => (prev.0 + len , prev.1),
            ["U", "L", "U"] | ["D", "L", "D"] => (prev.0 - len , prev.1),
            _ => unreachable!(),
        };

        prev = pos;
        points_v2.push(pos);
    }

    let mut area = 0;
    for i in 0..points_v2.len() {
        let sec = (i + 1) % points_v2.len();
        area += points_v2[i].0 * points_v2[sec].1;
        area -= points_v2[i].1 * points_v2[sec].0;
    }

    let sum = area / 2;
    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let mut points = vec![];

    for line in inp.lines() {
        let mut k = line.split("#");
        let _ = k.next();
        let color = k.next().unwrap();
        let len = isize::from_str_radix(&color[..5], 16).unwrap();
        let dir = &color[5..6];

        points.push((dir, len, color));
    }

    points.push(points[0]);
    points.push(points[1]);

    let mut prev = (0_isize, 0_isize);
    let mut points_v2 = vec![];
    for i in 0..points.len() - 2 {
        let trip = &points[i..i + 3];
        let len = trip[1].1;
        let pos = match [trip[0].0, trip[1].0, trip[2].0] {
            ["0", "3", "2"] => (prev.0, prev.1 - (len - 1)),
            ["2", "3", "0"] => (prev.0, prev.1 - (len + 1)),
            ["0", "1", "2"] => (prev.0, prev.1 + (len + 1)),
            ["2", "1", "0"] => (prev.0, prev.1 + (len - 1)),

            ["3", "0", "1"] => (prev.0 + (len+1), prev.1),
            ["1", "0", "3"] => (prev.0 + (len-1), prev.1),
            ["3", "2", "1"] => (prev.0 - (len-1), prev.1),
            ["1", "2", "3"] => (prev.0 - (len+1), prev.1),

            ["0", "3", "0"] | ["2", "3", "2"] => (prev.0, prev.1 - len),
            ["0", "1", "0"] | ["2", "1", "2"] => (prev.0, prev.1 + len),
            ["3", "0", "3"] | ["1", "0", "1"] => (prev.0 + len , prev.1),
            ["3", "2", "3"] | ["1", "2", "1"] => (prev.0 - len , prev.1),
            _ => unreachable!(),
        };

        prev = pos;
        points_v2.push(pos);
    }

    let mut area = 0;
    for i in 0..points_v2.len() {
        let sec = (i + 1) % points_v2.len();
        area += points_v2[i].0 * points_v2[sec].1;
        area -= points_v2[i].1 * points_v2[sec].0;
    }

    let sum = area / 2;
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
        let path = "tests/d18/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
