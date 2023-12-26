use std::{collections::VecDeque, convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '#' => Condition::Damaged,
            '.' => Condition::Operational,
            '?' => Condition::Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct History(Vec<Condition>);

impl History {
    pub fn is_fully_valid(&self, damaged_groups: &[u32]) -> bool {
        let actual = self.actual_damaged_groups();
        (actual.len() == damaged_groups.len())
            && actual
                .iter()
                .zip(damaged_groups.iter())
                .all(|(actual, expected)| actual == expected)
    }
    pub fn actual_damaged_groups(&self) -> Vec<u32> {
        let mut iter = self.0.iter().peekable();
        let mut output: Vec<u32> = vec![];
        while let Some(val) = iter.next() {
            match val {
                Condition::Damaged => {
                    let mut count = 1;
                    while let Some(Condition::Damaged) = iter.peek() {
                        count += 1;
                        iter.next();
                    }
                    output.push(count);
                }
                Condition::Operational => (),
                Condition::Unknown => (),
            }
        }
        output
    }
    pub fn has_no_unknown(&self) -> bool {
        !self.0.iter().any(|c| c == &Condition::Unknown)
    }
    pub fn produce_next_histories(&self) -> Vec<History> {
        if self.has_no_unknown() {
            vec![]
        } else {
            // Find the first that is unknown
            // Make the two new state
            let i = self
                .0
                .iter()
                .enumerate()
                .find_map(|(i, c)| (c == &Condition::Unknown).then_some(i))
                .unwrap();

            let mut operational = self.clone();
            operational.0[i] = Condition::Operational;
            let mut damaged = self.clone();
            damaged.0[i] = Condition::Damaged;

            vec![operational, damaged]
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionRecord {
    history: History,
    damaged_groups: Vec<u32>,
}

impl FromStr for ConditionRecord {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (history_str, groups_str) = s.split_once(" ").unwrap();
        let history: History = History(history_str.chars().map(|c| c.into()).collect());
        let damaged_groups: Vec<u32> = groups_str
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        Ok(ConditionRecord {
            history,
            damaged_groups,
        })
    }
}

impl ConditionRecord {
    pub fn unfold(&self) -> Self {
        let mut history = vec![];
        let mut damaged_groups = vec![];
        for _ in 0..5 {
            history.append(&mut self.history.clone().0);
            damaged_groups.append(&mut self.damaged_groups.clone());
        }
        Self {
            history: History(history),
            damaged_groups,
        }
    }
    pub fn different_arangements(&self) -> u32 {
        let mut arrangements_to_check: Vec<History> = vec![];
        let mut still_unknown: VecDeque<History> = VecDeque::new();
        still_unknown.push_front(self.history.clone());
        while let Some(history) = still_unknown.pop_front() {
            // println!("Looking at history: {history:?}");
            if history.has_no_unknown() {
                arrangements_to_check.push(history)
            } else {
                let next_histories = history.produce_next_histories();
                for history in next_histories {
                    still_unknown.push_back(history);
                }
            }
        }
        // println!("Found total arrangements {arrangements_to_check:?}");
        arrangements_to_check
            .iter()
            .filter(|&history| history.is_fully_valid(&self.damaged_groups))
            .count()
            .try_into()
            .unwrap()
    }
}

pub fn part1(s: &str) -> u32 {
    s.lines()
        .map(|line| line.parse::<ConditionRecord>().unwrap())
        .map(|record| record.different_arangements())
        .sum()
}

pub fn part2(s: &str) -> u32 {
    s.lines()
        .map(|line| line.parse::<ConditionRecord>().unwrap())
        .map(|record| record.different_arangements())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_given_1() {
        assert_eq!(part1(REAL_INPUT), 7344)
    }

    #[test]
    fn test_one_arrangement() {
        let record: ConditionRecord = "???.### 1,1,3".parse().unwrap();

        let arrangements = record.different_arangements();

        assert_eq!(arrangements, 1)
    }

    #[test]
    fn count_actual_grouped() {
        let record: ConditionRecord = ".#....###.#.###### 1,3,1,6".parse().unwrap();
        let expected_groups = vec![1, 3, 1, 6];

        let actual_groups = record.history.actual_damaged_groups();

        assert_eq!(expected_groups, actual_groups)
    }
}
