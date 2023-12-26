use std::{convert::Infallible, str::FromStr, time::Instant, vec};

fn main() {
    let input = include_str!("../input.txt");
    let start1 = Instant::now();
    let part1_val = part1(input);
    let time1 = start1.elapsed();
    println!("Part 1: {} in {:?}", part1_val, time1);
    let start2 = Instant::now();
    let part2_val = part2(input);
    let time2 = start2.elapsed();
    println!("Part 2: {} in {:?}", part2_val, time2);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Mapping {
    destination_start: i64,
    source_start: i64,
    tunnel_length: i64,
}

impl FromStr for Mapping {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.trim().split(" ").collect();
        Ok(Mapping {
            destination_start: splits[0].parse().unwrap(),
            source_start: splits[1].parse().unwrap(),
            tunnel_length: splits[2].parse().unwrap(),
        })
    }
}

impl Mapping {
    #[inline]
    fn source_end(&self) -> i64 {
        self.source_start + self.tunnel_length - 1
    }

    fn jump_distance(&self) -> i64 {
        self.destination_start - self.source_start
    }

    fn get_destination_if_in_range(&self, i: i64) -> Option<i64> {
        let max = self.source_start + self.tunnel_length - 1;
        if i < self.source_start || i > max {
            None
        } else {
            Some(i - self.source_start + self.destination_start)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MappingBlock {
    maps: Vec<Mapping>,
}

impl MappingBlock {
    fn get_destination(&self, source: i64) -> i64 {
        self.maps
            .iter()
            .find_map(|map| map.get_destination_if_in_range(source))
            .unwrap_or(source)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SeedRange {
    start: i64,
    end: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SplitSeedRange {
    originals: Vec<SeedRange>,
    splits: Vec<SeedRange>,
}

impl SplitSeedRange {
    fn combine(self, other: Self) -> Self {
        Self {
            originals: [self.originals, other.originals].concat(),
            splits: [self.splits, other.splits].concat(),
        }
    }
}

impl SeedRange {
    fn new(start: i64, length: i64) -> Self {
        Self {
            start,
            end: start + length - 1,
        }
    }
    fn from_start_end(start: i64, end: i64) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    fn split_from_mapping(self, mapping: &Mapping) -> SplitSeedRange {
        if (self.end < mapping.source_start) || (self.start > mapping.source_end()) {
            SplitSeedRange {
                originals: vec![self],
                splits: vec![],
            }
        } else {
            // Lower is all of the items on the bottom that do not get mapped
            let lower_start = self.start;
            let lower_end = mapping.source_start - 1;
            let lower = SeedRange::from_start_end(lower_start, lower_end);

            // Middle is all of the elements getting mapped
            let middle_start = self.start.max(mapping.source_start);
            let middle_end = self.end.min(mapping.source_end());
            let middle = SeedRange::from_start_end(
                middle_start + mapping.jump_distance(),
                middle_end + mapping.jump_distance(),
            );

            // Upper is all of the items on the top that do not get mapped
            let upper_start = mapping.source_end() + 1;
            let upper_end = self.end;
            let upper = SeedRange::from_start_end(upper_start, upper_end);

            let split = SplitSeedRange {
                originals: vec![lower, upper].into_iter().flatten().collect(),
                splits: vec![middle].into_iter().flatten().collect(),
            };

            split
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SeedRangesInput {
    seed_ranges: SeedRanges,
    mappings: Mappings,
}

impl From<SeedsInput> for SeedRangesInput {
    fn from(value: SeedsInput) -> Self {
        let seed_ranges: SeedRanges = SeedRanges(
            value
                .seeds
                .chunks_exact(2)
                .map(|slice| SeedRange::new(slice[0], slice[1]))
                .collect(),
        );
        Self {
            seed_ranges,
            mappings: value.mappings,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SeedRanges(Vec<SeedRange>);
impl SeedRanges {
    fn split_all_mapping_block(self, mapping_block: &MappingBlock) -> Self {
        let ranges: Vec<SeedRange> = self
            .0
            .into_iter()
            .map(|range| {
                mapping_block.maps.iter().fold(
                    SplitSeedRange {
                        originals: vec![range],
                        splits: vec![],
                    },
                    |split_range, mapping| {
                        let split_ranges: Vec<SplitSeedRange> = split_range
                            .originals
                            .into_iter()
                            .map(|range| range.split_from_mapping(mapping))
                            .collect();

                        let split_range: SplitSeedRange = split_ranges.into_iter().fold(
                            SplitSeedRange {
                                originals: vec![],
                                splits: split_range.splits,
                            },
                            |acc, r| acc.combine(r),
                        );
                        split_range
                    },
                )
            })
            .map(|split_range| [split_range.originals, split_range.splits].concat())
            .flatten()
            .collect();
        SeedRanges(ranges)
    }
    fn lowest_value(&self) -> i64 {
        self.0
            .iter()
            .map(|range| range.start)
            .min()
            .expect("Don't call lowest value with nothing!")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Mappings(Vec<MappingBlock>);
impl Mappings {
    fn get_location(&self, seed: i64) -> i64 {
        self.0
            .iter()
            .fold(seed, |i, mapping_block| mapping_block.get_destination(i))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SeedsInput {
    seeds: Vec<i64>,
    // We need to get through every mapping to get to the end
    mappings: Mappings,
}

impl FromStr for SeedsInput {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grouped_lines = s.split("\n\n");
        let seeds_line = grouped_lines.next().unwrap();
        let seeds: Vec<i64> = seeds_line.split(" ").collect::<Vec<&str>>()[1..]
            .iter()
            .map(|w| w.trim().parse().unwrap())
            .collect();
        let mappings: Vec<MappingBlock> = grouped_lines
            .map(|group| MappingBlock {
                maps: group.lines().collect::<Vec<&str>>()[1..]
                    .iter()
                    .map(|line| line.parse().unwrap())
                    .collect(),
            })
            .collect();
        Ok(SeedsInput {
            seeds,
            mappings: Mappings(mappings),
        })
    }
}

fn part1(s: &str) -> i64 {
    let input: SeedsInput = s.parse().unwrap();
    input
        .seeds
        .iter()
        .map(|seed| input.mappings.get_location(*seed))
        .min()
        .expect("Don't call me without seeds")
}

fn part2(s: &str) -> i64 {
    let input: SeedRangesInput = s.parse::<SeedsInput>().unwrap().into();
    let final_seed_ranges =
        input
            .mappings
            .0
            .iter()
            .fold(input.seed_ranges, |ranges, mapping_block| {
                let new_ranges = ranges.split_all_mapping_block(mapping_block);
                new_ranges
            });
    final_seed_ranges.lowest_value()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const SAMPLE_INPUT: &str = r##"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"##;

    #[test]
    fn parse_mapping() {
        let input = "0 15 37";
        let expected_mapping = Mapping {
            destination_start: 0,
            source_start: 15,
            tunnel_length: 37,
        };

        let actual_mapping: Mapping = input.parse().unwrap();

        assert_eq!(expected_mapping, actual_mapping);
    }

    #[test]
    fn mapping() {
        let mapping = Mapping {
            destination_start: 50,
            source_start: 98,
            tunnel_length: 2,
        };
        let input = 99;
        let expected = 51;

        let val = mapping.get_destination_if_in_range(input);

        assert_eq!(Some(expected), val)
    }

    #[test]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 35)
    }

    #[test]
    fn test_split_lower_overlap() {
        let ranges = SeedRanges(vec![SeedRange { start: 0, end: 10 }]);
        let block = MappingBlock {
            maps: vec![Mapping {
                source_start: -1,
                destination_start: 1,
                tunnel_length: 4,
            }],
        };
        let expected_ranges = SeedRanges(vec![
            SeedRange { start: 2, end: 4 },
            SeedRange { start: 3, end: 10 },
        ]);

        let actual_ranges = ranges.split_all_mapping_block(&block);

        assert!(ranges_equal(expected_ranges, actual_ranges));
    }

    #[test]
    fn test_split_no_overlap() {
        let ranges = SeedRanges(vec![SeedRange { start: 0, end: 10 }]);
        let block = MappingBlock {
            maps: vec![Mapping {
                source_start: 11,
                destination_start: 1,
                tunnel_length: 4,
            }],
        };
        let expected_ranges = ranges.clone();

        let actual_ranges = ranges.split_all_mapping_block(&block);

        assert!(ranges_equal(expected_ranges, actual_ranges));
    }

    #[test]
    fn test_split_real() {
        let ranges = SeedRanges(vec![SeedRange { start: 74, end: 87 }]);
        let block = MappingBlock {
            maps: vec![Mapping {
                source_start: 77,
                destination_start: 45,
                tunnel_length: 23,
            }],
        };
        let expected_ranges = SeedRanges(vec![
            SeedRange { start: 74, end: 76 },
            SeedRange { start: 45, end: 55 },
        ]);

        let actual_ranges = ranges.split_all_mapping_block(&block);

        assert!(ranges_equal(expected_ranges, actual_ranges));
    }

    #[test]
    fn test_split_one_overlap() {
        let ranges = SeedRanges(vec![SeedRange { start: 0, end: 10 }]);
        let block = MappingBlock {
            maps: vec![Mapping {
                source_start: 10,
                destination_start: 1,
                tunnel_length: 4,
            }],
        };
        let expected_ranges = SeedRanges(vec![
            SeedRange { start: 0, end: 9 },
            SeedRange { start: 1, end: 1 },
        ]);

        let actual_ranges = ranges.split_all_mapping_block(&block);

        assert!(ranges_equal(expected_ranges, actual_ranges));
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT), 46)
    }

    #[test]
    fn test_given_2_fast() {
        assert_eq!(part2(SAMPLE_INPUT), 46)
    }

    fn ranges_equal(expected: SeedRanges, actual: SeedRanges) -> bool {
        let expected_set: HashSet<SeedRange> = expected.0.into_iter().collect();
        let actual_set: HashSet<SeedRange> = actual.0.into_iter().collect();
        expected_set == actual_set
    }
}
