use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
enum Square {
    Ash,
    Rock,
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!("Can only have a couple chars"),
        }
    }
}

struct Pattern {
    squares: Vec<Vec<Square>>,
}

impl FromStr for Pattern {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let squares: Vec<Vec<Square>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Ok(Self { squares })
    }
}

#[derive(Debug)]
enum MirrorLine {
    Horizontal(u32),
    Vertical(u32),
}

impl MirrorLine {
    fn score(&self) -> u32 {
        match self {
            MirrorLine::Horizontal(rows_above) => rows_above * 100,
            MirrorLine::Vertical(columns_left) => *columns_left,
        }
    }
}
fn count_difference_between_squares(one: &[Square], two: &[Square]) -> u32 {
    one.iter()
        .zip(two.iter())
        .filter(|(a, b)| a != b)
        .count()
        .try_into()
        .unwrap()
}

impl Pattern {
    fn vertically_symmetrical_difference(&self, col: usize) -> u32 {
        let lowest_distance_to_edge = (self.squares[0].len() - (col + 1)).min(col + 1);
        let should_be_symmetrical_pairs = (1..=lowest_distance_to_edge).map(|distance| {
            (
                self.squares
                    .iter()
                    .map(|row| row[col + 1 - distance].clone())
                    .collect::<Vec<_>>(),
                self.squares
                    .iter()
                    .map(|row| row[col + distance].clone())
                    .collect::<Vec<_>>(),
            )
        });
        should_be_symmetrical_pairs
            .map(|(one, two)| count_difference_between_squares(&one, &two))
            .sum()
    }
    fn horizontally_symmetrical_difference(&self, row: usize) -> u32 {
        let lowest_distance_to_edge = (self.squares.len() - (row + 1)).min(row + 1);
        let should_be_symmetrical_pairs = (1..=lowest_distance_to_edge).map(|distance| {
            (
                &self.squares[row + 1 - distance],
                &self.squares[row + distance],
            )
        });
        should_be_symmetrical_pairs
            .map(|(one, two)| count_difference_between_squares(&one, &two))
            .sum()
    }
    fn mirror_line(&self, allowed_differences: u32) -> MirrorLine {
        let maybe_horizontal_line = (0..self.squares.len() - 1).find_map(|row| {
            (self.horizontally_symmetrical_difference(row) == allowed_differences)
                .then_some(MirrorLine::Horizontal((row + 1).try_into().unwrap()))
        });
        if let Some(horizontal_line) = maybe_horizontal_line {
            return horizontal_line;
        }

        let maybe_vertical_line = (0..self.squares[0].len() - 1).find_map(|col| {
            (self.vertically_symmetrical_difference(col) == allowed_differences)
                .then_some(MirrorLine::Vertical((col + 1).try_into().unwrap()))
        });
        if let Some(vertical_line) = maybe_vertical_line {
            return vertical_line;
        }
        unreachable!()
    }
}
pub fn part1(s: &str) -> u32 {
    s.split("\n\n")
        .map(|block| block.parse::<Pattern>().unwrap())
        .map(|pattern| pattern.mirror_line(0))
        .map(|line| line.score())
        .sum()
}

pub fn part2(s: &str) -> u32 {
    s.split("\n\n")
        .map(|block| block.parse::<Pattern>().unwrap())
        .map(|pattern| pattern.mirror_line(1))
        .map(|line| line.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 405)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 400)
    }
}
