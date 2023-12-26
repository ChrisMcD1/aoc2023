use std::{fs, ops::Index, str::FromStr};

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(s: &str) -> u64 {
    s.lines()
        .map(|s| s.parse::<Game>().unwrap())
        .filter(game_possible)
        .map(|g| g.id)
        .sum()
}

fn part2(s: &str) -> u64 {
    s.lines()
        .map(|s| s.parse::<Game>().unwrap())
        .map(|g| game_power(&g))
        .sum()
}

fn round_possible(round: &Round) -> bool {
    round.blue <= 14 && round.green <= 13 && round.red <= 12
}

fn game_possible(game: &Game) -> bool {
    game.rounds.iter().all(round_possible)
}

fn game_power(game: &Game) -> u64 {
    let min_red = game.rounds.iter().map(|r| r.red).max().unwrap();
    let min_green = game.rounds.iter().map(|r| r.green).max().unwrap();
    let min_blue = game.rounds.iter().map(|r| r.blue).max().unwrap();
    min_red * min_blue * min_green
}

struct Round {
    pub blue: u64,
    pub red: u64,
    pub green: u64,
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors_str = s.split(",");
        let mut newGuy = Self {
            blue: 0,
            red: 0,
            green: 0,
        };
        for color_str in colors_str {
            let (index_str, color) = color_str.trim().split_once(" ").unwrap();
            let index: u64 = index_str.parse().unwrap();
            match color {
                "blue" => newGuy.blue = index,
                "red" => newGuy.red = index,
                "green" => newGuy.green = index,
                _ => unreachable!(),
            }
        }
        Ok(newGuy)
    }
}

struct Game {
    id: u64,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_bit, rounds_bit) = s.split_once(":").unwrap();
        let (_, index_str) = game_bit.split_once(" ").unwrap();
        let index: u64 = index_str.parse().unwrap();

        let rounds: Vec<Round> = rounds_bit.split(";").map(|s| s.parse().unwrap()).collect();
        Ok(Game { id: index, rounds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green ";
        assert_eq!(part1(input), 8)
    }

    #[test]
    fn test_given_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green ";
        assert_eq!(part2(input), 2286)
    }
}
