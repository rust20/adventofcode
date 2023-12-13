use aoc::bin;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_d12(c: &mut Criterion) {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
    ];
    for fi in inputs {
        let path = "tests/d12/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");
        c.bench_function("d12 part1", |b| b.iter(|| bin::d12::part1(&reader)));
        c.bench_function("d12 part2", |b| b.iter(|| bin::d12::part2(&reader)));
    }
}

criterion_group!(benches, bench_d12);
criterion_main!(benches);
