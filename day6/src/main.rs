use std::{convert::Infallible, str::FromStr, time::Instant};

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

struct RaceRecord {
    total_time: i64,
    record_distance: i64,
}

impl RaceRecord {
    fn compute_total_distance(&self, holding_time: i64) -> i64 {
        let speed = holding_time;
        (self.total_time - holding_time) * speed
    }
    fn find_all_winning_amounts(&self) -> Vec<i64> {
        (0..self.total_time)
            .filter_map(|holding_time| {
                (self.compute_total_distance(holding_time) > self.record_distance)
                    .then_some(holding_time)
            })
            .collect()
    }
}

struct RaceRecords(Vec<RaceRecord>);

impl FromStr for RaceRecords {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let times = &lines[0].split_whitespace().collect::<Vec<&str>>()[1..];
        let distances = &lines[1].split_whitespace().collect::<Vec<&str>>()[1..];
        let races_vec = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| RaceRecord {
                total_time: time.parse().unwrap(),
                record_distance: distance.parse().unwrap(),
            });
        Ok(RaceRecords(races_vec.collect()))
    }
}

fn part1(s: &str) -> i64 {
    let races: RaceRecords = s.parse().unwrap();
    races
        .0
        .iter()
        .map(|race| {
            let amount: i64 = race.find_all_winning_amounts().len().try_into().unwrap();
            amount
        })
        .product()
}

fn part2(s: &str) -> i64 {
    let lines: Vec<&str> = s.lines().collect();
    let times: String = lines[0].chars().filter(|c| !c.is_whitespace()).collect();
    let distances: String = lines[1].chars().filter(|c| !c.is_whitespace()).collect();
    let (_, time) = times.split_once(":").unwrap();
    let (_, distance) = distances.split_once(":").unwrap();

    let race: RaceRecord = RaceRecord {
        total_time: time.parse().unwrap(),
        record_distance: distance.parse().unwrap(),
    };
    race.find_all_winning_amounts().len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r##"Time:      7  15   30
Distance:  9  40  200"##;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 288)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT), 71503)
    }
}
