use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tuple {
    x: i32,
    y: i32,
}

impl Tuple {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn move_tail(head: &Tuple, tail: &Tuple) -> Tuple {
    match (head, tail) {
        (h, t) if h.x == t.x - 1 && h.y == t.y + 2 => Tuple::new(t.x - 1, t.y + 1),
        (h, t) if h.x == t.x + 0 && h.y == t.y + 2 => Tuple::new(t.x + 0, t.y + 1),
        (h, t) if h.x == t.x + 1 && h.y == t.y + 2 => Tuple::new(t.x + 1, t.y + 1),

        (h, t) if h.x == t.x - 1 && h.y == t.y - 2 => Tuple::new(t.x - 1, t.y - 1),
        (h, t) if h.x == t.x + 0 && h.y == t.y - 2 => Tuple::new(t.x + 0, t.y - 1),
        (h, t) if h.x == t.x + 1 && h.y == t.y - 2 => Tuple::new(t.x + 1, t.y - 1),

        (h, t) if h.x == t.x + 2 && h.y == t.y - 1 => Tuple::new(t.x + 1, t.y - 1),
        (h, t) if h.x == t.x + 2 && h.y == t.y + 0 => Tuple::new(t.x + 1, t.y + 0),
        (h, t) if h.x == t.x + 2 && h.y == t.y + 1 => Tuple::new(t.x + 1, t.y + 1),

        (h, t) if h.x == t.x - 2 && h.y == t.y - 1 => Tuple::new(t.x - 1, t.y - 1),
        (h, t) if h.x == t.x - 2 && h.y == t.y + 0 => Tuple::new(t.x - 1, t.y + 0),
        (h, t) if h.x == t.x - 2 && h.y == t.y + 1 => Tuple::new(t.x - 1, t.y + 1),
        _ => tail.clone(),
    }
}

fn part1(input_raw: String) -> i32 {
    let input = input_raw.split("\n");
    let mut loc_set: HashSet<Tuple> = HashSet::new();
    let mut head = Tuple::new(0, 0);
    let mut tail = Tuple::new(0, 0);

    for command in input.into_iter() {
        if command.is_empty() {
            continue;
        }
        let args: Vec<&str> = command.split(" ").collect();
        let dir = args[0];
        let count: i32 = args[1].parse().unwrap();
        for _ in 0..count {
            head = match dir {
                "R" => Tuple::new(head.x + 1, head.y),
                "L" => Tuple::new(head.x - 1, head.y),
                "U" => Tuple::new(head.x, head.y + 1),
                "D" => Tuple::new(head.x, head.y - 1),
                _ => head,
            };
            tail = move_tail(&head, &tail);
            loc_set.insert(tail.clone());
        }
    }
    loc_set.len() as i32
}

#[allow(dead_code)]
fn print_vec_tup(x: i32, y: i32, t: &Vec<Tuple>) {
    let mut map = vec![vec![" "; x as usize]; y as usize];
    let k = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for val in (0..t.len()).rev() {
        map[(t[val].x + x / 2) as usize][(t[val].y + y / 2) as usize] = k[val].clone();
    }

    for j in (0..y).rev() {
        for i in 0..x {
            print!("{}", map[i as usize][j as usize]);
        }
        println!("");
    }
    println!("");
    println!("");

    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut user_input);

    println!("input {} ", user_input);
}

fn move_tail2(head: &Tuple, tail: &Tuple) -> Tuple {
    match (head, tail) {
        (h, t) if h.x == t.x - 1 && h.y == t.y + 2 => Tuple::new(t.x - 1, t.y + 1),
        (h, t) if h.x == t.x + 0 && h.y == t.y + 2 => Tuple::new(t.x + 0, t.y + 1),
        (h, t) if h.x == t.x + 1 && h.y == t.y + 2 => Tuple::new(t.x + 1, t.y + 1),

        (h, t) if h.x == t.x - 1 && h.y == t.y - 2 => Tuple::new(t.x - 1, t.y - 1),
        (h, t) if h.x == t.x + 0 && h.y == t.y - 2 => Tuple::new(t.x + 0, t.y - 1),
        (h, t) if h.x == t.x + 1 && h.y == t.y - 2 => Tuple::new(t.x + 1, t.y - 1),

        (h, t) if h.x == t.x + 2 && h.y == t.y - 1 => Tuple::new(t.x + 1, t.y - 1),
        (h, t) if h.x == t.x + 2 && h.y == t.y + 0 => Tuple::new(t.x + 1, t.y + 0),
        (h, t) if h.x == t.x + 2 && h.y == t.y + 1 => Tuple::new(t.x + 1, t.y + 1),

        (h, t) if h.x == t.x - 2 && h.y == t.y - 1 => Tuple::new(t.x - 1, t.y - 1),
        (h, t) if h.x == t.x - 2 && h.y == t.y + 0 => Tuple::new(t.x - 1, t.y + 0),
        (h, t) if h.x == t.x - 2 && h.y == t.y + 1 => Tuple::new(t.x - 1, t.y + 1),

        (h, t) if h.x == t.x + 2 && h.y == t.y + 2 => Tuple::new(t.x + 1, t.y + 1),
        (h, t) if h.x == t.x + 2 && h.y == t.y - 2 => Tuple::new(t.x + 1, t.y - 1),
        (h, t) if h.x == t.x - 2 && h.y == t.y + 2 => Tuple::new(t.x - 1, t.y + 1),
        (h, t) if h.x == t.x - 2 && h.y == t.y - 2 => Tuple::new(t.x - 1, t.y - 1),

        _ => tail.clone(),
    }
}

fn part2(input_raw: String) -> i32 {
    let input = input_raw.split("\n");
    let mut loc_set: HashSet<Tuple> = HashSet::new();
    let rope_length = 10;
    let mut rope = vec![Tuple::new(0, 0); rope_length];

    for command in input.into_iter() {
        if command.is_empty() {
            continue;
        }
        let args: Vec<&str> = command.split(" ").collect();
        let dir = args[0];
        let count: i32 = args[1].parse().unwrap();
        for _ in 0..count {
            // print_vec_tup(100, 100, &rope);
            rope[0] = match dir {
                "R" => Tuple::new(rope[0].x + 1, rope[0].y),
                "L" => Tuple::new(rope[0].x - 1, rope[0].y),
                "U" => Tuple::new(rope[0].x, rope[0].y + 1),
                "D" => Tuple::new(rope[0].x, rope[0].y - 1),
                _ => rope[0].clone(),
            };

            for i in 1..rope_length {
                rope[i] = move_tail2(&rope[i - 1], &rope[i])
            }
            loc_set.insert(rope[9].clone());
        }
    }

    loc_set.len() as i32
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
        assert_eq!(part1(get_input("input_test.txt")), 13);
    }

    #[test]
    fn test_part2_case_1() {
        assert_eq!(part2(get_input("input_test.txt")), 1);
    }
    #[test]
    fn test_part2_case_2() {
        assert_eq!(part2(get_input("input_test2.txt")), 36);
    }
}
