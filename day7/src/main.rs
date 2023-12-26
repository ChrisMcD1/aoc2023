use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
    time::Instant,
};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}
impl Card {
    fn to_value(&self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::One => 1,
            Card::Joker => 0,
        }
    }
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Card::Ace,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => Card::Joker,
            "T" => Card::Ten,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two,
            "1" => Card::One,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
    strength: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type_comp = self.strength.partial_cmp(&other.strength).unwrap();
        Some(match hand_type_comp {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.to_value().partial_cmp(&b.to_value()).unwrap())
                .find(|&cmp| cmp != Ordering::Equal)
                .unwrap(),
        })
    }
}

fn make_type(cards: &Vec<Card>) -> HandType {
    let grouped_cards = group_cards(cards);
    let joker_count: u8 = *grouped_cards.get(&Card::Joker).unwrap_or(&0);
    let mut counts_without_joker: Vec<u8> = grouped_cards
        .iter()
        .filter(|&(card, _)| card != &Card::Joker)
        .map(|(_, &count)| count)
        .collect();

    counts_without_joker.sort();
    counts_without_joker.reverse();

    let highest = counts_without_joker.get(0);
    let second = counts_without_joker.get(1);
    let highest_with_jokers = highest.unwrap_or(&0) + joker_count;
    match (highest_with_jokers, second) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, Some(2)) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, Some(2)) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        (1, _) => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn group_cards(cards: &Vec<Card>) -> HashMap<Card, u8> {
    let mut cards_grouped: HashMap<Card, u8> = HashMap::new();
    for card in cards {
        let current_card_count = cards_grouped.get(card);
        let updated_count = match current_card_count {
            Some(count) => count + 1,
            None => 1,
        };
        cards_grouped.insert(card.clone(), updated_count);
    }
    cards_grouped
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_value().cmp(&other.to_value()))
    }
}

impl HandType {
    fn to_value(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 10,
            HandType::FourOfAKind => 9,
            HandType::FullHouse => 8,
            HandType::ThreeOfAKind => 7,
            HandType::TwoPair => 6,
            HandType::OnePair => 5,
            HandType::HighCard => 4,
        }
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").unwrap();
        let cards: Vec<Card> = cards
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let bid: i64 = bid.parse().unwrap();
        Ok(Hand {
            bid,
            strength: make_type(&cards),
            cards,
        })
    }
}

#[derive(Debug, Clone)]
struct Hands(Vec<Hand>);

impl FromStr for Hands {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands: Vec<Hand> = s.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Hands(hands))
    }
}

fn part1(s: &str) -> i64 {
    let mut hands: Hands = s.parse().unwrap();
    hands.0.sort();
    println!("{:?}", hands);
    let wininngs: i64 = hands
        .0
        .iter()
        .enumerate()
        .map(|(rank, card)| -> i64 {
            let rank: i64 = rank.try_into().unwrap();
            (rank + 1) * card.bid
        })
        .sum();
    wininngs
}

fn part2(s: &str) -> i64 {
    let mut hands: Hands = s.parse().unwrap();
    hands.0.sort();
    println!("{:?}", hands);
    let wininngs: i64 = hands
        .0
        .iter()
        .enumerate()
        .map(|(rank, card)| -> i64 {
            let rank: i64 = rank.try_into().unwrap();
            (rank + 1) * card.bid
        })
        .sum();
    wininngs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_kind_parse() {
        let input = "AAAAA 123";
        let expected_hand = Hand {
            cards: vec![Card::Ace; 5],
            bid: 123,
            strength: HandType::FiveOfAKind,
        };

        let actual_hand: Hand = input.parse().unwrap();

        assert_eq!(expected_hand, actual_hand);
    }

    #[test]
    fn two_pair_parse() {
        let input = "AK3AK 123";
        let expected_hand = Hand {
            cards: vec![Card::Ace, Card::King, Card::Three, Card::Ace, Card::King],
            bid: 123,
            strength: HandType::TwoPair,
        };

        let actual_hand: Hand = input.parse().unwrap();

        assert_eq!(expected_hand, actual_hand);
    }

    const SAMPLE_INPUT: &str = r##"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"##;

    #[test]
    #[ignore]
    fn test_given_1() {
        assert_eq!(part1(SAMPLE_INPUT), 6440)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(SAMPLE_INPUT), 5905)
    }
}
