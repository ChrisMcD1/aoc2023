use std::{collections::HashMap, convert::Infallible, str::FromStr, time::Instant};

fn main() {
    let input = include_str!("../input.txt");
    let start1 = Instant::now();
    let part1_val = part1(input);
    let time1 = start1.elapsed();
    println!("Part 1: {} in {:?}", part1_val, time1);

    const SAMPLE_INPUT_2: &str = r##"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"##;
    let start2 = Instant::now();
    let part2_val = part2(input);
    let time2 = start2.elapsed();
    println!("Part 2: {} in {:?}", part2_val, time2);
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::Right,
            "L" => Self::Left,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
struct Directions {
    backing: Vec<Direction>,
    current_index: usize,
}

impl FromStr for Directions {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Directions {
            backing: s
                .chars()
                .map(|c| c.to_string())
                .map(|s| s.parse().unwrap())
                .collect(),
            current_index: 0,
        })
    }
}

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.backing.len() == self.current_index {
            self.current_index = 0;
        };
        let result = self.backing.get(self.current_index).unwrap();
        self.current_index += 1;
        Some(result.clone())
    }
}

#[derive(Debug, Clone)]
struct Maps(HashMap<String, (String, String)>);

impl FromStr for Maps {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Maps(
            s.lines()
                .map(|line| {
                    let (node_name, other) = line.split_once(" = ").unwrap();
                    let (left, right) = other
                        .trim_matches(|c| c == '(' || c == ')')
                        .split_once(",")
                        .unwrap();
                    (
                        node_name.trim().to_string(),
                        (left.trim().to_string(), right.trim().to_string()),
                    )
                })
                .collect(),
        ))
    }
}

impl Maps {
    fn get_next(&self, node: &str, direction: &Direction) -> String {
        let (left, right) = self.0.get(node).unwrap();
        match direction {
            Direction::Left => left,
            Direction::Right => right,
        }
        .to_string()
    }
}

fn part1(s: &str) -> i64 {
    let (directions_str, maps_str) = s.split_once("\n\n").unwrap();
    let directions: Directions = directions_str.parse().unwrap();
    let maps: Maps = maps_str.parse().unwrap();
    // print!("parsed {directions:?}, maps: {maps:?}");
    let mut count = 0;
    let mut current_node = "AAA".to_string();
    for direction in directions {
        if count % 100_000 == 0 {
            println!("at {count}");
        }
        current_node = maps.get_next(&current_node, &direction);
        count += 1;
        if current_node == "ZZZ" {
            break;
        }
    }
    count
}

fn part2(s: &str) -> i64 {
    let (directions_str, maps_str) = s.split_once("\n\n").unwrap();
    let directions: Directions = directions_str.parse().unwrap();
    let maps: Maps = maps_str.parse().unwrap();
    let mut current_nodes: Vec<String> = maps
        .0
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.to_string())
        .collect();
    print!("parsed {directions:?}, maps: {maps:?}");
    print!("Current nodes are {current_nodes:?}");
    // for starting_node in current_nodes {
    //     let mut count = 0;
    //     let mut moving_node = starting_node.clone();
    //     for direction in directions.clone() {
    //         moving_node = maps.get_next(&moving_node, &direction);
    //         count += 1;
    //         if moving_node.ends_with("Z") {
    //             break;
    //         }
    //     }
    //     println!("Got to a Z end on {starting_node} in {count}");
    // }
    let mut count = 0;
    for direction in directions {
        if count % 100_000 == 0 {
            println!("at {count}");
        }
        current_nodes = current_nodes
            .iter()
            .map(|node| maps.get_next(node, &direction))
            .collect();
        // print!("Current nodes are {current_nodes:?}");
        count += 1;
        if current_nodes.iter().all(|n| n.ends_with("Z")) {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r##"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"##;

    const SAMPLE_INPUT_2: &str = r##"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"##;

    const SAMPLE_INPUT_3: &str = r##"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"##;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 2)
    }

    #[test]
    fn test_full_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 15517)
    }

    #[test]
    fn test_given_1_2() {
        assert_eq!(part1(SAMPLE_INPUT_3), 6)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT_2), 6)
    }
}
