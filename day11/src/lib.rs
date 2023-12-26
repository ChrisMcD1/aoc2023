use std::{convert::Infallible, str::FromStr};

#[derive(Clone, Debug, PartialEq, Copy)]
enum Tile {
    Galaxy,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RawImage(Vec<Vec<Tile>>);

impl RawImage {
    fn expansion_rows(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, row)| row.iter().all(|t| t == &Tile::Empty).then_some(index))
            .collect()
    }
    fn expansion_columns(&self) -> Vec<usize> {
        (0..self.0[0].len())
            .filter(|column_index| {
                self.0
                    .iter()
                    .map(|row| row[*column_index])
                    .all(|t| t == Tile::Empty)
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ProcessedImage(Vec<Vec<Tile>>);

impl From<RawImage> for ProcessedImage {
    fn from(value: RawImage) -> Self {
        let mut new_guy = value.clone();
        // Expand Horizontally
        let empty_rows = value.expansion_rows();
        let empty_rows_shifted = empty_rows.iter().enumerate().map(|(i, row)| i + row);
        let empty_row = vec![Tile::Empty; new_guy.0.len()];
        for index in empty_rows_shifted {
            new_guy.0.insert(index, empty_row.clone())
        }

        // Expand Vertically
        let empty_columns = value.expansion_columns();
        let empty_columns_shifted = empty_columns.iter().enumerate().map(|(i, col)| i + col);
        for column_index in empty_columns_shifted {
            for row_index in 0..new_guy.0.len() {
                new_guy.0[row_index].insert(column_index, Tile::Empty);
            }
        }

        ProcessedImage(new_guy.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MegaExpandedImage {
    raw: Vec<Vec<Tile>>,
    expansion_rows: Vec<i64>,
    expansion_columns: Vec<i64>,
    expansion_factor: i64,
}

fn count_instances_fast(values: &[i64], lower: &i64, upper: &i64) -> i64 {
    values
        .iter()
        .filter(|&col| col <= &upper && col >= &lower)
        .count()
        .try_into()
        .unwrap()
}

struct Pairer<'a, T> {
    data: &'a [T],
}

impl<'a, T: Clone + PartialEq> Pairer<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Self { data }
    }

    fn comparisons(self) -> impl Iterator<Item = (&'a T, &'a T)> + 'a {
        self.data.iter().enumerate().flat_map(move |(i, one)| {
            self.data[i..]
                .iter()
                .map(move |two| (one, two))
                .filter(|(a, b)| a != b)
        })
    }
}

impl MegaExpandedImage {
    fn distances_between_galaxies(&self) -> Vec<i64> {
        let galaxies = identify_galaxies(&self.raw);
        let iterator = Pairer::new(&galaxies);

        iterator
            .comparisons()
            .map(|(one, two)| {
                let up = (one.0).min(two.0);
                let down = (one.0).max(two.0);
                let left = (one.1).min(two.1);
                let right = (one.1).max(two.1);
                let base_horizontal_distance = right - left;
                let expansion_columns_crossed =
                    count_instances_fast(&self.expansion_columns, &left, &right);

                let total_horizontal_distance: i64 = base_horizontal_distance
                    + (expansion_columns_crossed * (self.expansion_factor - 1));

                let base_vertical_distance = down - up;
                let expansion_rows_crossed: i64 =
                    count_instances_fast(&self.expansion_rows, &up, &down);
                let total_vertical_distance: i64 = (base_vertical_distance)
                    + (expansion_rows_crossed * (self.expansion_factor - 1));
                let distance = total_vertical_distance + total_horizontal_distance;
                distance
            })
            .collect()
    }
}

impl MegaExpandedImage {
    pub fn new(value: RawImage, expansion_factor: i64) -> Self {
        Self {
            expansion_rows: value
                .expansion_rows()
                .into_iter()
                .map(|a| a.try_into().unwrap())
                .collect(),
            expansion_columns: value
                .expansion_columns()
                .into_iter()
                .map(|a| a.try_into().unwrap())
                .collect(),
            raw: value.0,
            expansion_factor,
        }
    }
}

impl FromStr for RawImage {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.chars().map(|c| c.into()).collect())
                .collect(),
        ))
    }
}

fn identify_galaxies(value: &Vec<Vec<Tile>>) -> Vec<(i64, i64)> {
    value
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, tile)| {
                    (tile == &Tile::Galaxy)
                        .then_some((i.try_into().unwrap(), j.try_into().unwrap()))
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

impl ProcessedImage {
    fn distances_between_galaxies(&self) -> Vec<i64> {
        let galaxies: Vec<(i64, i64)> = identify_galaxies(&self.0);
        galaxies
            .iter()
            .flat_map(|one| {
                galaxies.iter().filter_map(|two| {
                    let left = (one.0).min(two.0);
                    let right = (one.0).max(two.0);
                    let up = (one.1).min(two.1);
                    let down = (one.1).max(two.1);
                    let horizontal_distance = right - left;
                    let vertical_distance = down - up;
                    let distance = horizontal_distance + vertical_distance;
                    (*one != *two).then_some(distance)
                })
            })
            .collect()
    }
}

pub fn part1(s: &str) -> i64 {
    let processed_image: ProcessedImage = s.parse::<RawImage>().unwrap().into();
    let distances = processed_image.distances_between_galaxies();
    distances.iter().sum::<i64>() / 2
}

pub fn part2(s: &str, expansion_factor: i64) -> i64 {
    let raw_image: RawImage = s.parse().unwrap();
    let mega_image: MegaExpandedImage = MegaExpandedImage::new(raw_image, expansion_factor);
    mega_image.distances_between_galaxies().iter().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");

    const SAMPLE_INPUT: &str = r########"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 374)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT, 10), 1030)
    }

    #[test]
    fn test_full_1() {
        assert_eq!(part1(REAL_INPUT), 9795148)
    }

    #[test]
    fn test_full_2() {
        assert_eq!(part2(REAL_INPUT, 1_000_000), 650672493820)
    }

    #[test]
    fn test_checking() {
        let left = 0;
        let right = 10;
        let values = vec![-1, 2, 4, 11];
        let first_way: Vec<i32> = (left..=right).filter(|a| values.contains(a)).collect();
        let second_way: Vec<i32> = values
            .clone()
            .into_iter()
            .filter(|&col| col <= right && col >= left)
            .collect();

        assert_eq!(first_way, second_way)
    }

    #[test]
    fn comparisons_simple() {
        let inputed = vec![(0, 1), (0, 2), (0, 3)];
        let expected_vec = vec![((0, 1), (0, 2)), ((0, 1), (0, 3)), ((0, 2), (0, 3))];
        let expect_pairs: HashSet<((i32, i32), (i32, i32))> = expected_vec.into_iter().collect();

        let iterator = Pairer::new(&inputed).comparisons();
        let actual_pairs: HashSet<((i32, i32), (i32, i32))> = iterator
            .map(|(l, r)| (l.to_owned(), r.to_owned()))
            .collect();

        assert_eq!(expect_pairs, actual_pairs);
    }

    use Tile::*;

    #[test]
    fn process_image() {
        let sample = r####".#.
.##
..."####;
        let expected_output = ProcessedImage(vec![
            vec![Empty, Empty, Galaxy, Empty],
            vec![Empty, Empty, Galaxy, Galaxy],
            vec![Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Empty],
        ]);

        let processed: ProcessedImage = sample.parse::<RawImage>().unwrap().into();
        assert_eq!(expected_output, processed);
    }
}
