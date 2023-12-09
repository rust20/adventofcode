use std::collections::HashMap;

fn part1(inp: &str) {
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut sum = 0;

    let map: Vec<Vec<char>> = inp
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let rl = map.len() as i32;
    let cl = map.first().unwrap().len() as i32;

    for row in 0..rl {
        let mut num = 0;
        let mut near_sym = false;

        for col in 0..cl {
            let curr = map[row as usize][col as usize];

            match curr {
                '0'..='9' => {
                    num = num * 10 + curr.to_digit(10).unwrap();
                    if near_sym {
                        continue;
                    }
                    for (yp, xp) in dirs {
                        let y = row as i32 + yp;
                        let x = col as i32 + xp;
                        if y < 0 || y >= rl || x < 0 || x >= cl {
                            continue;
                        }
                        match map[y as usize][x as usize] {
                            '0'..='9' | '.' => {}
                            _ => {
                                near_sym = true;
                                break;
                            }
                        }
                    }
                }
                _ => {
                    if num > 0 && near_sym {
                        sum += num;
                    }
                    num = 0;
                    near_sym = false;
                }
            }
        }
        if num > 0 && near_sym {
            sum += num;
        }
    }

    println!("part1: {}", sum)
}
fn part2(inp: &str) {
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let map: Vec<Vec<char>> = inp
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let rl = map.len() as i32;
    let cl = map.first().unwrap().len() as i32;

    let mut ht: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    for row in 0..rl {
        let mut num = 0;
        let mut gear = (-1, -1);

        for col in 0..cl {
            let curr = map[row as usize][col as usize];

            match curr {
                '0'..='9' => {
                    num = num * 10 + curr.to_digit(10).unwrap();
                    if gear != (-1, -1) {
                        continue;
                    }
                    for (yp, xp) in dirs {
                        let y = row as i32 + yp;
                        let x = col as i32 + xp;
                        if y < 0 || y >= rl || x < 0 || x >= cl {
                            continue;
                        }
                        match map[y as usize][x as usize] {
                            '*' => {
                                gear = (y, x);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    if num > 0 && gear != (-1, -1) {
                        if let Some(ratios) = ht.get_mut(&gear) {
                            ratios.push(num);
                        } else {
                            ht.insert(gear, vec![num]);
                        }
                    }
                    num = 0;
                    gear = (-1, -1);
                }
            }
        }
        if num > 0 && gear != (-1, -1) {
            if let Some(ratios) = ht.get_mut(&gear) {
                ratios.push(num);
            } else {
                ht.insert(gear, vec![num]);
            }
        }
    }

    let sum = ht
        .iter()
        .filter(|(_, val)| val.len() > 1)
        .map(|(_, val)| val.iter().fold(1, |acc, item| acc * item))
        .fold(0, |a, i| a + i);

    println!("part2: {}", sum)
}
fn main() {
    let file_input = "tests/d3/input1.txt";
    let reader = std::fs::read_to_string(file_input).expect("read input");

    part1(&reader);
    part2(&reader);
}
