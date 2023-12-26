use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    convert::Infallible,
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CityMap {
    blocks: Vec<Vec<u32>>,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Location {
    row: i32,
    col: i32,
}

impl Location {
    fn move_direction(&self, direction: &Direction) -> Self {
        match direction {
            Up => Self {
                row: self.row - 1,
                col: self.col,
            },
            Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Left => Self {
                row: self.row,
                col: self.col - 1,
            },
            Right => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Path {
    total_heat_loss: u32,
    current_location: Location,
    direction: Direction,
    moves_in_this_direction: u8,
}
impl Path {
    fn distance_from_start(&self) -> u32 {
        let row: u32 = self.current_location.row.try_into().unwrap();
        let col: u32 = self.current_location.col.try_into().unwrap();
        row + col
    }
    fn complex_score(&self) -> u32 {
        self.distance_from_start() / ((self.total_heat_loss + 1) * 5)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.complex_score().partial_cmp(&other.complex_score())
        // (self.distance_from_start()
        //     .partial_cmp(&other.distance_from_start()))
        // if self.total_heat_loss < other.total_heat_loss {
        //     Some(Ordering::Greater)
        // } else if self.total_heat_loss == other.total_heat_loss {
        //     Some(Ordering::Equal)
        // } else {
        //     Some(Ordering::Less)
        // }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
use Direction::*;

impl CityMap {
    fn get_block(&self, location: &Location) -> Option<u32> {
        let row: usize = location.row.try_into().ok()?;
        let col: usize = location.col.try_into().ok()?;
        self.blocks.get(row)?.get(col).copied()
    }
    pub fn generate_new_paths(&self, path: Path) -> Vec<Path> {
        let valid_directions: Vec<Direction> = if path.moves_in_this_direction >= 2 {
            // Must turn
            match path.direction {
                Up | Down => vec![Left, Right],
                Left | Right => vec![Up, Down],
            }
        } else {
            // Can go straight or turn!
            match path.direction {
                Up => vec![Left, Up, Right],
                Down => vec![Left, Down, Right],
                Left => vec![Up, Left, Down],
                Right => vec![Up, Right, Down],
            }
        };
        valid_directions
            .into_iter()
            .filter_map(|direction| {
                let new_location = path.current_location.move_direction(&direction);
                let heat_cost = self.get_block(&new_location)?;
                let moves_in_this_direction = if path.direction == direction {
                    path.moves_in_this_direction + 1
                } else {
                    0
                };
                Some(Path {
                    total_heat_loss: path.total_heat_loss + heat_cost,
                    current_location: new_location,
                    direction,
                    moves_in_this_direction,
                })
            })
            .collect()
    }

    fn width(&self) -> i32 {
        self.blocks[0].len().try_into().unwrap()
    }

    fn height(&self) -> i32 {
        self.blocks.len().try_into().unwrap()
    }

    fn path_reached_end(&self, path: &Path) -> bool {
        path.current_location.row == self.height() - 1
            && path.current_location.col == self.width() - 1
    }

    pub fn find_smallest_heat_loss_path(&self) -> Path {
        let mut paths_in_consideration: BinaryHeap<Path> = BinaryHeap::new();
        paths_in_consideration.push(Path {
            total_heat_loss: 0,
            current_location: Location { row: 0, col: 0 },
            direction: Down,
            moves_in_this_direction: 0,
        });
        paths_in_consideration.push(Path {
            total_heat_loss: 0,
            current_location: Location { row: 0, col: 0 },
            direction: Right,
            moves_in_this_direction: 0,
        });
        let mut winning_path: Option<Path> = None;
        let mut seen_paths: HashSet<Path> = HashSet::new();
        while let Some(path) = paths_in_consideration.pop() {
            if seen_paths.contains(&path) {
                continue;
            }
            seen_paths.insert(path.clone());
            // println!("Considering path: {path:?}");
            if self.path_reached_end(&path) {
                match &winning_path {
                    Some(current_winner) => {
                        if current_winner.total_heat_loss > path.total_heat_loss {
                            winning_path = Some(path);
                        }
                    }
                    None => winning_path = Some(path),
                }
            } else {
                if let Some(winner) = &winning_path {
                    if winner.total_heat_loss < path.total_heat_loss {
                        continue;
                    }
                }
                let new_paths = self.generate_new_paths(path);
                for new_path in new_paths.into_iter() {
                    // if seen_paths.contains(&new_path) {
                    //     continue;
                    // } else {
                    //     seen_paths.insert(new_path.clone());
                    paths_in_consideration.push(new_path);
                    // }
                }
            }
        }
        winning_path.expect("Should have finished the end of this loop with a path!")
    }
}

impl FromStr for CityMap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<Vec<u32>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
        Ok(Self { blocks })
    }
}

pub fn part1(s: &str) -> u32 {
    let city_map: CityMap = s.parse().unwrap();
    let best_path = city_map.find_smallest_heat_loss_path();
    println!("WE found the best path of {best_path:?}");
    best_path.total_heat_loss
}

pub fn part2(s: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 102)
    }

    #[test]
    #[ignore]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 51)
    }
}
