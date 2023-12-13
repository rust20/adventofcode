use std::time::Instant;

fn check_fix(lines: &str, x: usize, y: usize, expect_ver: bool, expect_loc: i64) -> bool {
    let col_len = lines.trim().chars().position(|v| v == '\n').unwrap();
    let row_len = lines.trim().len() / (col_len + 1);

    // println!("{}", lines.trim());
    let mut part = lines.to_string().bytes().collect::<Vec<u8>>();
    if part[y * (col_len + 1) + x] == b'.' {
        part[y * (col_len + 1) + x] = b'#';
    } else {
        part[y * (col_len + 1) + x] = b'.';
    }

    // println!("Next");
    // println!("{}", String::from_utf8(part.clone()).unwrap());

    let row_len = row_len + 1;
    let mut cols = vec![0; col_len];
    let mut rows = vec![0; row_len];

    // println!("[{} {}] {}", col_len, row_len, part.trim().len());

    for i in 0..row_len {
        let line = &part[i * (col_len + 1)..(i + 1) * (col_len + 1) - 1];
        // println!("{} {}", i, String::from_utf8(line.to_vec()).unwrap());
        for (j, val) in line.iter().enumerate() {
            if *val == b'#' {
                cols[j] += 2_i64.pow(i as u32);
                rows[i] += 2_i64.pow(j as u32);
            }
        }
    }

    // println!("{:?}", cols);
    // println!("{:?}", rows);

    let mut split = -1;
    for i in 1..col_len {
        // println!("checking at split {}", i);
        let mut is_mirror = true;
        for j in 0..(i.min(col_len - i)) {
            if cols[i - j - 1] != cols[i + j] {
                is_mirror = false;
            }
        }
        if is_mirror {
            if (true, i as i64) == first_sol(lines) {
                continue;
            }
            split = i as i64;
            break;
        }
    }

    // println!("{}", &part);
    if split > 0 {
        println!("expect is vert {}", split);
        return expect_ver && split == expect_loc;
    }
    for i in 1..row_len {
        // println!("checking at hor split {}", i);
        let mut is_mirror = true;
        for j in 0..(i.min(row_len - i)) {
            // println!("compare {} {} | {} {}",i-j-1,i+j, rows[i-j-1], rows[i+j]);
            if rows[i - j - 1] != rows[i + j] {
                is_mirror = false;
            }
        }
        if is_mirror {
            if (false, i as i64) == first_sol(lines) {
                continue;
            }
            split = i as i64;
            break;
        }
    }

    println!("expect is hor {}", split);
    return !expect_ver && split == expect_loc;
}

fn first_sol(lines: &str) -> (bool, i64) {
    let col_len = lines.trim().chars().position(|v| v == '\n').unwrap();
    let row_len = lines.trim().len() / (col_len + 1);

    // println!("{}", lines.trim());
    let part = lines.to_string().bytes().collect::<Vec<u8>>();
    // part[y * (row_len + 1) + x] = b'.';
    // println!("Next");
    // println!("{}", String::from_utf8(part.clone()).unwrap());

    let row_len = row_len + 1;
    let mut cols = vec![0; col_len];
    let mut rows = vec![0; row_len];

    // println!("[{} {}] {}", col_len, row_len, part.trim().len());

    for i in 0..row_len {
        let line = &part[i * (col_len + 1)..(i + 1) * (col_len + 1) - 1];
        // println!("{} {}", i, String::from_utf8(line.to_vec()).unwrap());
        for (j, val) in line.iter().enumerate() {
            if *val == b'#' {
                cols[j] += 2_i64.pow(i as u32);
                rows[i] += 2_i64.pow(j as u32);
            }
        }
    }

    // println!("{:?}", cols);
    // println!("{:?}", rows);

    let mut split = -1;
    for i in 1..col_len {
        // println!("checking at split {}", i);
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

    // println!("{}", &part);
    if split > 0 {
        println!("orig is vert {}", split);
        return (true, split as i64);
    }
    for i in 1..row_len {
        // println!("checking at hor split {}", i);
        let mut is_mirror = true;
        for j in 0..(i.min(row_len - i)) {
            // println!("compare {} {} | {} {}",i-j-1,i+j, rows[i-j-1], rows[i+j]);
            if rows[i - j - 1] != rows[i + j] {
                is_mirror = false;
            }
        }
        if is_mirror {
            split = i as i64;
            break;
        }
    }

    println!("orig is hor {}", split);
    return (false, split as i64);
}

pub fn part1(inp: &str) -> i64 {
    let parts = inp.split("\n\n");
    let mut sum = 0;
    for part in parts {
        let col_len = part.trim().chars().position(|v| v == '\n').unwrap();
        let row_len = part.trim().len() / (col_len + 1);
        let row_len = row_len + 1;
        let mut cols = vec![0; col_len];
        let mut rows = vec![0; row_len];

        // println!("[{} {}] {}", col_len, row_len, part.trim().len());

        for (i, line) in part.lines().enumerate() {
            // println!("{} {}", i, line);
            for (j, val) in line.char_indices() {
                if val == '#' {
                    cols[j] += 2_i64.pow(i as u32);
                    rows[i] += 2_i64.pow(j as u32);
                }
            }
        }

        let mut split = -1;
        for i in 1..col_len {
            // println!("checking at split {}", i);
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

        // println!("{}", &part);
        if split > 0 {
            // println!("is vert {}", split);
            // if (true, split) != first_sol(part) {
            //     println!("im shocked");
            // }
            sum += split;
            continue;
        }
        for i in 1..row_len {
            // println!("checking at hor split {}", i);
            let mut is_mirror = true;
            for j in 0..(i.min(row_len - i)) {
                // println!("compare {} {} | {} {}",i-j-1,i+j, rows[i-j-1], rows[i+j]);
                if rows[i - j - 1] != rows[i + j] {
                    is_mirror = false;
                }
            }
            if is_mirror {
                split = i as i64;
                break;
            }
        }

        // println!("is hor {}", split);
        // if (false, split) != first_sol(part) {
        //     println!("im shocked");
        // }
        sum += split * 100;
    }

    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let parts = inp.split("\n\n");
    let mut sum = 0;
    for part in parts {
        println!("=============================");
        println!("=============================");
        let col_len = part.trim().chars().position(|v| v == '\n').unwrap();
        let row_len = part.trim().len() / (col_len + 1);
        let row_len = row_len + 1;
        let mut cols = vec![0; col_len];
        let mut rows = vec![0; row_len];

        // println!("[{} {}] {}", col_len, row_len, part.trim().len());

        for (i, line) in part.lines().enumerate() {
            // println!("{} {}", i, line);
            for (j, val) in line.char_indices() {
                if val == '#' {
                    cols[j] += 2_i64.pow(i as u32);
                    rows[i] += 2_i64.pow(j as u32);
                }
            }
        }

        let mut smudge = (0, 0);
        let mut split = -1;
        for i in 1..col_len {
            // println!("checking at split {}", i);
            let mut has_smudge = false;
            let mut is_mirror = true;
            for j in 0..(i.min(col_len - i)) {
                let (l, r) = (cols[i - j - 1], cols[i + j]);
                println!(
                    "compare vert {} {} | {} {} | {}",
                    i - j - 1,
                    i + j,
                    l,
                    r,
                    has_smudge
                );
                if l != r {
                    if (l ^ r).count_ones() == 1 {
                        if has_smudge {
                            is_mirror = false;
                            break;
                        } else {
                            has_smudge = true;
                            let aaa = (l - r).abs().trailing_zeros();
                            smudge = (i - j - 1, aaa as usize);
                        }
                    } else {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if is_mirror && has_smudge {
                if (true, i as i64) == first_sol(part) {
                    continue;
                }
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
            let mut has_smudge = false;

            for j in 0..(i.min(row_len - i)) {
                let (l, r) = (rows[i - j - 1], rows[i + j]);
                if l != r {
                    if (l ^ r).count_ones() == 1 {
                        if has_smudge {
                            is_mirror = false;
                            break;
                        } else {
                            has_smudge = true;
                            let aaa = (l - r).abs().trailing_zeros();
                            smudge = (aaa as usize, i - j - 1);
                        }
                    } else {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if is_mirror && has_smudge {
                if (false, i as i64) == first_sol(part) {
                    continue;
                }
                split = i as i64;
                break;
            }
        }

        println!("is hor {} {:?}", split, smudge);
        if !check_fix(part, smudge.0, smudge.1, false, split) {
            println!("^something is wrong there");

            println!("{}", part.trim());
            let mut part = part.to_string().bytes().collect::<Vec<u8>>();
            let (x, y) = smudge;
            if part[y * (col_len + 1) + x] == b'.' {
                part[y * (col_len + 1) + x] = b'#';
            } else {
                part[y * (col_len + 1) + x] = b'.';
            }
            println!("Next");
            println!("{}", String::from_utf8(part.clone()).unwrap());
            println!("------------------------");
        }
        // if split < 0 {
        //     println!("is hor {}", split);
        // }
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
