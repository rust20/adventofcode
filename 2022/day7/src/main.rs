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

fn traverse_limit(node: Node, sum: u32) -> (u32, u32) {
    let tree_node = node.as_ref().borrow();
    match tree_node.val.clone() {
        FileType::Directory(name) => {
            let curr_dir = tree_node.children.iter().fold((0, sum), |acc, val| {
                let res = traverse_limit(val.clone(), acc.1);
                (acc.0 + res.0, res.1)
            });
            if name == "/" {
                return curr_dir;
            }
            (
                curr_dir.0,
                curr_dir.1 + (if curr_dir.0 <= LIMIT { curr_dir.0 } else { 0 }),
            )
        }
        FileType::File(_, f) => (f, sum),
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

    // let mut traverse = |node: Node| {
    //     let tree_node = node.as_ref().borrow();
    //     println!("0");
    //     match tree_node.val.clone() {
    //         FileType::Directory(name) => {
    //             let curr_sum = tree_node.children.iter().fold(0, |acc, val| {
    //                 let res = traverse(Rc::clone(val));
    //                 println!("{}", acc);
    //                 acc + (res)
    //             });
    //             println!("> {} {}", name.clone(), curr_sum);
    //             if curr_sum <= LIMIT {
    //                 sum += curr_sum;
    //             }
    //             curr_sum
    //         }
    //         FileType::File(name, f) => {
    //             println!("visited {} {}", name.clone(), f);
    //             if f <= LIMIT {
    //                 f
    //             } else {
    //                 0
    //             }
    //         }
    //     }
    // };

    let mut stack = Vec::new();
    let root = TreeNode::new(FileType::Directory("/".to_string()));

    let mut curr = root.clone();
    while let Some(i) = command_iter.next() {
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
        stack.push(curr.clone());

        let curr_cloned = curr.clone();
        let curr_borrowed = curr_cloned.borrow();
        let result = curr_borrowed
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

        curr = result.clone();

        continue;
    }

    traverse_limit(root, 0).1
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
