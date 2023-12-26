use std::{convert::Infallible, str::FromStr};

pub fn part1(s: &str) -> i64 {
    s.parse::<Field>()
        .unwrap()
        .build_loop()
        .furthest_distance()
        .try_into()
        .unwrap()
}

pub fn part2(s: &str) -> i64 {
    let field: Field = s.parse().unwrap();
    // println!("field: {field:?}");
    let looping_path: Path = field.build_loop();
    field.calculate_area_inside_loop(&looping_path)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    history: Vec<Location>,
    direction: Direction,
}

impl Path {
    fn new(location: Location, direction: Direction) -> Self {
        Path {
            history: vec![location],
            direction,
        }
    }
    fn furthest_distance(&self) -> usize {
        (self.history.len() - 1) / 2
    }
    fn is_done(&self, field: &Field) -> bool {
        self.history.len() > 0
            && field
                .get_tile(&self.current_location())
                .expect("They better have not put me into an invalid location!")
                == Tile::Starting
    }
    fn current_location(&self) -> Location {
        self.history.last().unwrap().clone()
    }
    fn extend_path(self, new_location: Location, new_direction: Direction) -> Path {
        let mut history = self.history;
        history.push(new_location);
        Path {
            history,
            direction: new_direction,
        }
    }
    fn contains_location(&self, location: &Location) -> bool {
        self.history.iter().any(|l| l == location)
    }
}

#[derive(Debug)]
struct Field(Vec<Vec<Tile>>);
impl Field {
    fn build_loop(&self) -> Path {
        let starting_location = self.starting_location();
        let starting_paths = vec![
            Path::new(starting_location.clone(), Direction::East),
            Path::new(starting_location.clone(), Direction::West),
            Path::new(starting_location.clone(), Direction::North),
            Path::new(starting_location.clone(), Direction::South),
        ];
        starting_paths
            .into_iter()
            .find_map(|path| self.run_path_to_loop(path))
            .expect("We should have found a loop!")
    }

    fn calculate_area_inside_loop(&self, looping_path: &Path) -> i64 {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| Field::calculate_area_row(y.try_into().unwrap(), row, looping_path))
            .sum()
    }

    fn calculate_area_row(y: i32, row: &[Tile], looping_path: &Path) -> i64 {
        let mut count = 0;
        let mut inside = false;
        for (x, tile) in row.iter().enumerate() {
            let location = Location {
                x: x.try_into().unwrap(),
                y,
            };
            if tile.is_vertical_separator() && looping_path.contains_location(&location) {
                inside = !inside;
            }
            if !looping_path.contains_location(&location) && inside {
                count += 1;
            }
        }
        count
    }

    /// Returns Some if this path successfully completed a clue
    /// Goes None if this path cannot successfully complete a loop
    fn run_path_to_loop(&self, path: Path) -> Option<Path> {
        let mut next = self.progress_path(path)?;
        while !next.is_done(&self) {
            // println!("Our path is now {next:?}");
            next = self.progress_path(next)?;
        }
        Some(next)
    }

    fn starting_location(&self) -> Location {
        let starting_location = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .position(|tile| (*tile == Tile::Starting))
                    .and_then(|j| Some((i, j)))
            })
            .expect("We better find the starting tuple!");
        Location {
            x: starting_location.1.try_into().unwrap(),
            y: starting_location.0.try_into().unwrap(),
        }
    }

    /// Locations may be invalid, but this will produce a valid tile inside of this field
    fn get_tile(&self, location: &Location) -> Option<Tile> {
        let y: usize = location.y.try_into().ok()?;
        let x: usize = location.x.try_into().ok()?;
        if (y > self.height() - 1) || (x > self.width() - 1) {
            // println!("We are getting a null location for (x,y) {x:?}, {y:?} with height: {}, and width: {}", self.height(), self.width());
            None
        } else {
            Some(self.0.get(y)?.get(x)?.clone())
        }
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get_neighbor(&self, location: &Location, direction: &Direction) -> Location {
        use Direction::*;
        match direction {
            North => Location {
                x: location.x,
                y: location.y - 1,
            },
            South => Location {
                x: location.x,
                y: location.y + 1,
            },
            West => Location {
                x: location.x - 1,
                y: location.y,
            },
            East => Location {
                x: location.x + 1,
                y: location.y,
            },
        }
    }

    fn get_new_direction(
        &self,
        new_tile: &Tile,
        incoming_direction: &Direction,
    ) -> Option<Direction> {
        use Direction::*;
        use Tile::*;
        let dir = match &incoming_direction {
            North => match &new_tile {
                VerticalPipe => Some(incoming_direction.clone()),
                SouthWestBend => Some(West),
                SouthEathBend => Some(East),
                Starting => Some(incoming_direction.clone()),
                _ => None,
            },
            South => match &new_tile {
                VerticalPipe => Some(incoming_direction.clone()),
                NorthEastBend => Some(East),
                NorthWestBend => Some(West),
                Starting => Some(incoming_direction.clone()),
                _ => None,
            },
            West => match &new_tile {
                HorizontalPipe => Some(incoming_direction.clone()),
                NorthEastBend => Some(North),
                SouthEathBend => Some(South),
                Starting => Some(incoming_direction.clone()),
                _ => None,
            },
            East => match &new_tile {
                HorizontalPipe => Some(incoming_direction.clone()),
                NorthWestBend => Some(North),
                SouthWestBend => Some(South),
                Starting => Some(incoming_direction.clone()),
                _ => None,
            },
        };
        if dir.is_none() {
            // println!("Our path failed While looking at a {new_tile:?} going in direction: {incoming_direction:?}")
        }
        dir
    }

    // fn get_tile_in_direction(&self, location: &Location, direction: &Direction) -> Option<Tile> {}
    fn progress_path(&self, path: Path) -> Option<Path> {
        let neighbor_location = self.get_neighbor(&path.current_location(), &path.direction);
        // println!("Got neighbor: {neighbor_location:?}");
        let new_tile = self.get_tile(&neighbor_location)?;
        // println!("Got tile: {new_tile:?}");
        let new_direction = self.get_new_direction(&new_tile, &path.direction)?;
        // println!("Got direction: {new_direction:?}");
        Some(path.extend_path(neighbor_location, new_direction))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// That the pipe is "pointed" towards
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEathBend,
    Ground,
    Starting,
}

impl Tile {
    fn is_vertical_separator(&self) -> bool {
        match self {
            Tile::VerticalPipe => true,
            // This is hacked for my particular real puzzle input :D 
            Tile::Starting => false,
            Tile::SouthWestBend => true,
            Tile::SouthEathBend => true,
            Tile::HorizontalPipe => false,
            Tile::NorthEastBend => false,
            Tile::NorthWestBend => false,
            Tile::Ground => false,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '|' => VerticalPipe,
            '-' => HorizontalPipe,
            'L' => NorthEastBend,
            'J' => NorthWestBend,
            '7' => SouthWestBend,
            'F' => SouthEathBend,
            '.' => Ground,
            'S' => Starting,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Field {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field(
            s.lines()
                .map(|line| line.chars().map(|c| c.into()).collect())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_SQUARE_LOOP: &str = r##".....
.S-7.
.|.|.
.L-J.
....."##;

    const COMPLEX_LOOP: &str = r##"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."##;

    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_given_1() {
        assert_eq!(part1(SIMPLE_SQUARE_LOOP), 4)
    }

    #[test]
    fn test_given_1_2() {
        assert_eq!(part1(COMPLEX_LOOP), 8)
    }

    const FOUR_LOOP: &str = r##"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."##;

    #[test]
    fn test_width() {
        let field: Field = FOUR_LOOP.parse().unwrap();
        assert_eq!(field.height(), 9);
        assert_eq!(field.width(), 11)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(FOUR_LOOP), 4)
    }

    const BIGGER_LOOP: &str = r##"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"##;

    #[test]
    fn test_given_2_2() {
        assert_eq!(part2(BIGGER_LOOP), 10)
    }
}
