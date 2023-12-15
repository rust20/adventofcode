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
    let mut boxes: Vec<Vec<((usize, usize), usize)>> = vec![Vec::new(); 256];

    let mut cursor: usize = 0;
    let mut hash: usize = 0;
    let mut label: (usize, usize) = (0, 0);
    let inp_vec: Vec<u8> = inp.bytes().collect();
    while cursor < inp.len() {
        match inp_vec[cursor] {
            b'a'..=b'z' => {
                let i = inp_vec[cursor] as usize;
                hash = ((hash + i) * 17) & 0b11111111;
            }
            b'=' => {
                label.1 = cursor;
                cursor+=1;
                let foc_len = (inp_vec[cursor] - b'0') as usize;
                if let Some(loc) = boxes[hash].iter().position(|&(k, _)| inp[k.0..k.1] == inp[label.0..label.1]) {
                    boxes[hash][loc] = (label, foc_len)
                } else {
                    boxes[hash].push((label, foc_len))
                }
            }
            b'-' => {
                label.1 = cursor;
                if let Some(loc) = boxes[hash].iter().position(|&(k, _)| inp[k.0..k.1] == inp[label.0..label.1]) {
                    boxes[hash].remove(loc);
                }
            }
            b',' => {
                hash = 0;
                label.0 = cursor + 1;
            }
            _ => {}
        }
        cursor += 1;
    }

    let sum: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, val)| {
            val.iter()
                .enumerate()
                .map(|(j, v)| (i + 1) * (j + 1) * v.1)
                .sum::<usize>()
        })
        .sum();

    sum as i64
}
pub fn part2_2(inp: &str) -> i64 {
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

        let start2_2 = Instant::now();
        println!("part2 {}", part2_2(&reader));
        let finish_2 = start2_2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
        println!("time2_2 {:?}", finish_2);
    }
}
