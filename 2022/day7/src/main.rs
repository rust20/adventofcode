use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;

#[derive(Clone, Debug)]
enum FileType {
    Directory(String),
    File(String, u32),
}

type Node = Rc<RefCell<TreeNode>>;

#[derive(Clone, Debug)]
struct TreeNode {
    val: FileType,
    children: Vec<Node>,
}

impl TreeNode {
    pub fn new(ft: FileType) -> Node {
        let new = TreeNode {
            val: ft,
            children: Vec::new(),
        };
        Rc::new(RefCell::new(new))
    }
}

const LIMIT: u32 = 100000;

fn traverse_limit(node: Node) -> u32 {
    let tree_node = node.as_ref().borrow();
    match tree_node.val {
        FileType::Directory(_) => tree_node.children.iter().fold(0, |acc, val| {
            let res = traverse_limit(Rc::clone(val));
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

    let mut sum = 0;

    let traverse = |node| {
        let tree_node = node.as_ref().borrow();
        match tree_node.val {
            FileType::Directory(_) => tree_node.children.iter().fold(0, |acc, val| {
                let res = traverse_limit(Rc::clone(val));
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
    };

    let root = TreeNode::new(FileType::Directory("/".to_string()));

    let mut curr = Rc::clone(&root);
    while let Some(i) = command_iter.next() {
        println!("{:?} {:?}", i, 0);
        if i == "" {
            break;
        }
        let args: Vec<&str> = i.split(" ").collect();

        if args[0] != "$" {
            let filename = args[1].to_string();
            if args[0] == "dir" {
                curr.as_ref()
                    .borrow_mut()
                    .children
                    .push(TreeNode::new(FileType::Directory(filename)));
                continue;
            }

            let filesize = args[0].parse::<u32>().unwrap();

            curr.as_ref()
                .borrow_mut()
                .children
                .push(TreeNode::new(FileType::File(filename, filesize)));
            continue;
        }

        if args[1] == "ls" {
            continue;
        }

        if !(args[1] == "cd") {
            continue;
        }
        if args[2] == "/" {
            continue;
        }
        if args[2] == ".." {
            curr = stack.pop().unwrap();
            continue;
        }
        stack.push(Rc::clone(&curr));

        let curr_cloned = Rc::clone(&curr);
        let curr_borrowed = curr_cloned.borrow();
        let result2 = curr_borrowed
            .children
            .iter()
            .find(|x| {
                if let FileType::Directory(dir) = x.as_ref().borrow_mut().val.clone() {
                    dir == args[2]
                } else {
                    false
                }
            })
            .unwrap();

        curr = Rc::clone(result2);

        continue;
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
