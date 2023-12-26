use std::{convert::Infallible, str::FromStr, time::Instant};

fn main() {
    let input = include_str!("../input.txt");
    let start1 = Instant::now();
    let part1_val = part1(input);
    let time1 = start1.elapsed();
    println!("Part 1: {} in {:?}", part1_val, time1);
    let start2 = Instant::now();
    let part2_val = part2(input);
    let time2 = start2.elapsed();
    println!("Part 2: {} in {:?}", part2_val, time2);
}

fn part1(s: &str) -> i64 {
    let cards: Vec<Card> = s.lines().map(|s| s.parse().unwrap()).collect();
    cards
        .iter()
        .map(|c| {
            let winning_count: i64 = c.winning_numbers().len().try_into().unwrap();
            let val = if winning_count > 0 {
                2_i64.pow((winning_count - 1).try_into().unwrap())
            } else {
                0
            };
            val
        })
        .sum()
}

fn part2(s: &str) -> i64 {
    let cards: Vec<Card> = s.lines().map(|s| s.parse().unwrap()).collect();
    let mut won_cards: Vec<i64> = vec![0; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let won_card_count = won_cards[i];
        let total_card_count = 1 + won_card_count;
        let new_cards_won = card.winning_numbers().len();
        for j in 1..=new_cards_won {
            won_cards[i + j] = won_cards[i + j] + (1 * total_card_count);
        }
    }
    let card_count: i64 = cards.len().try_into().unwrap();
    card_count + won_cards.iter().sum::<i64>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    winning_numbers: Vec<u64>,
    my_numbers: Vec<u64>,
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_card, rest) = s.split_once(":").unwrap();
        let (winning_numbers_str, my_numbers_str) = rest.split_once("|").unwrap();
        let winning_numbers = winning_numbers_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|c| c.trim().parse().unwrap())
            .collect();
        let my_numbers = my_numbers_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|c| c.trim().parse().unwrap())
            .collect();

        Ok(Card {
            winning_numbers,
            my_numbers,
        })
    }
}

impl Card {
    fn winning_numbers(&self) -> Vec<u64> {
        self.my_numbers
            .iter()
            .filter(|my| self.winning_numbers.iter().any(|other| *my == other))
            .map(|i| i.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r##"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"##;

    #[test]
    fn parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_card = Card {
            winning_numbers: vec![41, 48, 83, 86, 17],
            my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        let actual_card: Card = input.parse().unwrap();

        assert_eq!(expected_card, actual_card);
    }
    #[test]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 13)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT), 30)
    }
}
