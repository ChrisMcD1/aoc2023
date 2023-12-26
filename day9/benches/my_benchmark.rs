use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mylib::History;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    let parsed: Vec<History> = input.lines().map(|l| l.parse().unwrap()).collect();
    c.bench_function("part 1", |b| b.iter(|| mylib::part1(black_box(input))));
    c.bench_function("part 2", |b| b.iter(|| mylib::part2(black_box(input))));
    c.bench_function("parsing the input", |b| {
        b.iter(|| {
            let _vec: Vec<History> = input
                .lines()
                .map(|l| l.parse().unwrap())
                .collect();
        })
    });
    c.bench_function("running the histories", |b| {
        b.iter(|| {
            parsed.iter().map(|h| h.next_value()).sum::<i64>()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
