use std::time::Instant;

pub fn part1(inp: &str) -> i64 {
    let sequence = inp.trim().split(",").collect::<Vec<&str>>();
    let mut sum = 0;
    for seq in sequence {
        let mut curr: i32 = 0;
        for i in seq.bytes() {
            let i = i as i32;
            curr = ((curr + i) * 17) & 0b11111111;
        }
        sum += curr;
    }
    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let mut boxes: Vec<Vec<(&str, &str)>> = vec![Vec::new(); 256];

    let sequence = inp.trim().split(",").collect::<Vec<&str>>();

    for seq in sequence {

        let pos: Vec<&str> = seq.split(&['-', '='][..]).collect();
        let label = pos[0];

        let mut hash: usize = 0;
        for i in label.bytes() {
            let i = i as usize;
            hash = ((hash + i) * 17) & 0b11111111;
        }


        if seq.contains(&"=") {
            if let Some(loc) = boxes[hash].iter().position(|(k, _)| k == &label) {
                boxes[hash][loc] = (label, pos[1])
            } else {
                boxes[hash].push((label, pos[1]))
            }
        } else {
            if let Some(loc) = boxes[hash].iter().position(|(k, _)| k == &label) {
                boxes[hash].remove(loc);
            }
        }
    }

    let sum: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, val)| {
            val.iter()
                .enumerate()
                .map(|(j, v)| (i + 1) * (j + 1) * v.1.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sum();

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
        let path = "tests/d15/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
