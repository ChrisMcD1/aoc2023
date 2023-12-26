use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl FromStr for Category {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::ExtremelyCool,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => unreachable!(),
        })
    }
}

type WorkflowName = String;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflows(HashMap<WorkflowName, Workflow>);

impl FromStr for Workflows {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse::<WorkflowRow>().unwrap())
                .map(|row| (row.0, row.1))
                .collect(),
        ))
    }
}

impl Workflows {
    pub fn get_workflow(&self, name: &str) -> &Workflow {
        self.0
            .get(name)
            .expect("Better not get any bad workflow names!")
    }
    pub fn find_end_state(&self, part: &Part) -> &EndingState {
        let mut workflow_name = "in";
        loop {
            let workflow = self.get_workflow(workflow_name);
            let next_destination = workflow.find_destination(part);
            match next_destination {
                Position::EndingState(state) => return state,
                Position::NextWorkflow(w) => workflow_name = w,
            }
        }
    }

    pub fn run_block_to_ending_states(&self, block: RatingBlock) -> Vec<RatingBlock> {
        match block.position {
            Position::EndingState(_) => vec![block],
            Position::NextWorkflow(ref workflow_name) => {
                let moved_blocks = self.get_workflow(workflow_name).split_and_move_block(block);
                moved_blocks
                    .into_iter()
                    .map(|block| self.run_block_to_ending_states(block))
                    .flatten()
                    .collect()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Range {
    lowest: u64,
    highest: u64,
}

impl Range {
    pub fn new(lowest: u64, highest: u64) -> Self {
        Self { lowest, highest }
    }
    pub fn element_count(&self) -> u64 {
        ((self.highest - self.lowest) + 1).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct RatingBlock {
    position: Position,
    extremely_cool: Range,
    musical: Range,
    aerodynamic: Range,
    shiny: Range,
}

impl RatingBlock {
    pub fn move_to_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }
    pub fn set_range(mut self, category: Category, range: Range) -> Self {
        match category {
            Category::ExtremelyCool => self.extremely_cool = range,
            Category::Musical => self.musical = range,
            Category::Aerodynamic => self.aerodynamic = range,
            Category::Shiny => self.shiny = range,
        }
        self
    }
    pub fn parts_in_block(&self) -> u64 {
        self.extremely_cool.element_count()
            * self.musical.element_count()
            * self.aerodynamic.element_count()
            * self.shiny.element_count()
    }
    pub fn get_category(&self, category: &Category) -> &Range {
        match category {
            Category::ExtremelyCool => &self.extremely_cool,
            Category::Musical => &self.musical,
            Category::Aerodynamic => &self.aerodynamic,
            Category::Shiny => &self.shiny,
        }
    }

    pub fn apply_rule(self, rule: &Rule) -> SplitResult {
        match rule {
            Rule::FallThrough(destination) => {
                SplitResult::new(Some(self.move_to_position(destination.clone())), None)
            }
            Rule::Split(rule) => self.split(rule),
        }
    }

    pub fn split(self, split_rule: &SplitRule) -> SplitResult {
        let self_range = self.get_category(&split_rule.category);
        let amount = split_rule.amount;
        match split_rule.split_direction {
            SplitDirection::LessThan => {
                if amount > self_range.highest {
                    let moved = self.move_to_position(split_rule.destination.clone());
                    SplitResult::new(Some(moved), None)
                } else if amount <= self_range.highest && amount > self_range.lowest {
                    let lower_range = Range::new(self_range.lowest, amount - 1);
                    let upper_range = Range::new(amount, self_range.highest);
                    let lower_moved: RatingBlock = self
                        .clone()
                        .set_range(split_rule.category.clone(), lower_range)
                        .move_to_position(split_rule.destination.clone());
                    let upper_not_moved: RatingBlock =
                        self.set_range(split_rule.category.clone(), upper_range);
                    SplitResult::new(Some(lower_moved), Some(upper_not_moved))
                } else {
                    SplitResult::new(None, Some(self))
                }
            }
            SplitDirection::GreaterThan => {
                if amount < self_range.lowest {
                    let moved = self.move_to_position(split_rule.destination.clone());
                    SplitResult::new(Some(moved), None)
                } else if amount >= self_range.lowest && amount < self_range.highest {
                    let lower_range = Range::new(self_range.lowest, amount);
                    let upper_range = Range::new(amount + 1, self_range.highest);
                    let lower_not_moved: RatingBlock = self
                        .clone()
                        .set_range(split_rule.category.clone(), lower_range);
                    let upper_moved: RatingBlock = self
                        .set_range(split_rule.category.clone(), upper_range)
                        .move_to_position(split_rule.destination.clone());
                    SplitResult::new(Some(upper_moved), Some(lower_not_moved))
                } else {
                    SplitResult::new(None, Some(self))
                }
            }
        }
    }
}

struct SplitResult {
    moved_block: Option<RatingBlock>,
    not_moved_block: Option<RatingBlock>,
}

impl SplitResult {
    pub fn new(moved: Option<RatingBlock>, not_moved: Option<RatingBlock>) -> Self {
        Self {
            moved_block: moved,
            not_moved_block: not_moved,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Position {
    EndingState(EndingState),
    NextWorkflow(WorkflowName),
}

impl FromStr for Position {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::EndingState(EndingState::Accepted),
            "R" => Self::EndingState(EndingState::Rejected),
            other => Self::NextWorkflow(other.to_owned()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SplitDirection {
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SplitRule {
    category: Category,
    split_direction: SplitDirection,
    amount: u64,
    destination: Position,
}

impl SplitRule {
    fn new(
        category: Category,
        split_direction: SplitDirection,
        amount: u64,
        destination: Position,
    ) -> Self {
        Self {
            category,
            split_direction,
            amount,
            destination,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    Split(SplitRule),
    FallThrough(Position),
}

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(":") {
            Some((condition, destination)) => {
                let destination: Position = destination.parse().unwrap();
                let category: Category = condition[0..1].parse().unwrap();
                let amount: u64 = condition[2..].parse().unwrap();
                let split_direction: SplitDirection = if condition.contains('<') {
                    SplitDirection::LessThan
                } else {
                    SplitDirection::GreaterThan
                };
                Rule::Split(SplitRule::new(
                    category,
                    split_direction,
                    amount,
                    destination,
                ))
            }
            None => Rule::FallThrough(s.parse::<Position>().unwrap()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules: Vec<Rule> = s.split(",").map(|a| a.parse().unwrap()).collect();
        Ok(Self { rules })
    }
}

impl Workflow {
    fn split_and_move_block(&self, block: RatingBlock) -> Vec<RatingBlock> {
        self.rules
            .iter()
            .fold(
                (Vec::new(), Some(block)),
                |(mut output, not_moved_opt), rule| match not_moved_opt {
                    None => (output, None),
                    Some(not_moved) => {
                        let rule_result = not_moved.apply_rule(rule);
                        match rule_result.moved_block {
                            Some(moved) => output.push(moved),
                            None => {}
                        }
                        (output, rule_result.not_moved_block)
                    }
                },
            )
            .0
    }

    fn find_destination(&self, part: &Part) -> &Position {
        self.rules
            .iter()
            .find_map(|rule| match rule {
                Rule::Split(split_rule) => match split_rule.split_direction {
                    SplitDirection::LessThan => {
                        part.get_category(&split_rule.category) < split_rule.amount
                    }
                    SplitDirection::GreaterThan => {
                        part.get_category(&split_rule.category) > split_rule.amount
                    }
                }
                .then_some(&split_rule.destination),
                Rule::FallThrough(destination) => Some(destination),
            })
            .expect("Must match a rule! There is a fallthrough for a reason")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct WorkflowRow(WorkflowName, Workflow);

impl FromStr for WorkflowRow {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once("{").unwrap();
        let rest_clean = rest.trim_end_matches("}");
        let workflow: Workflow = rest_clean.parse().unwrap();
        Ok(WorkflowRow(name.to_owned(), workflow))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Part {
    extremely_cool: u64,
    musical: u64,
    aerodynamic: u64,
    shiny: u64,
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_num(s: &str) -> u64 {
            let (_, num) = s.split_once("=").unwrap();
            num.parse().unwrap()
        }

        let trimmed = s.trim_start_matches("{").trim_end_matches("}");
        let parts: Vec<&str> = trimmed.split(",").collect();
        let extremely_cool = extract_num(parts[0]);
        let musical = extract_num(parts[1]);
        let aerodynamic = extract_num(parts[2]);
        let shiny = extract_num(parts[3]);
        Ok(Self {
            extremely_cool,
            musical,
            aerodynamic,
            shiny,
        })
    }
}

impl Part {
    pub fn score(&self) -> u64 {
        self.extremely_cool + self.musical + self.aerodynamic + self.shiny
    }
    pub fn get_category(&self, category: &Category) -> u64 {
        match category {
            Category::ExtremelyCool => self.extremely_cool,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum EndingState {
    Accepted,
    Rejected,
}

pub fn part1(s: &str) -> u64 {
    let (workflow_section, part_section) = s.split_once("\n\n").unwrap();
    let workflows: Workflows = workflow_section.parse().unwrap();
    let parts: Vec<Part> = part_section
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    parts
        .iter()
        .filter_map(|part| match workflows.find_end_state(part) {
            EndingState::Accepted => Some(part.score()),
            EndingState::Rejected => None,
        })
        .sum()
}

pub fn part2(s: &str) -> u64 {
    let (workflow_section, _part_section) = s.split_once("\n\n").unwrap();
    let workflows: Workflows = workflow_section.parse().unwrap();
    let block = RatingBlock {
        position: Position::NextWorkflow("in".to_string()),
        extremely_cool: Range {
            lowest: 1,
            highest: 4000,
        },
        musical: Range {
            lowest: 1,
            highest: 4000,
        },
        aerodynamic: Range {
            lowest: 1,
            highest: 4000,
        },
        shiny: Range {
            lowest: 1,
            highest: 4000,
        },
    };

    workflows
        .run_block_to_ending_states(block)
        .into_iter()
        // .inspect(|block| println!("ending block: {:?}", block))
        .filter(|block| block.position == Position::EndingState(EndingState::Accepted))
        .map(|block| block.parts_in_block())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 19114)
    }

    #[test]
    fn parse_workflow_row() {
        use Category::*;
        use Rule::*;
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let expected = WorkflowRow(
            "px".to_owned(),
            Workflow {
                rules: vec![
                    Split(SplitRule::new(
                        Aerodynamic,
                        SplitDirection::LessThan,
                        2006,
                        Position::NextWorkflow("qkq".to_owned()),
                    )),
                    Split(SplitRule::new(
                        Musical,
                        SplitDirection::GreaterThan,
                        2090,
                        Position::EndingState(EndingState::Accepted),
                    )),
                    FallThrough(Position::NextWorkflow("rfg".to_owned())),
                ],
            },
        );

        let actual: WorkflowRow = input.parse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn split_once_less_than() {
        let rating_block = RatingBlock {
            position: Position::NextWorkflow("in".to_owned()),
            extremely_cool: Range::new(0, 10),
            musical: Range::new(0, 10),
            aerodynamic: Range::new(0, 10),
            shiny: Range::new(0, 10),
        };
        let workflow = Workflow {
            rules: vec![
                Rule::Split(SplitRule::new(
                    Category::Aerodynamic,
                    SplitDirection::LessThan,
                    5,
                    Position::NextWorkflow("moved".to_owned()),
                )),
                Rule::FallThrough(Position::NextWorkflow("fallthrough".to_owned())),
            ],
        };

        let expected_blocks: HashSet<_> = vec![
            RatingBlock {
                position: Position::NextWorkflow("fallthrough".to_owned()),
                extremely_cool: Range::new(0, 10),
                musical: Range::new(0, 10),
                aerodynamic: Range::new(5, 10),
                shiny: Range::new(0, 10),
            },
            RatingBlock {
                position: Position::NextWorkflow("moved".to_owned()),
                extremely_cool: Range::new(0, 10),
                musical: Range::new(0, 10),
                aerodynamic: Range::new(0, 4),
                shiny: Range::new(0, 10),
            },
        ]
        .into_iter()
        .collect();

        let actual_blocks: HashSet<_> = workflow
            .split_and_move_block(rating_block)
            .into_iter()
            .collect();

        assert_eq!(expected_blocks, actual_blocks);
    }

    #[test]
    fn parts_in_block() {
        let rating_block = RatingBlock {
            position: Position::NextWorkflow("in".to_owned()),
            extremely_cool: Range::new(1, 10),
            musical: Range::new(1, 10),
            aerodynamic: Range::new(1, 10),
            shiny: Range::new(1, 10),
        };
        let expected_count: u64 = 10_u64.pow(4);

        let actual_count = rating_block.parts_in_block();

        assert_eq!(expected_count, actual_count);
    }

    #[test]
    fn split_once_greater() {
        let rating_block = RatingBlock {
            position: Position::NextWorkflow("in".to_owned()),
            extremely_cool: Range::new(0, 10),
            musical: Range::new(0, 10),
            aerodynamic: Range::new(0, 10),
            shiny: Range::new(0, 10),
        };
        let workflow = Workflow {
            rules: vec![
                Rule::Split(SplitRule::new(
                    Category::Aerodynamic,
                    SplitDirection::GreaterThan,
                    5,
                    Position::NextWorkflow("moved".to_owned()),
                )),
                Rule::FallThrough(Position::NextWorkflow("fallthrough".to_owned())),
            ],
        };

        let expected_blocks: HashSet<RatingBlock> = vec![
            RatingBlock {
                position: Position::NextWorkflow("moved".to_owned()),
                extremely_cool: Range::new(0, 10),
                musical: Range::new(0, 10),
                aerodynamic: Range::new(6, 10),
                shiny: Range::new(0, 10),
            },
            RatingBlock {
                position: Position::NextWorkflow("fallthrough".to_owned()),
                extremely_cool: Range::new(0, 10),
                musical: Range::new(0, 10),
                aerodynamic: Range::new(0, 5),
                shiny: Range::new(0, 10),
            },
        ]
        .into_iter()
        .collect();

        let actual_blocks: HashSet<RatingBlock> = workflow
            .split_and_move_block(rating_block)
            .into_iter()
            .collect();

        assert_eq!(expected_blocks, actual_blocks);
    }

    #[test]
    fn move_fallthrough() {
        let rating_block = RatingBlock {
            position: Position::NextWorkflow("in".to_owned()),
            extremely_cool: Range::new(0, 10),
            musical: Range::new(0, 10),
            aerodynamic: Range::new(0, 10),
            shiny: Range::new(0, 10),
        };
        let workflow = Workflow {
            rules: vec![Rule::FallThrough(Position::NextWorkflow(
                "fallthrough".to_owned(),
            ))],
        };

        let expected_blocks: HashSet<RatingBlock> = vec![RatingBlock {
            position: Position::NextWorkflow("fallthrough".to_owned()),
            extremely_cool: Range::new(0, 10),
            musical: Range::new(0, 10),
            aerodynamic: Range::new(0, 10),
            shiny: Range::new(0, 10),
        }]
        .into_iter()
        .collect();

        let actual_blocks: HashSet<RatingBlock> = workflow
            .split_and_move_block(rating_block)
            .into_iter()
            .collect();

        assert_eq!(expected_blocks, actual_blocks);
    }

    #[test]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 167409079868000)
    }

    #[test]
    fn test_real_2() {
        assert_eq!(part2(REAL_INPUT), 131550418841958)
    }
}
