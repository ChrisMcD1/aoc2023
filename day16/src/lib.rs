use std::{collections::HashSet, convert::Infallible, str::FromStr};

#[derive(Clone, Debug, Copy)]
pub enum MirrorEnum {
    UpToTheRight,
    DownToTheRight,
}

#[derive(Clone, Debug, Copy)]
pub enum SplitterEnum {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Copy)]
pub enum Tile {
    Empty,
    Mirror(MirrorEnum),
    Splitter(SplitterEnum),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use MirrorEnum::*;
        use SplitterEnum::*;
        match value {
            '.' => Tile::Empty,
            '-' => Tile::Splitter(Horizontal),
            '|' => Tile::Splitter(Vertical),
            '\\' => Tile::Mirror(DownToTheRight),
            '/' => Tile::Mirror(UpToTheRight),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Position {
    col: i32,
    row: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Position {
        Self { row, col }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn new(position: Position, direction: Direction) -> Self {
        Beam {
            position,
            direction,
        }
    }
    fn next_position(&self) -> Position {
        let row = self.position.row;
        let col = self.position.col;
        match self.direction {
            Direction::Up => Position::new(row - 1, col),
            Direction::Down => Position::new(row + 1, col),
            Direction::Left => Position::new(row, col - 1),
            Direction::Right => Position::new(row, col + 1),
        }
    }
}

pub struct Field(Vec<Vec<Tile>>);
impl Field {
    fn width(&self) -> i32 {
        self.0.len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.0[0].len().try_into().unwrap()
    }
    fn get_tile(&self, position: &Position) -> Option<&Tile> {
        let row_index: Option<usize> = position.row.try_into().ok();
        let col_index: Option<usize> = position.col.try_into().ok();
        self.0.get(row_index?)?.get(col_index?)
    }
    fn progress_beam(&self, beam: &Beam) -> Option<BeamProgress> {
        let beam_next_position = beam.next_position();
        let tile = self.get_tile(&beam_next_position)?;
        use Direction::*;
        use MirrorEnum::*;
        use SplitterEnum::*;
        use Tile::*;
        let new_beam_directions: Vec<Direction> = match (tile, beam.direction) {
            (Empty, _) | (Splitter(Horizontal), Right | Left) | (Splitter(Vertical), Down | Up) => {
                vec![beam.direction]
            }
            (Mirror(UpToTheRight), Right) => vec![Up],
            (Mirror(UpToTheRight), Left) => vec![Down],
            (Mirror(UpToTheRight), Down) => vec![Left],
            (Mirror(UpToTheRight), Up) => vec![Right],

            (Mirror(DownToTheRight), Right) => vec![Down],
            (Mirror(DownToTheRight), Left) => vec![Up],
            (Mirror(DownToTheRight), Down) => vec![Right],
            (Mirror(DownToTheRight), Up) => vec![Left],
            (Splitter(Horizontal), Down | Up) => vec![Left, Right],
            (Splitter(Vertical), Left | Right) => vec![Up, Down],
        };

        let new_beams: Vec<Beam> = new_beam_directions
            .into_iter()
            .map(|direction| Beam::new(beam_next_position.clone(), direction))
            .collect();

        Some(BeamProgress {
            new_beams,
            energized: vec![beam_next_position].into_iter().collect(),
        })
    }
    fn energized_squares_from(&self, initial_beam: Beam) -> u32 {
        let width: usize = self.width().try_into().unwrap();
        let height: usize = self.height().try_into().unwrap();
        let mut beams: Vec<Beam> = vec![initial_beam];
        let mut previously_seen_beams: HashSet<Beam> = HashSet::with_capacity(width * height * 4);
        let mut energized: HashSet<Position> = HashSet::with_capacity(width * height);
        while let Some(beam) = beams.pop() {
            let progress_opt = self.progress_beam(&beam);
            match progress_opt {
                Some(progress) => {
                    for new_beam in progress.new_beams {
                        if !previously_seen_beams.contains(&new_beam) {
                            previously_seen_beams.insert(new_beam.clone());
                            beams.push(new_beam);
                        }
                    }
                    for energy in progress.energized {
                        energized.insert(energy);
                    }
                }
                None => (),
            }
        }
        energized.len().try_into().unwrap()
    }
}

pub struct BeamProgress {
    new_beams: Vec<Beam>,
    energized: HashSet<Position>,
}

impl FromStr for Field {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Ok(Self(data))
    }
}

pub fn part1(s: &str) -> u32 {
    let field: Field = s.parse().unwrap();
    field.energized_squares_from(Beam::new(Position::new(0, -1), Direction::Right))
}

pub fn part2(s: &str) -> u32 {
    let field: Field = s.parse().unwrap();
    let left_wall =
        (0..field.height()).map(|row| Beam::new(Position::new(row, -1), Direction::Right));
    let right_wall = (0..field.height())
        .map(|row| Beam::new(Position::new(row, field.width()), Direction::Left));
    let top_wall = (0..field.width()).map(|col| Beam::new(Position::new(-1, col), Direction::Down));
    let bottom_wall =
        (0..field.width()).map(|col| Beam::new(Position::new(field.height(), col), Direction::Up));
    let all_potential_starting_beams = left_wall
        .chain(right_wall)
        .chain(top_wall)
        .chain(bottom_wall);
    all_potential_starting_beams
        .map(|starting_beam| field.energized_squares_from(starting_beam))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 46)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 51)
    }

    #[test]
    fn test_real_1() {
        assert_eq!(part1(REAL_INPUT), 8539)
    }

    #[test]
    #[ignore]
    fn test_real_2() {
        assert_eq!(part2(REAL_INPUT), 8674)
    }
}
