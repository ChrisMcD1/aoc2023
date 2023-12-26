use std::{convert::Infallible, str::FromStr};

pub struct HashString {
    data: String,
}

impl HashString {
    pub fn new(str: &str) -> Self {
        Self {
            data: str.to_owned(),
        }
    }
    pub fn hash(&self) -> u32 {
        hash(&self.data)
    }
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, item| {
        let ascii_value: u32 = (item as u8).try_into().unwrap();
        ((acc + ascii_value) * 17) % 256
    })
}

pub enum Operation {
    Remove,
    Insert(u32),
}

pub struct Step {
    label: String,
    operation: Operation,
}

impl FromStr for Step {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('-') {
            let (label, _) = s.split_once('-').unwrap();
            Ok(Self {
                label: label.to_owned(),
                operation: Operation::Remove,
            })
        } else {
            let (label, focal_length) = s.split_once("=").unwrap();
            Ok(Self {
                label: label.to_owned(),
                operation: Operation::Insert(focal_length.parse().unwrap()),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lens {
    label: String,
    focal_length: u32,
}
impl Lens {
    fn new(label: String, focal_length: u32) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

impl Step {
    pub fn hash(&self) -> u32 {
        hash(&self.label)
    }
}

#[derive(Debug, Clone)]
pub struct Box {
    lenses: Vec<Lens>,
}

pub struct Boxes(Vec<Box>);

impl Boxes {
    fn process_steps(steps: &[Step]) -> Self {
        let mut boxes: Vec<Box> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Box { lenses: vec![] });
        }
        for step in steps {
            match step.operation {
                Operation::Remove => {
                    let target_box = boxes.get_mut(hash(&step.label) as usize).unwrap();
                    target_box
                        .lenses
                        .retain_mut(|lens| lens.label != step.label);
                }
                Operation::Insert(focal_length) => {
                    let target_box = boxes.get_mut(hash(&step.label) as usize).unwrap();
                    let existing_index = target_box
                        .lenses
                        .iter()
                        .enumerate()
                        .find_map(|(i, lens)| (lens.label == step.label).then_some(i));
                    let new_lens = Lens::new(step.label.to_owned(), focal_length);
                    match existing_index {
                        None => target_box.lenses.push(new_lens),
                        Some(index) => target_box.lenses[index] = new_lens,
                    }
                }
            }
        }
        Boxes(boxes)
    }
    fn calculate_power(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, box_a)| {
                let box_multiplier: u32 = (i + 1).try_into().unwrap();
                box_a
                    .lenses
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| {
                        let slot_power: u32 = (j + 1).try_into().unwrap();
                        let power: u32 = slot_power * lens.focal_length * box_multiplier;
                        power
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

pub fn part1(s: &str) -> u32 {
    s.trim()
        .split(",")
        .map(|str| HashString::new(str))
        .map(|s| s.hash())
        .sum()
}

pub fn part2(s: &str) -> u32 {
    let steps: Vec<Step> = s
        .trim()
        .split(",")
        .map(|s| s.parse::<Step>().unwrap())
        .collect();
    let boxes = Boxes::process_steps(&steps);
    boxes.calculate_power()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str =
        r#########"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 1320)
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 145)
    }
}
