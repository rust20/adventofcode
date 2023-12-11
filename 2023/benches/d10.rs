use aoc::bin;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_d11(c: &mut Criterion) {
    #[rustfmt::skip]
    let inputs = vec![
        "input1.txt",
        "input2.txt",
    ];

    for fi in inputs {
        let path = "tests/d11/".to_string();
        let reader = std::fs::read_to_string(path + fi).expect("read input");

        c.bench_function("d11 part 1", |b| b.iter(|| bin::d11::part1(&reader)));
        c.bench_function("d11 part 2", |b| b.iter(|| bin::d11::part2(&reader)));
    }

}

criterion_group!(benches, bench_d11);
criterion_main!(benches);
