use std::borrow::BorrowMut;
// use regex::Regex;
use std::cell::{RefCell, RefMut};
// use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;

type Node = Rc<RefCell<Box<FileType>>>;

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    children: Vec<Node>,
}

#[derive(Clone, Debug)]
enum FileType {
    Directory(Directory),
    File(String, u32),
}

struct TreeNode {
    val: FileType,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(ft: FileType) -> Self {
        Self {
            val: ft,
            children: Vec::new(),
        }
    }
}

const LIMIT: u32 = 100000;

fn traverse_limit(node: Node) -> u32 {
    match *node.as_ref().borrow().clone() {
        FileType::Directory(d) => d.children.iter().fold(0, |acc, val| {
            let res = traverse_limit(val.clone());
            acc + (if res <= LIMIT { res } else { 0 })
        }),
        FileType::File(_, f) => {
            if f <= LIMIT {
                f
            } else {
                0
            }
        }
    }
}

fn get_input() -> String {
    let mut data = String::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn part1(input_raw: String) -> u32 {
    let input = input_raw.split("\n");

    let mut command_iter = input.into_iter();

    let mut stack = Vec::new();

    let root_dir = FileType::Directory(Directory {
        name: "/".to_string(),
        children: Vec::new(),
    });
    let root = Rc::new(RefCell::new(Box::new(root_dir)));
    let mut curr = Rc::clone(&root);

    while let Some(i) = command_iter.next() {
        println!("{:?}", i);
        let args: Vec<&str> = i.split(" ").collect();

        if args[0] != "$" {
            let filename = args[1].to_string();
            let filename_ = filename.clone();
            if args[0] == "dir" {
                if let FileType::Directory(mut dir) = *curr.as_ref().borrow_mut().clone() {
                    let ft = FileType::Directory(Directory {
                        name: filename,
                        children: Vec::new(),
                    });
                    dir.children.push(Rc::new(RefCell::new(Box::new(ft))));
                    println!("push dir {} {:?}", filename_, dir.children);
                }
                continue;
            }

            let filesize = args[0].parse::<u32>().unwrap();

            if let FileType::Directory(mut dir) = *curr.as_ref().borrow_mut().clone() {
                let ft = FileType::File(filename, filesize);
                dir.children.push(Rc::new(RefCell::new(Box::new(ft))));
                println!("push file {} {:?}", filename_, dir.children);
            }
            continue;
        }

        if args[0] == "$" && args[1] == "ls" {
            continue;
        }

        if args[0] == "$" && args[1] == "cd" {
            if args[2] == "/" {
                continue;
            }
            if args[2] == ".." {
                curr = stack.pop().unwrap();
                continue;
            }
            stack.push(Rc::clone(&curr));
            let mut a = Rc::clone(&curr);
            if let FileType::Directory(dir) = *curr.as_ref().borrow().clone() {
                let childs = dir.children;
                println!("{:?}", childs);
                let tmp = childs
                    .iter()
                    .find(|x| {
                        if let FileType::Directory(d) = *x.as_ref().borrow().clone() {
                            d.name == args[2].to_string()
                        } else {
                            false
                        }
                    })
                    .unwrap();
                a = Rc::clone(&tmp);
            }
            curr = a;
            continue;
        }
    }

    traverse_limit(root)
}

fn part2(input_raw: String) -> i32 {
    let input = input_raw.chars();
    let res = input.count();
    res as i32
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
