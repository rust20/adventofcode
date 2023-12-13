use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn part1(inp: &str) {
    let sp = inp.find("\n\n").unwrap();
    let path: Vec<u8> = inp[0..sp].bytes().collect();
    let tree = &inp[sp..].trim();

    let mut hs = HashMap::new();

    for line in tree.lines() {
        let [parent, children_tmp] = line.split(" = ").collect::<Vec<&str>>()[..] else {
            unreachable!()
        };

        let [left, right] = children_tmp[1..children_tmp.len() - 1]
            .split(", ")
            .collect::<Vec<&str>>()[..]
        else {
            unreachable!()
        };

        hs.insert(parent, (left, right));
    }

    let finish = "ZZZ";
    let mut curr = "AAA";
    let mut pos = 0;
    let mut step = 0;
    while curr != finish {
        match path[pos] {
            b'L' => curr = hs[curr].0,
            b'R' => curr = hs[curr].1,
            _ => {}
        }
        pos = (pos + 1) % path.len();
        step += 1;
    }

    println!("part 1: {}", step);
}

fn part2(inp: &str) {
    let sp = inp.find("\n\n").unwrap();
    let path: Vec<u8> = inp[0..sp].bytes().collect();
    let tree = &inp[sp..].trim();

    let mut positions = Vec::new();
    let mut finish = HashSet::new();
    let mut hs = HashMap::new();

    let mut map = HashMap::new();
    let mut count = 0;

    for line in tree.lines() {
        let [parent, children_tmp] = line.split(" = ").collect::<Vec<&str>>()[..] else {
            unreachable!()
        };

        let [left, right] = children_tmp[1..children_tmp.len() - 1]
            .split(", ")
            .collect::<Vec<&str>>()[..]
        else {
            unreachable!()
        };

        if parent.ends_with("A") {
            positions.push(parent);
        } else if parent.ends_with("Z") {
            finish.insert(parent);
        }

        hs.insert(parent, (left, right));
        map.insert(parent, count);
        count += 1;
    }

    let mut left = vec![0; map.len()];
    let mut right = vec![0; map.len()];
    for (k, v) in &hs {
        let p = map[k];
        let l = map[v.0];
        let r = map[v.1];
        left[p] = l;
        right[p] = r;
    }

    let mut loc = vec![(0, 0); positions.len()];
    let mut pos = positions.iter().map(|val| map[val]).collect::<Vec<usize>>();
    let fin = finish.iter().map(|val| map[val]).collect::<Vec<usize>>();

    let mut done = false;
    let mut step = 0;
    while !done {
        match path[step % path.len()] {
            b'L' => {
                for p in pos.iter_mut() {
                    *p = left[*p];
                }
            }
            b'R' => {
                for p in pos.iter_mut() {
                    *p = right[*p];
                }
            }
            _ => {}
        }
        step += 1;
        done = loc.iter().all(|(l, _)| *l != 0);
        for (i, val) in pos.iter().enumerate() {
            if fin.contains(val) {
                if loc[i].0 == 0 {
                    loc[i].0 = *val;
                    loc[i].1 = step;
                }
            }
        }
    }

    let mut prod = 1;
    for prob in loc {
        prod = lcm(prod, prob.1 as u64);
    }

    println!("part 2: {}", prod);
}

fn lcm(v1: u64, v2: u64) -> u64 {
    let mut a = v1;
    let mut b = v2;
    while b != 0 {
        (a, b) = (b, a % b)
    }
    v1 * v2 / a
}

fn part2_slow(inp: &str) {
    let sp = inp.find("\n\n").unwrap();
    let path: Vec<u8> = inp[0..sp].bytes().collect();
    let tree = &inp[sp..].trim();

    let mut positions = Vec::new();
    let mut finish = HashSet::new();
    let mut hs = HashMap::new();

    for line in tree.lines() {
        let [parent, children_tmp] = line.split(" = ").collect::<Vec<&str>>()[..] else {
            unreachable!()
        };

        let [left, right] = children_tmp[1..children_tmp.len() - 1]
            .split(", ")
            .collect::<Vec<&str>>()[..]
        else {
            unreachable!()
        };

        if parent.ends_with("A") {
            positions.push(parent);
        } else if parent.ends_with("Z") {
            finish.insert(parent);
        }

        hs.insert(parent, (left, right));
    }

    let mut l = vec![("", 0); positions.len()];

    let mut done = false;
    let mut step = 0;
    while !done {
        match path[step % path.len()] {
            b'L' => {
                for p in positions.iter_mut() {
                    *p = hs[*p].0;
                }
            }
            b'R' => {
                for p in positions.iter_mut() {
                    *p = hs[*p].1;
                }
            }
            _ => {}
        }

        step += 1;
        done = l.iter().all(|(l, _)| !l.is_empty());
        for (i, val) in positions.iter().enumerate() {
            if finish.contains(val) {
                if l[i].0.is_empty() {
                    l[i].0 = *val;
                    l[i].1 = step;
                }
            }
        }
    }

    let mut prod = 1;
    for prob in l {
        prod = lcm(prod, prob.1 as u64);
    }

    // println!("part 2: {}", prod);
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
        "input3.txt",
        // "input4.txt"
    ];

    for fi in inputs {
        let path = "tests/d8/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        println!(">> {}", fi);
        let start1 = Instant::now();
        part1(&reader);
        let start2 = Instant::now();
        part2(&reader);
        let start2_slow = Instant::now();
        part2_slow(&reader);
        let finish = start2_slow.elapsed();
        println!(
            "time1 {:?}, time2 {:?}, time2_slow {:?}",
            start2 - start1,
            start2_slow - start2,
            finish
        );
        println!()
    }
}
