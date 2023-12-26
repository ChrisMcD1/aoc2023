use core::fmt;
use std::{collections::HashMap, convert::Infallible, fmt::Display, str::FromStr, time::Instant};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Tile {
    RoundedRock,
    CubeRock,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            Tile::RoundedRock => '0',
            Tile::CubeRock => '#',
            Tile::Empty => '.',
        };
        write!(f, "{char}")
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::RoundedRock,
            '#' => Tile::CubeRock,
            '.' => Tile::Empty,
            _ => unreachable!("Bad tile {value}"),
        }
    }
}

pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn next(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Platform {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Platform {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Ok(Self { tiles })
    }
}

impl Platform {
    fn location_from_blocker(
        &self,
        blocker: &Option<&(usize, usize)>,
        starting_row: usize,
        starting_col: usize,
        direction: &Direction,
    ) -> (usize, usize) {
        match blocker {
            Some((row, col)) => match direction {
                Direction::North => (row + 1, *col),
                Direction::West => (*row, col + 1),
                Direction::South => (row - 1, *col),
                Direction::East => (*row, col - 1),
            },
            None => match direction {
                Direction::North => (0, starting_col),
                Direction::West => (starting_row, 0),
                Direction::South => (self.height() - 1, starting_col),
                Direction::East => (starting_row, self.width() - 1),
            },
        }
    }
    fn find_final_location(
        &self,
        starting_row: usize,
        starting_col: usize,
        direction: &Direction,
    ) -> (usize, usize) {
        let potential_blockers =
            self.build_coordinates_in_direction(starting_row, starting_col, direction);
        let optional_blocker = potential_blockers
            .iter()
            .find(|(row, col)| self.tiles[*row][*col] != Tile::Empty);

        self.location_from_blocker(&optional_blocker, starting_row, starting_col, direction)
    }
    fn width(&self) -> usize {
        self.tiles[0].len()
    }
    fn height(&self) -> usize {
        self.tiles.len()
    }
    fn build_coordinates_in_direction(
        &self,
        starting_row: usize,
        starting_col: usize,
        direction: &Direction,
    ) -> Vec<(usize, usize)> {
        match direction {
            Direction::North => (0..starting_row)
                .rev()
                .map(|row| (row, starting_col))
                .collect(),
            Direction::West => (0..starting_col)
                .rev()
                .map(|col| (starting_row, col))
                .collect(),
            Direction::South => (starting_row + 1..self.height())
                .map(|row| (row, starting_col))
                .collect(),
            Direction::East => (starting_col + 1..self.width())
                .map(|col| (starting_row, col))
                .collect(),
        }
    }
    fn tiles_to_consider(&self, direction: &Direction) -> Vec<(usize, usize, Tile)> {
        use Direction::*;
        match direction {
            North | West => self
                .tiles
                .iter()
                .enumerate()
                .flat_map(move |(row, row_vec)| {
                    row_vec
                        .iter()
                        .enumerate()
                        .map(move |(col, tile)| (row, col, tile.clone()))
                })
                .collect(),
            South | East => self
                .tiles
                .iter()
                .enumerate()
                .rev()
                .flat_map(move |(row, row_vec)| {
                    row_vec
                        .iter()
                        .enumerate()
                        .rev()
                        .map(move |(col, tile)| (row, col, tile.clone()))
                })
                .collect(),
        }
    }
    fn tilt(&self, direction: &Direction) -> Self {
        let mut output: Platform = Platform {
            tiles: vec![vec![Tile::Empty; self.tiles[0].len()]; self.tiles.len()],
        };
        for (row, col, tile) in self.tiles_to_consider(direction) {
            let (adjusted_row, adjusted_col) = if tile == Tile::RoundedRock {
                Self::find_final_location(&output, row, col, direction)
            } else {
                (row, col)
            };
            output.tiles[adjusted_row][adjusted_col] = tile.clone()
        }
        output
    }
    fn calculate_load(&self) -> u32 {
        let max_score = self.tiles.len();
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let rounded_count = row
                    .iter()
                    .filter(|&tile| tile == &Tile::RoundedRock)
                    .count();
                rounded_count * (max_score - i)
            })
            .sum::<usize>()
            .try_into()
            .unwrap()
    }
    fn print(self) -> Self {
        println!("{self}");
        self
    }
    fn cycle(self) -> Self {
        self.tilt(&Direction::North)
            .tilt(&Direction::West)
            .tilt(&Direction::South)
            .tilt(&Direction::East)
    }
    fn identify_cycle_length(self) -> IdentifiedCycle {
        let mut seen_platforms: HashMap<Platform, usize> = HashMap::new();
        let mut platform: Platform = self;
        let mut current_cycle_count = 0;
        loop {
            seen_platforms.insert(platform.clone(), current_cycle_count);
            platform = platform.cycle();
            current_cycle_count += 1;
            if let Some(previous_cycle_count) = seen_platforms.get(&platform) {
                return IdentifiedCycle {
                    platform,
                    count_reached: current_cycle_count,
                    cycle_length: current_cycle_count - previous_cycle_count,
                };
            }
        }
    }
}

struct IdentifiedCycle {
    platform: Platform,
    count_reached: usize,
    cycle_length: usize,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines: Vec<String> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|tile| tile.to_string()).collect())
            .collect();
        let grid = lines.join("\n");
        write!(f, "Platform: \n{}", grid)
    }
}

pub fn part1(s: &str) -> u32 {
    s.parse::<Platform>()
        .unwrap()
        .tilt(&Direction::North)
        .calculate_load()
}

pub fn part2(s: &str) -> u32 {
    let start = Instant::now();
    let mut platform: Platform = s.parse().unwrap();
    let identified = platform.clone().identify_cycle_length();
    platform = identified.platform;
    println!("Ifentified cycles after {:?}", start.elapsed());
    let remaining_cycles = (1_000_000_000 - identified.count_reached) % identified.cycle_length;
    let cycles_to_reach_max = remaining_cycles;
    for _ in 0..cycles_to_reach_max {
        platform = platform.cycle();
    }
    let load = platform.calculate_load();
    println!("calculated load after {:?}", start.elapsed());
    load
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#########;

    const TILTED_INPUT: &str = r####"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."####;

    const CYCLE_1_INPUT: &str = r####".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."####;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 136)
    }

    #[test]
    fn test_tilt_up() {
        let input: Platform = GIVEN_INPUT.parse().unwrap();
        let expected: Platform = TILTED_INPUT.parse().unwrap();

        let actual = input.tilt(&Direction::North);

        assert_eq!(
            expected, actual,
            "testing expected {expected}\nvs actual {actual}"
        )
    }

    #[test]
    fn test_cycle() {
        let input: Platform = GIVEN_INPUT.parse().unwrap();
        let expected: Platform = CYCLE_1_INPUT.parse().unwrap();

        let actual = input.cycle();

        assert_eq!(
            expected, actual,
            "testing expected {expected}\nvs actual {actual}"
        )
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 64)
    }

    #[test]
    fn test_actual_1() {
        assert_eq!(part1(REAL_INPUT), 109665)
    }

    #[test]
    fn test_actual_2() {
        assert_eq!(part2(REAL_INPUT), 96061)
    }
}
