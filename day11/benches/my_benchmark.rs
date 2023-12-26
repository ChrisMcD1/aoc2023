use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mylib::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("part 1", |b| b.iter(|| mylib::part1(black_box(input))));
    c.bench_function("part 2", |b| b.iter(|| mylib::part2(black_box(input), 1_000_000)));
    c.bench_function("parse mega", |b| {
        b.iter(|| {
            let raw_image = black_box(input).parse::<RawImage>().unwrap();
            let _mega_image: MegaExpandedImage = MegaExpandedImage::new(raw_image, 1_000_000);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
