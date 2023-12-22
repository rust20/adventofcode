use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    time::Instant,
};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum ModType {
    Broadcast,
    Flipflop,
    Conj,
    Untyped,
}

type ModRef<'a> = Rc<RefCell<Modules<'a>>>;

struct Modules<'a> {
    name: u8,
    mod_t: ModType,
    state: bool,
    inputs: Vec<bool>,
    dest: Vec<ModRef<'a>>,
    cycle: i64,
}

impl<'a> Modules<'a> {
    fn new(name: u8, mod_t: ModType) -> Rc<RefCell<Self>> {
        let m = Modules {
            dest: vec![],
            name,
            mod_t,
            state: false,
            inputs: vec![],
            cycle: -1,
        };
        Rc::new(RefCell::new(m))
    }
}

pub fn part1(inp: &str) -> i64 {
    let mut hs = HashMap::new();
    let mut names = HashMap::new();
    let mut idx = 0;

    for line in inp.lines() {
        let mut inp_split = line.split(" -> ");
        let mut module = inp_split.next().unwrap();
        let dest = inp_split.next().unwrap().split(", ").collect::<Vec<&str>>();
        let mod_t = if module == "broadcaster" {
            ModType::Broadcast
        } else if module.starts_with("%") {
            module = &module[1..];
            ModType::Flipflop
        } else {
            module = &module[1..];
            ModType::Conj
        };

        let m = Modules::new(idx, mod_t);

        names.insert(module, idx);
        hs.insert(idx, (m, dest));
        idx += 1;
    }

    let mut broadcaster = Modules::new(names["broadcaster"], ModType::Broadcast);
    let module_num = names.len();

    for (_, (module, _)) in hs.iter() {
        let mut m = module.as_ref().borrow_mut();
        if m.mod_t == ModType::Conj {
            m.inputs = vec![true; module_num];
        }
    }
    for (mod_name, (module, dests)) in hs.iter() {
        let mut m = module.as_ref().borrow_mut();
        for dest in dests {
            // println!("dest {}", dest);

            if let Some(id) = names.get(dest) {
                let d = &hs[id];
                m.dest.push(d.0.clone());

                let mut d_mod = d.0.as_ref().borrow_mut();
                if d_mod.mod_t == ModType::Conj {
                    d_mod.inputs[*mod_name as usize] = false;
                }
            } else {
                names.insert(dest, idx);
                m.dest.push(Modules::new(idx, ModType::Untyped));
                idx += 1;
            }
        }

        if m.borrow_mut().mod_t == ModType::Broadcast {
            broadcaster = module.clone();
        }
    }

    let mut queue = VecDeque::new();
    let mut a = [0, 0];

    for i in 0..1000 {
        queue.push_back((broadcaster.clone(), false, 255));

        while let Some((v, pulse, sender)) = queue.pop_front() {
            a[pulse as usize] += 1;
            let mut curr = v.as_ref().borrow_mut();

            // println!("{} -{}-> {}:{:?}", sender, if pulse {"high"} else {"low"}, curr.name, curr.mod_t);

            match curr.mod_t {
                ModType::Broadcast => {
                    send(&mut queue, &curr, pulse);
                }
                ModType::Flipflop => {
                    if !pulse {
                        curr.state = !curr.state;
                        send(&mut queue, &curr, curr.state);
                    }
                }
                ModType::Conj => {
                    curr.inputs[sender as usize] = pulse;
                    if curr.inputs.iter().all(|v| *v) {
                        if curr.cycle == 0 {
                            println!("{} {}", curr.name, i);
                            curr.cycle = i;
                        }
                        send(&mut queue, &curr, false);
                    } else {
                        send(&mut queue, &curr, true);
                    }
                }
                ModType::Untyped => {}
            }
        }
    }

    a[0] * a[1]
}

fn send<'a>(queue: &mut VecDeque<(ModRef<'a>, bool, u8)>, module: &Modules<'a>, pulse: bool) {
    for val in &module.dest {
        queue.push_back((val.clone(), pulse, module.name))
    }
    // module
    //     .dest
    //     .iter()
    //     .for_each(|val| queue.push_back((val.clone(), pulse, module.name)));
}

pub fn part2(inp: &str) -> i64 {
    let mut hs = HashMap::new();
    let mut names = HashMap::new();
    let mut inv_names = HashMap::new();
    let mut idx = 0;

    for line in inp.lines() {
        let mut inp_split = line.split(" -> ");
        let mut module = inp_split.next().unwrap();
        let dest = inp_split.next().unwrap().split(", ").collect::<Vec<&str>>();
        let mod_t = if module == "broadcaster" {
            ModType::Broadcast
        } else if module.starts_with("%") {
            module = &module[1..];
            ModType::Flipflop
        } else {
            module = &module[1..];
            ModType::Conj
        };

        let m = Modules::new(idx, mod_t);

        names.insert(module, idx);
        inv_names.insert(idx, module);
        hs.insert(idx, (m, dest));
        idx += 1;
    }

    let mut broadcaster = Modules::new(names["broadcaster"], ModType::Broadcast);
    let module_num = names.len();

    for (_, (module, _)) in hs.iter() {
        let mut m = module.as_ref().borrow_mut();
        if m.mod_t == ModType::Conj {
            m.inputs = vec![true; module_num];
        }
    }
    for (mod_name, (module, dests)) in hs.iter() {
        let mut m = module.as_ref().borrow_mut();
        for dest in dests {
            // println!("dest {}", dest);

            if let Some(id) = names.get(dest) {
                let d = &hs[id];
                m.dest.push(d.0.clone());

                let mut d_mod = d.0.as_ref().borrow_mut();
                if d_mod.mod_t == ModType::Conj {
                    d_mod.inputs[*mod_name as usize] = false;
                }
            } else {
                names.insert(dest, idx);
                inv_names.insert(idx, dest);
                m.dest.push(Modules::new(idx, ModType::Untyped));
                idx += 1;
            }
        }

        if m.borrow_mut().mod_t == ModType::Broadcast {
            broadcaster = module.clone();
        }
    }

    let mut queue = VecDeque::new();
    let mut a = [0, 0];
    let target = names["rx"];

    let mut not_have_cyles = hs.iter().filter(|(v, s)| s.0.as_ref().borrow_mut().mod_t == ModType::Conj).map(|v|*v.0).collect::<Vec<u8>>();
    let mut prods = vec![];

    for i in 0..1_000_000_000{
        queue.push_back((broadcaster.clone(), false, 255));

        while let Some((v, pulse, sender)) = queue.pop_front() {

            let mut curr = v.as_ref().borrow_mut();

            if curr.name == target && !pulse {
                println!("yay {}", i);
                return i
            }

            match curr.mod_t {
                ModType::Broadcast => {
                    send(&mut queue, &curr, pulse);
                }
                ModType::Flipflop => {
                    if !pulse {
                        curr.state = !curr.state;
                        send(&mut queue, &curr, curr.state);
                    }
                }
                ModType::Conj => {
                    curr.inputs[sender as usize] = pulse;
                    if curr.inputs.iter().all(|v| *v) {
                        // println!("{} {}", [curr.name], i);
                        if curr.cycle < 0 {
                            println!("{} {}", inv_names[&curr.name], i);
                            curr.cycle = i;

                            let pos  = not_have_cyles.iter().position(|&v| v == curr.name).unwrap();
                            not_have_cyles.remove(pos);
                            prods.push(i + 1);
                            println!("{:?} ", not_have_cyles);

                            if not_have_cyles.len() == 1 {
                                return prods.iter().product()
                            }
                        }
                        send(&mut queue, &curr, false);
                    } else {
                        send(&mut queue, &curr, true);
                    }
                }
                ModType::Untyped => {}
            }
        }
    }

    a[0] * a[1]
}

#[allow(dead_code)]
fn main() {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
        "input3.txt",
    ];

    for fi in inputs {
        let path = "tests/d20/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        let start1 = Instant::now();
        println!("part1 {}", part1(&reader));
        let start2 = Instant::now();
        println!("part2 {}", part2(&reader));
        let finish = start2.elapsed();

        println!("time1 {:?}, time2 {:?}", start2 - start1, finish);
    }
}
