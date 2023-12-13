use std::iter::zip;

fn part1(inp: &str) {
    let [a, b] = inp.lines().collect::<Vec<&str>>()[..] else {
        unreachable!()
    };
    let times = a.split(" ").filter_map(|val| val.parse::<i32>().ok());
    let records = b.split(" ").filter_map(|val| val.parse::<i32>().ok());

    let mut prod = 1;
    for (time, record) in zip(times, records) {
        let mut count = 1;
        for press_duration in 0..=time {
            count += ((time - press_duration) * press_duration > record) as u64
        }
        prod *= count - 1;
    }

    println!("part 1: {}", prod);
}

fn part2(inp: &str) {
    let [a, b] = inp.lines().collect::<Vec<&str>>()[..] else {
        unreachable!()
    };
    let x = a
        .replace(" ", "")
        .replace("Time:", "")
        .parse::<u64>()
        .unwrap();
    let c = b
        .replace(" ", "")
        .replace("Distance:", "")
        .parse::<u64>()
        .unwrap();

    let mut count = 0;
    for y in 0..=x {
        count += ((x - y) * y > c) as u64
    }

    println!("part 2: {}", count);
}


fn p1_v2(inp: &str) {
    let [a, b] = inp.lines().collect::<Vec<&str>>()[..] else {
        unreachable!()
    };
    let times = a.split(" ").filter_map(|val| val.parse::<i32>().ok());
    let records = b.split(" ").filter_map(|val| val.parse::<i32>().ok());

    let mut prod = 1;
    for (time, dist) in zip(times, records) {
        let a = 1.0;
        let b = -time as f64;
        let c = dist as f64;

        let det = (b * b - 4.0 * a * c).sqrt();
        let x1 = (-b + det) / (2.0 * a);
        let x2 = (-b - det) / (2.0 * a);

        let max = x1.max(x2).ceil();
        let min = x1.min(x2).floor();

        let count = max - min - 1.0;
        prod *= count as i64;
    }

    println!("part 1 v2: {}", prod);
}

fn p2_v2(inp: &str) {
    let [a, b] = inp.lines().collect::<Vec<&str>>()[..] else {
        unreachable!()
    };
    let time = a
        .replace(" ", "")
        .replace("Time:", "")
        .parse::<i64>()
        .unwrap();
    let dist = b
        .replace(" ", "")
        .replace("Distance:", "")
        .parse::<i64>()
        .unwrap();

    let a = 1.0;
    let b = -time as f64;
    let c = dist as f64;

    let det = (b * b - 4.0 * a * c).sqrt();
    let x1 = (-b + det) / (2.0 * a);
    let x2 = (-b - det) / (2.0 * a);

    let max = x1.max(x2).ceil();
    let min = x1.min(x2).floor();

    let count = max - min - 1.0;
    let count = count as i64;

    println!("part 2 v2: {}", count);
}

#[allow(dead_code)]
fn main() {
    let fi = "tests/d6/input1.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");
    part1(&reader);
    part2(&reader);
    p1_v2(&reader);
    p2_v2(&reader);
}
