use std::collections::HashMap;

struct WordMap(HashMap<&'static str, u64>);
impl WordMap {
    /// Finds an integer corresponding to the matching function
    pub fn find_int<F>(&self, f: F) -> Option<u64>
    where
        F: Fn(&str) -> bool,
    {
        self.0
            .iter()
            .find(|(word, _)| f(word))
            .map(|(_, int)| int.to_owned())
    }
}

fn main() {
    println!("Hello, world!");
    let words_to_int: WordMap = WordMap(HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]));
    let input = include_str!("../input.txt");
    let answer: u64 = input
        .lines()
        .map(line_to_int)
        .collect::<Option<Vec<_>>>()
        .unwrap()
        .iter()
        .sum();
    println!("Part 1: {}", answer);
    let answer2: u64 = input
        .lines()
        .map(|w| line_to_int_with_words(w, &words_to_int))
        .collect::<Option<Vec<_>>>()
        .unwrap()
        .iter()
        .sum();
    println!("Part 2: {}", answer2);
}

/// looks at a string and returns the first char as a number if it is a number,
/// or the number value if it one of the words in `word_to_int`
fn parse_front_numeric_or_word(s: &str, words_to_int: &WordMap) -> Option<u64> {
    let parsed_numeric: Option<u64> = s.chars().next()?.to_string().parse().ok();
    parsed_numeric.or_else(|| words_to_int.find_int(|word| s.starts_with(word)))
}

fn get_first_with_words(s: &str, words_to_int: &WordMap) -> Option<u64> {
    s.char_indices()
        .find_map(|(i, _)| parse_front_numeric_or_word(&s[i..], words_to_int))
}

/// looks at a string and returns the first char as a number if it is a number,
/// or the number value if it one of the words in `word_to_int`
fn parse_last_numeric_or_word(s: &str, words_to_int: &WordMap) -> Option<u64> {
    let parsed_numeric: Option<u64> = s.chars().last()?.to_string().parse::<u64>().ok();
    parsed_numeric.or_else(|| words_to_int.find_int(|word| s.ends_with(word)))
}

fn get_last_with_words(s: &str, words_to_int: &WordMap) -> Option<u64> {
    s.char_indices().find_map(|(i, _)| {
        let last = s.len() - i;
        parse_last_numeric_or_word(&s[..last], words_to_int)
    })
}

fn line_to_int_with_words(s: &str, words_to_int: &WordMap) -> Option<u64> {
    let first_char = get_first_with_words(s, words_to_int)?;
    let last_char = get_last_with_words(s, words_to_int)?;

    let number = format!("{}{}", first_char, last_char);
    number.parse().ok()
}

fn line_to_int(s: &str) -> Option<u64> {
    let first_char = s.chars().find(|c| c.is_numeric())?;
    let last_char = s.chars().rev().find(|c| c.is_numeric())?;

    let number = format!("{}{}", first_char, last_char);
    number.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() -> () {
        let words_to_int = WordMap(HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]));
        let input = include_str!("../input.txt");
        let answer: u64 = input
            .lines()
            .map(line_to_int)
            .collect::<Option<Vec<_>>>()
            .unwrap()
            .iter()
            .sum();
        assert_eq!(answer, 52974);
        let answer2: u64 = input
            .lines()
            .map(|w| line_to_int_with_words(w, &words_to_int))
            .collect::<Option<Vec<_>>>()
            .unwrap()
            .iter()
            .sum();
        assert_eq!(answer2, 53340);
    }
}
