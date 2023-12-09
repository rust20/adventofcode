use std::collections::HashMap;

fn part1(inp: &str) {
    let mut sum: i32 = 0;

    let hs: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for line in inp.lines()  {
        let fsc:Vec<&str> = line.split(": ").collect();
        let k = fsc[0];
        let t = fsc[1];

        let n: i32 = k.trim_start_matches("Game ").parse().unwrap();

        let mut curr_game = true;

        for game in t.split("; ") {
            let bs = game.split(", ").map(|x| {x.split(" ")});
            for g in bs {
                let a: Vec<&str> = g.collect();

                let x = a.first().unwrap().parse::<i32>().unwrap();
                let y = a.last().unwrap();

                if hs[y] < x {
                    curr_game = false;
                }
            }
        }
        if curr_game {
            sum += n
        }
    }

    println!("{}", sum)
}

fn part2(inp: &str) {
    let mut sum: i32 = 0;

    for line in inp.lines()  {
        let mut hs: HashMap<&str, i32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let fsc:Vec<&str> = line.split(": ").collect();
        let t = fsc[1];

        for game in t.split("; ") {
            let bs = game.split(", ").map(|x| {x.split(" ")});
            for g in bs {
                let a: Vec<&str> = g.collect();

                let x = a.first().unwrap().parse::<i32>().unwrap();
                let y = a.last().unwrap();

                if hs[y] < x {
                    hs.insert(y, x);
                }
            }
        }
        sum += hs["green"] * hs["blue"] * hs["red"];
    }

    println!("{}", sum)

}

fn main() {
    let file_input = "tests/d2/input2.txt";
    let reader = std::fs::read_to_string(file_input).expect("read input");
    
    part1(&reader);
    part2(&reader);
}

// vim: set ts=4 sts=4 sw=4 et:
