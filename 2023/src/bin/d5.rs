fn part1(inp: &str) {
    let lines = inp.lines();
    let mut seeds = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line
                .trim_start_matches("seeds: ")
                .split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .map(|v| (v, v, true))
                .collect::<Vec<(i64, i64, bool)>>();
        } else if line.ends_with("map:") {
            for seed in seeds.iter_mut() {
                seed.2 = true;
            }
        } else {
            if let [dst, src, rn] = line
                .split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()[..]
            {
                for (_i, seed, m) in seeds.iter_mut() {
                    if *m && src <= *seed && *seed < src + rn {
                        *m = false;
                        *seed = *seed - src + dst;
                    }
                }
            }
        }
    }

    let low = seeds.iter().reduce(|a, b| if a.1 < b.1 {a} else {b}).unwrap();
    println!("part1: {}", low.1);
}

fn part2(inp: &str) {
    let lines = inp.lines();
    let mut seeds = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds: ") {
            let t =  line
                .trim_start_matches("seeds: ")
                .split(" ")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            for i in (0..t.len()).step_by(2) {
                seeds.push((t[i], t[i+1], false));
            }

        } else if line.ends_with("map:") {
            for seed in seeds.iter_mut() {
                seed.2 = false;
            }
        } else {
            if let [dst, src, rn] = line
                .split(" ")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()[..]
            {

                /* breakdown cases
                 *
                 * range seed : a1 a2
                 * range map  : rn1 rn2 
                 *
                 * // no split
                 * rn1 rn2 a1 a2
                 * a1 a2 rn1 rn2
                 * rn1 a1 a2 rn2 || rn1 <= a1 && a2 <= rn2 // case 1
                 * 
                 * // split 2
                 * a1 rn1 a2 rn2 || a1 < rn1 && rn1 < a2  // case 2
                 * rn1 a1 rn2 a2 || a1 < rn2 && rn2 < a2  // case 3
                 *
                 * // split 3
                 * a1 rn1 rn2 a2 || a1 < rn1 && rn2 < a2 // case 4
                 *
                 */

                let mut i = 0;
                while i < seeds.len() {
                    let (mut start, mut n, mut modified) = seeds[i];

                    let r1 = src;
                    let r2 = src + rn;
                    let s1 = start;
                    let s2 = start + n;

                    if modified || r2 <= s1 || s2 <= r1 {
                        i+=1;
                        continue;
                    }

                    if r1 <= s1 && s2 <= r2 {
                        // first case, no change
                    } else if s1 < r1 && r1 < s2 && s2 <= r2{
                        start = r1;
                        n = s2 - r1;

                        seeds.push((s1, r1 - s1, false));
                    } else if r1 <= s1 && s1 < r2 && r2 < s2 {
                        n = r2 - s1;

                        seeds.push((r2, s2 - r2, false));
                    } else if s1 < r1 && r2 < s2 {
                        start = r1;
                        n = r2 - r1;

                        seeds.push((s1, r1 - s1, false));
                        seeds.push((r2, s2 - r2, false));
                    }

                    modified = true;
                    start = start - src + dst;

                    seeds[i] = (start, n, modified);
                    i += 1;
                }
            }
        }
    }

    let low = seeds.iter().reduce(|a, b| if a.0 < b.0 {a} else {b}).unwrap();
    println!("part2: {}", low.0);
}

#[allow(dead_code)]
fn main() {
    let fi = "tests/d5/input1.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");

    part1(&reader);
    part2(&reader);
}
