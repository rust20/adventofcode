use std::fs::File;
use std::io::{BufReader, Read};

type Tup = (i32, i32);

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("tests/d08/input1.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

struct Map {
    trees: Vec<Vec<i32>>,
    visible: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(source: String) -> Self {
        let tree_map: Vec<Vec<i32>> = source
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        let row_max = tree_map.len() - 1;
        let col_max = tree_map[0].len() - 1;

        let visibility_map = vec![vec![false; col_max + 1]; row_max + 1];

        Self {
            trees: tree_map,
            visible: visibility_map,
        }
    }

    pub fn get_map_size(&self) -> (i32, i32) {
        (
            (self.trees.len() - 1) as i32,
            (self.trees[0].len() - 1) as i32,
        )
    }

    pub fn tget(&self, loc: Tup) -> i32 {
        self.trees[loc.0 as usize][loc.1 as usize]
    }

    pub fn set_visible(&mut self, loc: Tup) {
        self.visible[loc.0 as usize][loc.1 as usize] = true;
    }

    pub fn mark(&mut self, mut s: Tup, d: Tup) {
        let row_max = (self.trees.len() - 1) as i32;
        let col_max = (self.trees[0].len() - 1) as i32;

        let mut min = self.tget(s);

        s.0 += d.0;
        s.1 += d.1;
        while 0 < s.0 && s.0 < row_max && 0 < s.1 && s.1 < col_max {
            let curr = self.tget(s);

            if curr > min {
                self.set_visible(s);
                min = curr;
            }
            s.0 += d.0;
            s.1 += d.1;
        }
    }

    pub fn count_visible(&self) -> i32 {
        self.visible.iter().fold(0, |acc, val| {
            acc + val
                .iter()
                .fold(0, |acc1, val1| acc1 + if *val1 { 1 } else { 0 })
        })
    }

    #[allow(dead_code)]
    pub fn print_vis(&self) {
        self.visible.iter().for_each(|x| {
            x.iter()
                .for_each(|y| print!("{}", if *y { "1" } else { "0" }));
            println!()
        })
    }

    #[allow(dead_code)]
    pub fn print_trees(&self) {
        self.trees.iter().for_each(|x| {
            x.iter().for_each(|y| print!("{}", y));
            println!()
        })
    }

    pub fn get_scenic_score(&self, loc: Tup) -> i32 {
        let dir: Vec<Tup> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        dir.iter()
            .fold(1, |acc, dir| acc * self.scenic_dir(loc, *dir))
    }

    fn scenic_dir(&self, mut s: Tup, d: Tup) -> i32 {
        let (row_max, col_max) = self.get_map_size();

        let mut score = 1;
        let max = self.tget(s);

        s.0 += d.0;
        s.1 += d.1;
        while 0 < s.0 && s.0 < row_max && 0 < s.1 && s.1 < col_max {
            if self.tget(s) >= max {
                break;
            }
            score += 1;
            s.0 += d.0;
            s.1 += d.1;
        }

        score
    }
}

fn part1(input_raw: String) -> i32 {
    let mut map = Map::new(input_raw);
    let (row_max, col_max) = map.get_map_size();
    let dir: Vec<Tup> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for i in 1..row_max {
        map.mark((i as i32, 0), dir[2]);
        map.mark((i as i32, row_max), dir[3]);
    }

    for i in 1..col_max {
        map.mark((0, i as i32), dir[0]);
        map.mark((col_max, i as i32), dir[1]);
    }

    // map.print_vis();
    // map.print_trees();

    map.count_visible() + 2 * (row_max as i32 + col_max as i32)
}

fn part2(input_raw: String) -> i32 {
    let map = Map::new(input_raw);
    let (row_max, col_max) = map.get_map_size();

    let mut max_score = 0;

    for i in 0..=row_max {
        for j in 0..=col_max {
            let score = map.get_scenic_score((i, j));
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    let input_raw = get_input();

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);

    println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_case_1() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
    }
}
