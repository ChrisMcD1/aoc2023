use std::time::Instant;
use mylib::*;

fn main() {
    let input = include_str!("../input.txt");
    let start1 = Instant::now();
    let part1_val = part1(input);
    let time1 = start1.elapsed();
    println!("Part 1: {} in {:?}", part1_val, time1);
    let start2 = Instant::now();
    let part2_val = part2(input, 1_000_000);
    let time2 = start2.elapsed();
    println!("Part 2: {} in {:?}", part2_val, time2);
}
