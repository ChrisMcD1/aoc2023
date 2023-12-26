use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Location {
    row: i32,
    col: i32,
}

impl Location {
    fn move_step(&self, step: &Step) -> Vec<(Location, ColorCode)> {
        let locations: Vec<Location> = match step.direction {
            Direction::Up => (1..=step.distance)
                .map(|up| Location {
                    row: self.row - up,
                    col: self.col,
                })
                .collect(),
            Direction::Down => (1..=step.distance)
                .map(|down| Location {
                    row: self.row + down,
                    col: self.col,
                })
                .collect(),
            Direction::Left => (1..=step.distance)
                .map(|left| Location {
                    row: self.row,
                    col: self.col - left,
                })
                .collect(),
            Direction::Right => (1..=step.distance)
                .map(|right| Location {
                    row: self.row,
                    col: self.col + right,
                })
                .collect(),
        };
        locations
            .into_iter()
            .map(|location| (location, step.color_code.clone()))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type ColorCode = String;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Step {
    direction: Direction,
    distance: i32,
    color_code: ColorCode,
}

impl FromStr for Step {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let substrs: Vec<&str> = s.split(" ").collect();
        let dir_string = substrs[0];
        let distance_string = substrs[1];
        let color_code = substrs[2].to_owned();
        let direction: Direction = match dir_string {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };
        let distance: i32 = distance_string.parse().unwrap();
        Ok(Self {
            direction,
            distance,
            color_code,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Steps(Vec<Step>);

impl FromStr for Steps {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(|line| line.parse().unwrap()).collect()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Locations(Vec<(Location, ColorCode)>);

impl From<Steps> for Locations {
    fn from(value: Steps) -> Self {
        let mut locations: Vec<(Location, ColorCode)> = vec![];
        let mut past_position = Location { row: 0, col: 0 };
        for step in value.0.into_iter() {
            let mut next_locations = past_position.move_step(&step);
            past_position = match next_locations.last() {
                Some(location) => location.0.clone(),
                None => past_position,
            };
            locations.append(&mut next_locations);
        }
        Locations(locations)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Layout(Vec<Vec<Option<ColorCode>>>);
impl From<Locations> for Layout {
    fn from(locations: Locations) -> Self {
        let lowest_row: usize = locations
            .0
            .iter()
            .map(|location| location.0.row)
            .min()
            .unwrap()
            .try_into()
            .unwrap();
        let lowest_col: usize = locations
            .0
            .iter()
            .map(|location| location.0.col)
            .min()
            .unwrap()
            .try_into()
            .unwrap();
        let highest_row: usize = locations
            .0
            .iter()
            .map(|location| location.0.row)
            .max()
            .unwrap()
            .try_into()
            .unwrap();
        let highest_col: usize = locations
            .0
            .iter()
            .map(|location| location.0.col)
            .max()
            .unwrap()
            .try_into()
            .unwrap();

        let width: usize = ((highest_col - lowest_col) + 1).try_into().unwrap();
        let height: usize = ((highest_row - lowest_row) + 1).try_into().unwrap();
        let mut layout: Vec<Vec<Option<ColorCode>>> = vec![vec![None; width]; height];
        for location in locations.0 {
            let row: usize = location.0.row.try_into().unwrap();
            let col: usize = location.0.col.try_into().unwrap();
            let row: usize = row - lowest_row;
            let col: usize = col - lowest_col;
            layout[row][col] = Some(location.1);
        }
        Layout(layout)
    }
}

impl Layout {
    fn pretty_string(&self) -> String {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|location| match location {
                        Some(_) => "#",
                        None => ".",
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn fill_inside(&mut self) {
        for row in self.0.iter_mut() {
            let mut inside = false;
            for space in row.iter_mut() {
                if space.is_some() {
                    inside = !inside;
                }
                if inside {
                    *space = Some("hello".to_string());
                }
            }
        }
    }
    fn count_volume(&self) -> u32 {
        let sum: usize = self
            .0
            .iter()
            .map(|row| row.iter().filter(|tile| tile.is_some()).count())
            .sum();
        sum.try_into().unwrap()
    }
}

pub fn part1(s: &str) -> u32 {
    let steps: Steps = s.parse().unwrap();
    let locations: Locations = steps.into();
    let mut layout: Layout = locations.into();
    println!("Layout: {layout:?}");
    println!("before filling we have: {:?}", layout.count_volume());
    println!("layout_pretty: {:?}", layout.pretty_string());
    layout.fill_inside();
    println!("layout_pretty after filling: {:?}", layout.pretty_string());
    layout.count_volume()
}

pub fn part2(s: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 62)
    }

    #[test]
    #[ignore]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 51)
    }
}
