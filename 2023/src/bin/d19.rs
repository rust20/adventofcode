use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
enum Stmt<'a> {
    Cmp(usize, &'a str, isize, &'a str),
    Dest(&'a str),
}
pub fn part1(inp: &str) -> i64 {
    let mut lines = inp.split("\n");
    let mut funcs = HashMap::new();

    while let Some(val) = lines.next() {
        if val.is_empty() {
            break;
        }

        let sep = val.find("{").unwrap();
        let func_name = &val[0..sep];

        let list = val[sep + 1..val.len() - 1]
            // .trim_matches(|v| matches!(v, '{' | '}'))
            .split(",")
            .map(|stmt| {
                let mut stmt_part = stmt.split(":");
                let cmp = stmt_part.next().unwrap();
                if let Some(num) = stmt_part.next() {
                    Stmt::Cmp(
                        match &cmp[0..1] {
                            "x" => 0,
                            "m" => 1,
                            "a" => 2,
                            "s" => 3,
                            _ => unreachable!(),
                        },
                        &cmp[1..2],
                        cmp[2..].parse().unwrap(),
                        num,
                    )
                } else {
                    Stmt::Dest(cmp)
                }
            })
            .collect::<Vec<Stmt>>();

        // println!(">{}", func_name);
        // for i in &list {
        //     println!(">> {:?}", i);
        // }

        funcs.insert(func_name, list);
    }

    let mut sum = 0;
    while let Some(val) = lines.next() {
        if val == "" {
            break;
        }
        let val: Vec<isize> = val
            .trim_matches(|v| matches!(v, '{' | '}'))
            .split(",")
            .map(|v| v[2..].parse().unwrap())
            .collect();
        let mut state = "in";
        loop {
            if state == "R" || state == "A" {
                break;
            }
            let conds = &funcs[state];
            for cond in conds {
                match cond {
                    Stmt::Cmp(lhs, cmp, rhs, dest) => {
                        let res = match *cmp {
                            "<" => val[*lhs] < *rhs,
                            ">" => val[*lhs] > *rhs,
                            _ => unreachable!(),
                        };
                        state = dest;
                        if res {
                            break;
                        }
                    }
                    Stmt::Dest(dest) => {
                        state = dest;
                    }
                }
            }
        }

        if state == "A" {
            sum += val.iter().sum::<isize>();
        }
    }

    sum as i64
}

pub fn part2(inp: &str) -> i64 {
    let mut lines = inp.split("\n");
    let mut funcs = HashMap::new();

    while let Some(val) = lines.next() {
        if val.is_empty() {
            break;
        }

        let sep = val.find("{").unwrap();
        let func_name = &val[0..sep];

        let list = val[sep + 1..val.len() - 1]
            .split(",")
            .map(|stmt| {
                let mut stmt_part = stmt.split(":");
                let cmp = stmt_part.next().unwrap();
                if let Some(num) = stmt_part.next() {
                    Stmt::Cmp(
                        match &cmp[0..1] {
                            "x" => 0,
                            "m" => 1,
                            "a" => 2,
                            "s" => 3,
                            _ => unreachable!(),
                        },
                        &cmp[1..2],
                        cmp[2..].parse().unwrap(),
                        num,
                    )
                } else {
                    Stmt::Dest(cmp)
                }
            })
            .collect::<Vec<Stmt>>();

        // println!(">{}", func_name);
        // for i in &list {
        //     println!(">> {:?}", i);
        // }

        funcs.insert(func_name, list);
    }

    let mut valid = Vec::new();
    let mut stack = Vec::new();
    stack.push(([(1, 4000), (1, 4000), (1, 4000), (1, 4000)], "in"));

    while let Some((mut val, state)) = stack.pop() {
        if state == "A" {
            valid.push(val);
            continue;
        }
        if state == "R" {
            continue;
        }
        let conds = &funcs[state];
        for cond in conds {
            match cond {
                Stmt::Cmp(lhs, cmp, rhs, dest) => {
                    match *cmp {
                        "<" => {
                            let mut val_acc = val;
                            val_acc[*lhs].1 = *rhs - 1;
                            val[*lhs].0 = *rhs;
                            stack.push((val_acc, dest))
                        }
                        ">" => {
                            let mut val_acc = val;
                            val_acc[*lhs].0 = *rhs + 1;
                            val[*lhs].1 = *rhs;
                            stack.push((val_acc, dest))
                        }
                        _ => unreachable!(),
                    };

                }
                Stmt::Dest(dest) => stack.push((val, dest)),
            }
        }
    }

    let mut sum: isize = 0;
    for v in valid {
        sum += v.iter().map(|v| v.1 - v.0 + 1).product::<isize>();
    }

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
        let path = "tests/d19/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
