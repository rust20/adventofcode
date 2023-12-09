fn part1(inp: &str) {
    let mut ht = [0;256];
    ht[b'0' as usize] = 0;
    ht[b'1' as usize] = 1;
    ht[b'2' as usize] = 2;
    ht[b'3' as usize] = 3;
    ht[b'4' as usize] = 4;
    ht[b'5' as usize] = 5;
    ht[b'6' as usize] = 6;
    ht[b'7' as usize] = 7;
    ht[b'8' as usize] = 8;
    ht[b'9' as usize] = 9;

    let lines = inp.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let filtered = line.chars().into_iter().filter(|x| { x.is_digit(10) });
        let collected:  Vec<char> = filtered.collect();
        let first = collected.first().unwrap().clone();
        let last = collected.last().unwrap().clone();

        sum += ht[first as usize] * 10 + ht[last as usize];
    }
    println!("part1: {}", sum);
}

fn part2(inp: &str) {
    let mut sum: u32 = 0;
    for line in inp.lines() {
        let mut digits = Vec::new();
        for i in 0..line.len() {
            let curr = &line[i..];
            if curr.starts_with("0") || curr.starts_with("zero") {digits.push(0)}
            else if curr.starts_with("1") || curr.starts_with("one") {digits.push(1)}
            else if curr.starts_with("2") || curr.starts_with("two") {digits.push(2)}
            else if curr.starts_with("3") || curr.starts_with("three") {digits.push(3)}
            else if curr.starts_with("4") || curr.starts_with("four") {digits.push(4)}
            else if curr.starts_with("5") || curr.starts_with("five") {digits.push(5)}
            else if curr.starts_with("6") || curr.starts_with("six") {digits.push(6)}
            else if curr.starts_with("7") || curr.starts_with("seven") {digits.push(7)}
            else if curr.starts_with("8") || curr.starts_with("eight") {digits.push(8)}
            else if curr.starts_with("9") || curr.starts_with("nine") {digits.push(9)}
        }

        sum += digits.first().unwrap() * 10 + digits.last().unwrap();

    }
    println!("part2: {}", sum);
}

fn main() {
    let file_input = "tests/d1/input1.txt";
    let reader = std::fs::read_to_string(file_input).expect("read input");
    
    part1(&reader);
    part2(&reader);
}
