#![feature(test)]
extern crate test;

#[derive(Debug, PartialEq, PartialOrd)]
enum Type {
    HC,       // everything else
    OneP,     // one is two,
    TwoP,     // two is two, one dont care
    ThreeOfk, // one is three, two is one
    FullH,    // one is three, one is two
    FourOfk,  // one is four
    FiveOfk,  // one is five
}

fn part1(inp: &str) {
    let mut st = vec![0; 256];
    st[b'A' as usize] = 12;
    st[b'K' as usize] = 11;
    st[b'Q' as usize] = 10;
    st[b'J' as usize] = 9;
    st[b'T' as usize] = 8;
    st[b'9' as usize] = 7;
    st[b'8' as usize] = 6;
    st[b'7' as usize] = 5;
    st[b'6' as usize] = 4;
    st[b'5' as usize] = 3;
    st[b'4' as usize] = 2;
    st[b'3' as usize] = 1;
    st[b'2' as usize] = 0;

    let mut plays = Vec::new();
    for line in inp.lines() {
        let [k, n] = line.split(" ").collect::<Vec<&str>>()[..] else {
            unreachable!();
        };

        let mut check = vec![0; 13];

        let bid: i32 = n.parse().unwrap();
        let s: Vec<i32> = k.bytes().map(|val| st[val as usize]).collect();
        for v in &s {
            check[*v as usize] += 1;
        }

        let mut ty = Type::HC;
        for curr in check {
            match curr {
                5 => {
                    ty = Type::FiveOfk;
                    break;
                }
                4 => {
                    ty = Type::FourOfk;
                    break;
                }
                3 => {
                    if ty == Type::OneP {
                        ty = Type::FullH;
                        break;
                    }
                    ty = Type::ThreeOfk;
                }
                2 => {
                    if ty == Type::ThreeOfk {
                        ty = Type::FullH;
                        break;
                    } else if ty == Type::OneP {
                        ty = Type::TwoP;
                        break;
                    }
                    ty = Type::OneP;
                }
                _ => {}
            }
        }

        plays.push((ty, s, bid));
    }

    plays.sort_by(|(t1, s1, _), (t2, s2, _)| {
        if t1 == t2 {
            s1.cmp(s2)
        } else {
            t1.partial_cmp(t2).unwrap()
        }
    });

    let sum = plays
        .iter()
        .enumerate()
        .fold(0, |acc, v| acc + (v.0 + 1) as i32 * v.1 .2);
    println!("part 1: {}", sum);
}

fn part2(inp: &str) {
    let mut st = vec![0; 256];

    st[b'A' as usize] = 12;
    st[b'K' as usize] = 11;
    st[b'Q' as usize] = 10;
    st[b'T' as usize] = 9;
    st[b'9' as usize] = 8;
    st[b'8' as usize] = 7;
    st[b'7' as usize] = 6;
    st[b'6' as usize] = 5;
    st[b'5' as usize] = 4;
    st[b'4' as usize] = 3;
    st[b'3' as usize] = 2;
    st[b'2' as usize] = 1;
    st[b'J' as usize] = 0;

    let mut plays = Vec::new();
    for line in inp.lines() {
        let [k, n] = line.split(" ").collect::<Vec<&str>>()[..] else {
            unreachable!();
        };

        let mut check = vec![0; 13];

        let bid: i32 = n.parse().unwrap();
        let s: Vec<i32> = k.bytes().map(|val| st[val as usize]).collect();
        for v in &s {
            check[*v as usize] += 1;
        }

        let mut ty = Type::HC;

        let n_zero = check[0];
        check[0] = 0;

        check.sort();
        check.reverse();
        check[0] += n_zero;

        for curr in check {
            match curr {
                5 => {
                    ty = Type::FiveOfk;
                    break;
                }
                4 => {
                    ty = Type::FourOfk;
                    break;
                }
                3 => {
                    if ty == Type::OneP {
                        ty = Type::FullH;
                        break;
                    }
                    ty = Type::ThreeOfk;
                }
                2 => {
                    if ty == Type::ThreeOfk {
                        ty = Type::FullH;
                        break;
                    } else if ty == Type::OneP {
                        ty = Type::TwoP;
                        break;
                    }
                    ty = Type::OneP;
                }
                _ => {}
            }
        }

        plays.push((ty, s, bid));
    }

    plays.sort_by(|(t1, s1, _), (t2, s2, _)| {
        if t1 == t2 {
            s1.cmp(s2)
        } else {
            t1.partial_cmp(t2).unwrap()
        }
    });

    let sum = plays
        .iter()
        .enumerate()
        .fold(0, |acc, v| acc + (v.0 + 1) as i32 * v.1.2);
    println!("part 2: {}", sum);
}

fn main() {
    let fi = "tests/d7/input1.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");
    part1(&reader);
    part2(&reader);
}

#[bench]
fn pt1(b: &mut test::Bencher) {
    let fi = "tests/d7/input1.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");
    b.iter(|| part1(&reader))
}

#[bench]
fn pt2(b: &mut test::Bencher) {
    let fi = "tests/d7/input1.txt";
    let reader = std::fs::read_to_string(fi).expect("read input");
    b.iter(|| part2(&reader))
}
