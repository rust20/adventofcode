use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

type Point = (i32, i32);
const DIR: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn u2point(x: usize, y: usize) -> Point {
    (x as i32, y as i32)
}

fn point2u(p: Point) -> (usize, usize) {
    (p.0 as usize, p.1 as usize)
}

fn valid_move(from: i32, to: i32) -> bool {
    to - from <= 1
}

#[allow(dead_code)]
fn bfs(s: Point, e: Point, map: &Vec<Vec<i32>>, mark: &mut Vec<Vec<i32>>) {
    let x_len = map[0].len() as i32;
    let y_len = map.len() as i32;

    let mut queue: VecDeque<Point> = VecDeque::new();

    queue.push_back(s);

    loop {
        let curr = match queue.pop_front() {
            Some(p) => p,
            None => break,
        };

        if curr == e {
            continue;
        }

        let curr_dist = mark[curr.1 as usize][curr.0 as usize];
        let u_curr = point2u(curr);

        for i in 0..4 {
            let next = (curr.0 + DIR[i].0, curr.1 + DIR[i].1);
            if next.1 < 0 || next.1 >= y_len || next.0 < 0 || next.0 >= x_len {
                continue;
            }

            let u_next = point2u(next);
            if !valid_move(map[u_curr.1][u_curr.0], map[u_next.1][u_next.0]) {
                continue;
            }

            let next_dist = mark[next.1 as usize][next.0 as usize];
            if next_dist <= curr_dist + 1 {
                continue;
            }

            mark[next.1 as usize][next.0 as usize] = curr_dist + 1;

            queue.push_back(next);
        }
    }
}

#[allow(dead_code)]
fn dfs(s: Point, e: Point, map: &Vec<Vec<i32>>, mark: &mut Vec<Vec<i32>>) {
    if s == e {
        return;
    }

    let x_len = map[0].len() as i32;
    let y_len = map.len() as i32;

    let u_s = point2u(s);
    let curr_val = mark[u_s.1][u_s.0];

    for i in 0..4 {
        let next = (s.0 + DIR[i].0, s.1 + DIR[i].1);
        let u_next = point2u(next);

        if next.0 < 0 || next.0 >= x_len || next.1 < 0 || next.1 >= y_len {
            continue;
        }

        if !valid_move(map[u_s.1][u_s.0], map[u_next.1][u_next.0]) {
            continue;
        }

        if mark[u_next.1][u_next.0] <= curr_val + 1 {
            continue;
        }

        mark[u_next.1][u_next.0] = curr_val + 1;

        dfs(next, e, map, mark);
    }
}

fn part1(input_raw: String) -> i32 {
    let input: Vec<Vec<char>> = input_raw
        .split_terminator("\n")
        .map(|x| x.chars().collect())
        .collect();

    let row = input.len();
    let col = input[0].len();

    // let high_val = i32::MAX;
    let high_val = 1_000_000;

    let mut marker = vec![vec![high_val; col]; row];
    let mut map = vec![vec![0; col]; row];

    let mut start: Point = (-1, -1);
    let mut end: Point = (-1, -1);

    for y in 0..row {
        for x in 0..col {
            map[y][x] = match input[y][x] {
                'S' => 0,
                'E' => 'z' as i32 - 'a' as i32,
                _ => input[y][x] as i32 - 'a' as i32,
            };

            if input[y][x] == 'S' {
                start = u2point(x, y);
            } else if input[y][x] == 'E' {
                end = u2point(x, y);
            }
        }
    }

    marker[start.1 as usize][start.0 as usize] = 0;

    bfs(start, end, &map, &mut marker);
    // dfs(start, end, &map, &mut marker2);

    marker[end.1 as usize][end.0 as usize]
}

#[allow(dead_code)]
fn print_wwww(row: usize, col: usize, marker: Vec<Vec<i32>>, input: &Vec<Vec<char>>, high_val: i32) {
    for y in 0..row {
        for x in 0..col {
            let a = if marker[y][x] != high_val {
                marker[y][x]
            } else {
                -1
            };
            print!("{}{:0width$} ", input[y][x], a, width = 3);
        }
        println!();
    }
}

fn part2(input_raw: String) -> i32 {
    let input: Vec<Vec<char>> = input_raw
        .split_terminator("\n")
        .map(|x| x.chars().collect())
        .collect();

    let row = input.len();
    let col = input[0].len();

    // let high_val = i32::MAX;
    let high_val = 1_000_000;

    let marker = vec![vec![high_val; col]; row];
    let mut map = vec![vec![0; col]; row];

    let mut starts: Vec<Point> = Vec::new();
    let mut end: Point = (-1, -1);

    for y in 0..row {
        for x in 0..col {
            map[y][x] = match input[y][x] {
                'S' => 0,
                'E' => 'z' as i32 - 'a' as i32,
                _ => input[y][x] as i32 - 'a' as i32,
            };

            if input[y][x] == 'S' || input[y][x] == 'a'{
                starts.push(u2point(x, y));
            } else if input[y][x] == 'E' {
                end = u2point(x, y);
            }
        }
    }

    starts.iter().map(|x| {
        let mut t = marker.clone();
        t[x.1 as usize][x.0 as usize] = 0;

        bfs(*x, end, &map, &mut t);
        // dfs(*x, end, &map, &mut t);
        t[end.1 as usize][end.0 as usize] as i32
    }).min().unwrap()


}

fn main() {
    let input_raw = get_input("input.txt");

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);

    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_case_1() {
        assert_eq!(part1(get_input("input_test.txt")), 31);
    }

    #[test]
    fn test_part2_case_1() {
        assert_eq!(part2(get_input("input_test.txt")), 29);
    }
}
