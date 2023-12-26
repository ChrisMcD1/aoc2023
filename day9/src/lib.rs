use std::{convert::Infallible, str::FromStr};

pub struct History(Vec<i64>);

impl FromStr for History {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i64> = s
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(History(values))
    }
}

impl History {
    fn produce_history(&self) -> Vec<Vec<i64>> {
        let mut values: Vec<Vec<i64>> = vec![self.0.clone()];
        let mut differences = self.0.clone();
        while differences.iter().any(|&x| x != 0) {
            differences = differences.windows(2).map(|s| s[1] - s[0]).collect();
            values.push(differences.clone());
        }
        values
    }

    #[inline]
    fn next_value_for_row(prev_row_next_value: i64, row: &Vec<i64>) -> i64 {
        row.last().unwrap() + prev_row_next_value
    }

    pub fn next_value(&self) -> i64 {
        self.produce_history()
            .iter()
            .rev()
            .fold(0, Self::next_value_for_row)
    }

    #[inline]
    fn previous_value_for_row(prev_row_prev_value: i64, row: &Vec<i64>) -> i64 {
        row.first().unwrap() - prev_row_prev_value
    }

    pub fn previous_value(&self) -> i64 {
        self.produce_history()
            .iter()
            .rev()
            .fold(0, Self::previous_value_for_row)
    }
}

pub fn part1(s: &str) -> i64 {
    s.lines()
        .map(|line| line.parse::<History>().unwrap())
        .map(|history| history.next_value())
        .sum()
}

pub fn part2(s: &str) -> i64 {
    s.lines()
        .map(|line| line.parse::<History>().unwrap())
        .map(|history| history.previous_value())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r##"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"##;

    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 114)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT), 2)
    }

    #[test]
    fn test_real_1() {
        assert_eq!(part1(REAL_INPUT), 1861775706)
    }

    #[test]
    fn test_real_2() {
        assert_eq!(part2(REAL_INPUT), 1082)
    }
}
