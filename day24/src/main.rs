use mylib::*;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");
    let start1 = Instant::now();
    let real_range = Range::new(200000000000000.0, 400000000000000.0);
    let part1_val = part1(input, &real_range);
    let time1 = start1.elapsed();
    println!("Part 1: {} in {:?}", part1_val, time1);
    let start2 = Instant::now();
    let part2_val = part2(input);
    let time2 = start2.elapsed();
    println!("Part 2: {} in {:?}", part2_val, time2);
}
