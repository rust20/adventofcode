use std::cmp::min;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

fn get_input(file_name: &str) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    OpenBracket,
    CloseBracket,
    Number(i32),
}

fn parse(list: String) -> Vec<Token> {
    let mut curr_buffer: String = "".to_string();
    let mut stack = Vec::new();

    for c in list.chars() {
        match c {
            '0'..='9' => {
                curr_buffer = curr_buffer + &c.to_string();
            }
            '[' => stack.push(Token::OpenBracket),
            ']' => {
                if !curr_buffer.is_empty() {
                    stack.push(Token::Number(curr_buffer.parse().unwrap()));
                    curr_buffer = "".to_string();
                }
                stack.push(Token::CloseBracket)
            }
            ',' => {
                if !curr_buffer.is_empty() {
                    stack.push(Token::Number(curr_buffer.parse().unwrap()));
                    curr_buffer = "".to_string();
                }
            }
            _ => {}
        }
    }
    stack
}

fn build_tree(thing: &Vec<Token>) -> Packet {
    let mut stack = Vec::new();
    let mut head = Packet::List(Vec::new());

    for token in thing.iter() {
        match token {
            Token::OpenBracket => {
                stack.push(head);
                head = Packet::List(Vec::new());
            }
            Token::CloseBracket => {
                let tmp = head;
                head = stack.pop().unwrap();
                if let Packet::List(ref mut curr_list) = head {
                    curr_list.push(tmp);
                }
            }
            Token::Number(val) => {
                if let Packet::List(ref mut curr_list) = head {
                    curr_list.push(Packet::Num(*val));
                }
            }
        }
    }

    let mut a = match head {
        Packet::List(res) => res,
        Packet::Num(_) => Vec::new(),
    };
    a.pop().unwrap()
}

fn compare(ref left: &Packet, ref right: &Packet) -> i32 {
    match (&left, &right) {
        (Packet::Num(val_left), Packet::Num(val_right)) => {
            if val_left > val_right {
                0
            } else if val_left < val_right {
                1
            } else {
                2
            }
        }
        (Packet::Num(left_val), Packet::List(_)) => {
            compare(&Packet::List(vec![Packet::Num(*left_val)]), right)
        }
        (Packet::List(_), Packet::Num(right_val)) => {
            compare(left, &Packet::List(vec![Packet::Num(*right_val)]))
        }
        (Packet::List(val_left), Packet::List(val_right)) => {
            let (lval, rval) = (val_left.len(), val_right.len());
            for i in 0..(min(lval, rval)) {
                let res = compare(&val_left[i], &val_right[i]);
                if res < 2 {
                    return res;
                }
            }

            return if lval == rval {
                2
            } else if lval < rval {
                1
            } else {
                0
            };
        }
    }
}

fn part1(input_raw: String) -> i32 {
    let input: Vec<&str> = input_raw
        .split_terminator("\n")
        .filter(|x| !x.is_empty())
        .collect();
    let mut sum = 0;

    for i in 0..(input.len() / 2) {
        let idx = i * 2;
        let l_token = parse(input[idx].to_string());
        let r_token = parse(input[idx + 1].to_string());

        let l_tree = build_tree(&l_token);
        let r_tree = build_tree(&r_token);

        if compare(&l_tree, &r_tree) == 1 {
            sum += i + 1;
        }
    }

    sum as i32
}

fn part2(input_raw: String) -> i32 {
    let mut input: Vec<Vec<Token>> = input_raw
        .split_terminator("\n")
        .filter(|x| !x.is_empty())
        .map(|x| parse(x.to_string()))
        .collect();

    let first = vec![
        Token::OpenBracket,
        Token::OpenBracket,
        Token::Number(2),
        Token::CloseBracket,
        Token::CloseBracket,
    ];
    let second = vec![
        Token::OpenBracket,
        Token::OpenBracket,
        Token::Number(6),
        Token::CloseBracket,
        Token::CloseBracket,
    ];

    input.push(first.clone());
    input.push(second.clone());

    input.sort_by(|a, b| {
        let x = build_tree(a);
        let y = build_tree(b);
        let res = compare(&x, &y);
        match res {
            0 => std::cmp::Ordering::Greater,
            1 => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    });

    let mut res = 1;

    for i in 0..input.len() {
        if do_vecs_match(&first, &input[i]) {
            res *= i+1
        }
    }

    for i in 0..input.len() {
        if do_vecs_match(&second, &input[i]) {
            res *= i+1
        }
    }

    res as i32
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn main() {
    let before = Instant::now();
    let input_raw = get_input("tests/d13/input1.txt");

    let res1 = part1(input_raw.clone());
    let res2 = part2(input_raw);


    println!("{} {}", res1, res2);
    println!("Elapsed time: {:.2?}", before.elapsed());
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
        assert_eq!(part2(get_input("input_test.txt")), 140);
    }
}
