use std::{fs, collections::HashMap};

fn main() {
    println!("Hello, world!");
    let words_to_int: HashMap<&str, u64> = HashMap::from([
           ("one", 1),
           ("two", 2),
           ("three", 3),
           ("four", 4),
           ("five", 5),
           ("six", 6),
           ("seven", 7),
           ("eight", 8),
           ("nine", 9),
    ]);
    let input = fs::read_to_string("input.txt").unwrap();
    let answer: u64 = input.lines().map(line_to_int).sum();
    println!("Part 1: {}", answer);
    let answer2: u64 = input.lines().map(|w| line_to_int_with_words(w, &words_to_int)).sum();
    println!("Part 2: {}", answer2);
}


fn get_first_with_words(s: &str, words_to_int: &HashMap<&str, u64>) -> u64 {
    let mut has_found_word = false;
    let mut mut_s = s;
    let mut value: Option<u64> = None;
    while !has_found_word {
        value = {
            let s_string = mut_s.to_string();
            if mut_s.chars().next().unwrap().is_numeric() {
                Some(mut_s.chars().next().unwrap().to_string().parse().unwrap())
            } else {
                words_to_int.iter().find(|(word, _)| s_string.starts_with(*word)).map(|(_, int)| int.to_owned())
            }
        };
        if value.is_some() {
            has_found_word = true
        } else {
            mut_s = &mut_s[1..]
        }
    }
    value.unwrap()
}

fn get_last_with_words(s: &str, words_to_int: &HashMap<&str, u64>) -> u64 {
    let mut has_found_word = false;
    let mut mut_s = s;
    let mut value: Option<u64> = None;
    while !has_found_word {
        value = {
            let s_string = mut_s.to_string();
            if mut_s.chars().last().unwrap().is_numeric() {
                Some(mut_s.chars().last().unwrap().to_string().parse().unwrap())
            } else {
                words_to_int.iter().find(|(word, _)| s_string.ends_with(*word)).map(|(_, int)| int.to_owned())
            }
        };
        if value.is_some() {
            has_found_word = true
        } else {
            let length = mut_s.len();
            mut_s = &mut_s[..length - 1]
        }
    }
    value.unwrap()
}

fn line_to_int_with_words(s: &str, words_to_int: &HashMap<&str, u64>) -> u64 {
    let first_char = get_first_with_words(s, words_to_int);
    let last_char = get_last_with_words(s, words_to_int);

    let number = format!("{}{}", first_char, last_char);
    number.parse().unwrap()
}

fn line_to_int(s: &str) -> u64 {
    let first_char = s.chars().find(|c| c.is_numeric()).unwrap();
    let last_char = s.chars().rev().find(|c| c.is_numeric()).unwrap();

    let number = format!("{}{}", first_char, last_char);
    number.parse().unwrap()
}
