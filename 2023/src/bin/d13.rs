use std::time::Instant;

pub fn part1(inp: Vec<u8>) -> i64 {
    let parts = inp.iter().position(|v|"\n\n");
    let mut sum = 0;
    for part in parts {
        let col_len = part.trim().chars().position(|v| v == '\n').unwrap();
        let row_len = part.trim().len() / (col_len + 1);
        let row_len = row_len + 1;
        let mut cols = vec![0; col_len];
        let mut rows = vec![0; row_len];

        for (i, val) in part.char_indices() {
            if val == '#' {
                let x = i%(col_len+1);
                let y = i/(col_len+1);
                cols[x] += 2_i64.pow(y as u32);
                rows[y] += 2_i64.pow(x as u32);
            }
        }

        let mut split = -1;
        for i in 1..col_len {
            let mut is_mirror = true;
            for j in 0..(i.min(col_len - i)) {
                if cols[i - j - 1] != cols[i + j] {
                    is_mirror = false;
                }
            }
            if is_mirror {
                split = i as i64;
                break;
            }
        }

        if split > 0 {
            sum += split;
            continue;
        }
        for i in 1..row_len {
            let mut is_mirror = true;
            for j in 0..(i.min(row_len - i)) {
                if rows[i - j - 1] != rows[i + j] {
                    is_mirror = false;
                }
            }
            if is_mirror {
                split = i as i64;
                break;
            }
        }

        sum += split * 100;
    }

    sum as i64
}

fn calc_split(inp: &Vec<i64>) -> i64 {
    let col_len = inp.len();
    let cols = inp;
    let mut split = -1;
    for i in 1..col_len {
        let mut has_smudge = false;
        let mut is_mirror = true;
        for j in 0..(i.min(col_len - i)) {
            let (l, r) = (cols[i - j - 1], cols[i + j]);
            if l != r {
                if (l ^ r).count_ones() == 1 {
                    if has_smudge {
                        is_mirror = false;
                        break;
                    } else {
                        has_smudge = true;
                    }
                } else {
                    is_mirror = false;
                    break;
                }
            }
        }
        if is_mirror && has_smudge {
            split = i as i64;
            break;
        }
    }
    split
}

pub fn part2(inp: &str) -> i64 {
    let parts = inp.split("\n\n");
    let mut sum = 0;
    for part in parts {
        let col_len = part.trim().chars().position(|v| v == '\n').unwrap();
        let row_len = part.trim().len() / (col_len + 1);
        let row_len = row_len + 1;
        let mut cols = vec![0; col_len];
        let mut rows = vec![0; row_len];

        for (i, val) in part.char_indices() {
            if val == '#' {
                let x = i%(col_len+1);
                let y = i/(col_len+1);
                cols[x] += 2_i64.pow(y as u32);
                rows[y] += 2_i64.pow(x as u32);
            }
        }

        let split = calc_split(&cols);
        if split > 0 {
            sum += split;
            continue;
        }
        let split = calc_split(&rows);
        sum += split * 100;
    }

    sum as i64
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
        // "input3.txt",
    ];

    for fi in inputs {
        let path = "tests/d13/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
