use itertools::Itertools;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Position {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(",").collect::<Vec<_>>();
        let x = values[0].trim().parse().unwrap();
        let y = values[1].trim().parse().unwrap();
        let z = values[2].trim().parse().unwrap();
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Velocity {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(",").collect::<Vec<_>>();
        let x = values[0].trim().parse().unwrap();
        let y = values[1].trim().parse().unwrap();
        let z = values[2].trim().parse().unwrap();
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl FromStr for Hailstone {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position_string, velocity_string) = s.split_once("@").unwrap();
        let position: Position = position_string.parse()?;
        let velocity: Velocity = velocity_string.parse()?;
        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum HailstoneCollision {
    Past(Position),
    Future(Position),
    NoCollision,
}

struct SlopeIntercept {
    slope: f64,
    intercept: f64,
}

impl Hailstone {
    fn slope_intercept_form(&self) -> SlopeIntercept {
        // We have x, y, position and velocity
        // (y - y1) = M(x - x1)
        let slope = self.velocity.y / self.velocity.x;
        let intercept = self.position.y - (slope * self.position.x);
        SlopeIntercept { slope, intercept }
    }

    fn time_on_path_x_y(&self, position: &Position) -> f64 {
        // let real_x = vectorX * time + positionX
        // time = (realX - positionX)/vectorX
        let time = (position.x - self.position.x) / self.velocity.x;
        time
    }

    fn intersection_point_x_y(&self, other: &Hailstone) -> HailstoneCollision {
        let us = self.slope_intercept_form();
        let them = other.slope_intercept_form();
        let a = us.slope;
        let c = us.intercept;
        let b = them.slope;
        let d = them.intercept;
        if a == b {
            HailstoneCollision::NoCollision
        } else {
            // We have a collision! Is it in the future though?
            let x = (d - c) / (a - b);
            let y = a * ((d - c) / (a - b)) + c;
            let intersection = Position { x, y, z: 1.0 };
            let us_time = self.time_on_path_x_y(&intersection);
            let their_time = other.time_on_path_x_y(&intersection);
            if us_time > 0.0 && their_time > 0.0 {
                HailstoneCollision::Future(intersection)
            } else {
                HailstoneCollision::Past(intersection)
            }
        }
    }
}

pub struct Range {
    lower: f64,
    upper: f64,
}

impl Range {
    pub fn new(lower: f64, upper: f64) -> Self {
        Self { lower, upper }
    }
}

pub fn part1(s: &str, range: &Range) -> u64 {
    let hailstones: Vec<Hailstone> = s.lines().map(|line| line.parse().unwrap()).collect();
    let future_path_intersections = hailstones
        .iter()
        .tuple_combinations()
        .map(|(us, them)| us.intersection_point_x_y(them));
    let count = future_path_intersections
        .filter(|intersection| match &intersection {
            HailstoneCollision::Future(position) => {
                position.x >= range.lower
                    && position.x <= range.upper
                    && position.y >= range.lower
                    && position.y <= range.upper
            }
            _ => false,
        })
        .count();
    count.try_into().unwrap()
}

pub fn part2(s: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#########;

    #[test]
    fn test_given_1() {
        let range = Range {
            lower: 7.0,
            upper: 27.0,
        };
        assert_eq!(part1(GIVEN_INPUT, &range), 2)
    }

    #[test]
    fn test_actual_1() {
        let real_range = Range::new(200000000000000.0, 400000000000000.0);
        assert_eq!(part1(REAL_INPUT, &real_range), 25810)
    }

    #[test]
    #[ignore]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 167409079868000)
    }
}
