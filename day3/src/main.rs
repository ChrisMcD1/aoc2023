use std::{convert::Infallible, fs, iter::FlatMap, ops::Index, str::FromStr, time::Instant};

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt").unwrap();
    let start1 = Instant::now();
    let part1_val = part1(&input);
    let time1 = start1.elapsed();
    println!("Part 1: {} in {:?}", part1_val, time1);
    let start2 = Instant::now();
    let part2_val = part2(&input);
    let time2 = start2.elapsed();
    println!("Part 2: {} in {:?}", part2_val, time2);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position {
    row: i64,
    col: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Symbol {
    position: Position,
    char: char,
}

impl Symbol {
    fn into_gear(&self, numbers: &[Number]) -> Option<Gear> {
        if self.char == '*' {
            let adjacent_numbers: Vec<&Number> = numbers
                .iter()
                .filter(|n| number_and_symbol_are_adjacent(n, self))
                .collect();
            if adjacent_numbers.len() != 2 {
                None
            } else {
                Some(Gear {
                    position: self.position.clone(),
                    part1: adjacent_numbers[0].clone(),
                    part2: adjacent_numbers[1].clone(),
                })
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Gear {
    position: Position,
    part1: Number,
    part2: Number,
}

impl Gear {
    fn ratio(&self) -> i64 {
        self.part1.value * self.part2.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Number {
    value: i64,
    start: Position,
    end: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn line_to_schematic(line: &str, row: i64) -> Schematic {
    let mut new_schematic = Schematic {
        numbers: vec![],
        symbols: vec![],
    };
    let mut chars_enumerated = line.chars().enumerate().peekable();
    while let Some((col, char)) = chars_enumerated.next() {
        if char == '.' {
            continue;
        } else if char.is_numeric() {
            let mut number_string = char.to_string();
            let start_col: i64 = col.try_into().unwrap();
            let mut end_col = start_col;
            while let Some((new_end, new_char)) = chars_enumerated.peek() {
                if !new_char.is_numeric() {
                    break;
                }
                end_col = (*new_end).try_into().unwrap();
                number_string.push(*new_char);
                chars_enumerated.next();
            }
            new_schematic.numbers.push(Number {
                value: number_string.parse().unwrap(),
                start: Position {
                    row,
                    col: start_col,
                },
                end: Position { row, col: end_col },
            });
        } else {
            // We must be a symbol at this point
            new_schematic.symbols.push(Symbol {
                char,
                position: Position {
                    row,
                    col: col.try_into().unwrap(),
                },
            })
        }
    }
    new_schematic
}

fn combine_schematic(s1: Schematic, s2: Schematic) -> Schematic {
    Schematic {
        numbers: [s1.numbers, s2.numbers].concat(),
        symbols: [s1.symbols, s2.symbols].concat(),
    }
}

fn flatten_schematic_slow(schematics: Vec<Schematic>) -> Schematic {
    let mut schematic_list = schematics.into_iter();
    let first = schematic_list.next().unwrap();
    let schematic: Schematic = schematic_list.fold(first, |acc, x| combine_schematic(acc, x));
    schematic
}

fn flatten_schematic(schematics: Vec<Schematic>) -> Schematic {
    let mut new_schematic = Schematic {
        numbers: vec![],
        symbols: vec![],
    };
    for mut schematic in schematics.into_iter() {
        new_schematic.numbers.append(&mut schematic.numbers);
        new_schematic.symbols.append(&mut schematic.symbols);
    }
    new_schematic
}

impl FromStr for Schematic {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = Instant::now();
        let schematics: Vec<Schematic> = s
            .lines()
            .enumerate()
            .map(|(index, line)| line_to_schematic(line, index.try_into().unwrap()))
            .collect();
        println!("Made the individual schematics in {:?}", start.elapsed());
        let schematic = flatten_schematic(schematics);
        println!("Folded in {:?}", start.elapsed());
        Ok(schematic)
    }
}

fn number_and_symbol_are_adjacent(number: &Number, symbol: &Symbol) -> bool {
    (symbol.position.row >= number.start.row - 1 && symbol.position.row <= number.start.row + 1)
        && (symbol.position.col >= number.start.col - 1
            && symbol.position.col <= number.end.col + 1)
}

fn number_is_adjacent_to_any_symbol(number: &Number, symbols: &[Symbol]) -> bool {
    symbols
        .iter()
        .any(|symbol| number_and_symbol_are_adjacent(number, symbol))
}

fn part1(s: &str) -> i64 {
    let start = Instant::now();
    let schematic: Schematic = s.parse().unwrap();
    println!("Elasped {:?} to parse schematic", start.elapsed());
    let result = schematic
        .numbers
        .iter()
        .filter(|x| number_is_adjacent_to_any_symbol(x, &schematic.symbols))
        .map(|x| x.value)
        .sum();

    println!("Elapsed {:?} to do the math", start.elapsed());
    result
}

fn part2(s: &str) -> i64 {
    let schematic: Schematic = s.parse().unwrap();
    schematic
        .symbols
        .iter()
        .map(|x| x.into_gear(&schematic.numbers))
        .flatten()
        .map(|g| g.ratio())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_line_to_schematic() {
        let input = "617*.#.5..";
        let expected_schematic = Schematic {
            numbers: vec![
                Number {
                    value: 617,
                    start: Position { row: 0, col: 0 },
                    end: Position { row: 0, col: 2 },
                },
                Number {
                    value: 5,
                    start: Position { row: 0, col: 7 },
                    end: Position { row: 0, col: 7 },
                },
            ],
            symbols: vec![
                Symbol {
                    position: Position { row: 0, col: 3 },
                    char: '*',
                },
                Symbol {
                    position: Position { row: 0, col: 5 },
                    char: '#',
                },
            ],
        };

        let actual_schematic = line_to_schematic(input, 0);

        assert_eq!(expected_schematic, actual_schematic);
    }

    #[test]
    fn test_given_1() {
        let input = " 467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361)
    }

    #[test]
    fn test_given_2() {
        let input = " 467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(input), 467835)
    }
}
