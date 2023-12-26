use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mylib::Range;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    let real_range = Range::new(200000000000000.0, 400000000000000.0);
    c.bench_function("part 1", |b| {
        b.iter(|| mylib::part1(black_box(input), &real_range))
    });
    // c.bench_function("part 2", |b| b.iter(|| mylib::part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
