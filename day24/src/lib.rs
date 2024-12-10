use itertools::Itertools;
use std::{convert::Infallible, fmt::Debug, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
struct Position<T> {
    x: T,
    y: T,
    z: T,
}

impl Position<i64> {
    fn cross_product(&self, other: &Position<i64>) -> Position<i64> {
        Position {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    fn plane_intersection(&self, line: Position<i64>) -> Position<i64> {}
}

impl<T> FromStr for Position<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
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
struct Velocity<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> FromStr for Velocity<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
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
struct Hailstone<T> {
    position: Position<T>,
    velocity: Velocity<T>,
}

impl<T> FromStr for Hailstone<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position_string, velocity_string) = s.split_once("@").unwrap();
        let position: Position<T> = position_string.parse()?;
        let velocity: Velocity<T> = velocity_string.parse()?;
        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum HailstoneCollision<T> {
    Past(Position<T>),
    Future(Position<T>),
    NoCollision,
}

struct SlopeIntercept<T> {
    slope: T,
    intercept: T,
}

impl Hailstone<i64> {
    fn shift_relative_to(&self, other: &Hailstone<i64>) -> Hailstone<i64> {
        let new_x_position = self.position.x - other.position.x;
        let new_y_position = self.position.y - other.position.y;
        let new_z_position = self.position.z - other.position.z;

        let new_x_velocity = self.velocity.x - other.velocity.x;
        let new_y_velocity = self.velocity.y - other.velocity.y;
        let new_z_velocity = self.velocity.z - other.velocity.z;
        Hailstone {
            position: Position {
                x: new_x_position,
                y: new_y_position,
                z: new_z_position,
            },
            velocity: Velocity {
                x: new_x_velocity,
                y: new_y_velocity,
                z: new_z_velocity,
            },
        }
    }
    fn at_time(&self, time: i64) -> Position<i64> {
        Position {
            x: self.position.x + self.velocity.x * time,
            y: self.position.y + self.velocity.y * time,
            z: self.position.z + self.velocity.z * time,
        }
    }
}

impl Hailstone<f64> {
    fn slope_intercept_form(&self) -> SlopeIntercept<f64> {
        // We have x, y, position and velocity
        // (y - y1) = M(x - x1)
        let slope = self.velocity.y / self.velocity.x;
        let intercept = self.position.y - (slope * self.position.x);
        SlopeIntercept { slope, intercept }
    }

    fn time_on_path_x_y(&self, position: &Position<f64>) -> f64 {
        // let real_x = vectorX * time + positionX
        // time = (realX - positionX)/vectorX
        let time = (position.x - self.position.x) / self.velocity.x;
        time
    }

    fn intersection_point_x_y(&self, other: &Hailstone<f64>) -> HailstoneCollision<f64> {
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
    let hailstones: Vec<Hailstone<f64>> = s.lines().map(|line| line.parse().unwrap()).collect();
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
    let hailstones: Vec<Hailstone<i64>> = s.lines().map(|line| line.parse().unwrap()).collect();
    let reference = &hailstones[0];
    let plane_definer = hailstones[1].shift_relative_to(reference);
    let plane_point_1 = plane_definer.at_time(0);
    let plane_point_2 = plane_definer.at_time(1);
    let plane_normal = plane_point_1.cross_product(&plane_point_2);

    let intersection_1 = hailstones[2].shift_relative_to(reference);
    let intersection_2 = hailstones[3].shift_relative_to(reference);
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
