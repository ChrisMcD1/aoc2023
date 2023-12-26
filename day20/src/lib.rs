use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    rc::Rc,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PulseType {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct ModuleIdentifier(Rc<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pulse {
    to: ModuleIdentifier,
    from: ModuleIdentifier,
    pulse_type: PulseType,
}

trait ModuleOutput {
    fn output_pulse_type(&mut self, input: &Pulse) -> Option<PulseType>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FlipFlop {
    state: FlipFlopState,
}

impl ModuleOutput for FlipFlop {
    fn output_pulse_type(&mut self, input: &Pulse) -> Option<PulseType> {
        match input.pulse_type {
            PulseType::Low => {
                let output = match self.state {
                    FlipFlopState::On => Some(PulseType::Low),
                    FlipFlopState::Off => Some(PulseType::High),
                };
                self.state = match self.state {
                    FlipFlopState::On => FlipFlopState::Off,
                    FlipFlopState::Off => FlipFlopState::On,
                };
                output
            }
            PulseType::High => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Conjuction {
    inputs_received: HashMap<ModuleIdentifier, PulseType>,
}

impl ModuleOutput for Conjuction {
    fn output_pulse_type(&mut self, input: &Pulse) -> Option<PulseType> {
        self.inputs_received
            .insert(input.from.clone(), input.pulse_type);

        if self
            .inputs_received
            .iter()
            .all(|(_key, pulse_type)| pulse_type == &PulseType::High)
        {
            Some(PulseType::Low)
        } else {
            Some(PulseType::High)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
    Broadcaster,
}

impl ModuleOutput for ModuleType {
    fn output_pulse_type(&mut self, input: &Pulse) -> Option<PulseType> {
        match self {
            ModuleType::FlipFlop(flip_flop) => flip_flop.output_pulse_type(input),
            ModuleType::Conjuction(conjuction) => conjuction.output_pulse_type(input),
            ModuleType::Broadcaster => Some(input.pulse_type.to_owned()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Module {
    identifier: ModuleIdentifier,
    module_type: ModuleType,
    outputs: Vec<ModuleIdentifier>,
}

impl Module {
    fn new(intermediate: &IntermediateModuleParsing, others: &[IntermediateModuleParsing]) -> Self {
        let module_type: ModuleType = match intermediate.module_type_identifier {
            ModuleTypeIdentifier::FlipFlop => ModuleType::FlipFlop(FlipFlop {
                state: FlipFlopState::Off,
            }),
            ModuleTypeIdentifier::Conjuction => {
                let inputs = others
                    .iter()
                    .filter(|other| other.outputs.contains(&intermediate.identifier))
                    .map(|other| other.identifier.clone());

                let inputs_received: HashMap<ModuleIdentifier, PulseType> = inputs
                    .map(|identifier| (identifier, PulseType::Low))
                    .collect();
                ModuleType::Conjuction(Conjuction { inputs_received })
            }
            ModuleTypeIdentifier::Broadcaster => ModuleType::Broadcaster,
        };

        Self {
            identifier: intermediate.identifier.clone(),
            module_type,
            outputs: intermediate.outputs.clone(),
        }
    }
    fn produce_pulses(&mut self, input: &Pulse) -> Vec<Pulse> {
        let output_pulse_type = self.module_type.output_pulse_type(input);

        match output_pulse_type {
            Some(pulse_type) => self
                .outputs
                .iter()
                .map(|to| Pulse {
                    to: to.clone(),
                    from: self.identifier.clone(),
                    pulse_type,
                })
                .collect(),
            None => vec![],
        }
    }
}

struct PulseCounts {
    low: u64,
    high: u64,
}

struct Modules(HashMap<ModuleIdentifier, Module>);

impl Modules {
    fn push_button(&mut self) -> Vec<Pulse> {
        let mut output: Vec<Pulse> = Vec::with_capacity(1000);
        let initial_pulse = Pulse {
            to: ModuleIdentifier("broadcaster".to_string().into()),
            from: ModuleIdentifier("button".to_string().into()),
            pulse_type: PulseType::Low,
        };
        let mut pulse_queue: VecDeque<Pulse> = VecDeque::with_capacity(1000);
        pulse_queue.push_back(initial_pulse);
        while let Some(pulse) = pulse_queue.pop_front() {
            match self.0.get_mut(&pulse.to) {
                Some(target) => {
                    let new_pulses = target.produce_pulses(&pulse);

                    for new_pulse in new_pulses {
                        pulse_queue.push_back(new_pulse);
                    }
                }
                None => (),
            };
            output.push(pulse);
        }
        output
    }
}

enum ModuleTypeIdentifier {
    FlipFlop,
    Conjuction,
    Broadcaster,
}

struct IntermediateModuleParsing {
    identifier: ModuleIdentifier,
    outputs: Vec<ModuleIdentifier>,
    module_type_identifier: ModuleTypeIdentifier,
}

impl FromStr for IntermediateModuleParsing {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("->").unwrap();
        let outputs: Vec<ModuleIdentifier> = right
            .trim()
            .split(",")
            .map(|identifier| ModuleIdentifier(identifier.trim().to_string().into()))
            .collect();
        let (module_type_identifier, identifier) = if left.contains("%") {
            (
                ModuleTypeIdentifier::FlipFlop,
                ModuleIdentifier(left.trim()[1..].to_string().into()),
            )
        } else if left.contains("&") {
            (
                ModuleTypeIdentifier::Conjuction,
                ModuleIdentifier(left.trim()[1..].to_string().into()),
            )
        } else {
            (
                ModuleTypeIdentifier::Broadcaster,
                ModuleIdentifier("broadcaster".to_string().into()),
            )
        };
        Ok(Self {
            identifier,
            outputs,
            module_type_identifier,
        })
    }
}

impl FromStr for Modules {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let intermediate_modules: Vec<IntermediateModuleParsing> =
            s.lines().map(|line| line.parse().unwrap()).collect();

        let modules: Vec<Module> = intermediate_modules
            .iter()
            .map(|intermediate| Module::new(intermediate, &intermediate_modules))
            .collect();

        let module_map: HashMap<ModuleIdentifier, Module> = modules
            .into_iter()
            .map(|module| (module.identifier.clone(), module))
            .collect();

        Ok(Self(module_map))
    }
}

pub fn part1(s: &str) -> u64 {
    let mut modules: Modules = s.parse().unwrap();
    let mut pulse_totals = PulseCounts { low: 0, high: 0 };

    for _ in 0..1000 {
        let pulses = modules.push_button();
        let lows_counts: u64 = pulses
            .iter()
            .filter(|pulse| pulse.pulse_type == PulseType::Low)
            .count()
            .try_into()
            .unwrap();
        let high_counts: u64 = pulses
            .iter()
            .filter(|pulse| pulse.pulse_type == PulseType::High)
            .count()
            .try_into()
            .unwrap();
        pulse_totals.low += lows_counts;
        pulse_totals.high += high_counts;
    }

    pulse_totals.low * pulse_totals.high
}

pub fn part2(s: &str) -> u64 {
    let mut modules: Modules = s.parse().unwrap();
    let mut press_count = 0;
    loop {
        let pulses = modules.push_button();
        press_count += 1;
        if pulses.iter().any(|pulse| {
            pulse.to == ModuleIdentifier("th".to_string().into())
                && pulse.from == ModuleIdentifier("zl".to_string().into())
                && pulse.pulse_type == PulseType::High
        }) {
            println!("zl sent a high to th at {press_count}");
        }
        if pulses.iter().any(|pulse| {
            pulse.to == ModuleIdentifier("th".to_string().into())
                && pulse.from == ModuleIdentifier("xn".to_string().into())
                && pulse.pulse_type == PulseType::High
        }) {
            println!("xn sent a high to th at {press_count}");
        }
        if pulses.iter().any(|pulse| {
            pulse.to == ModuleIdentifier("th".to_string().into())
                && pulse.from == ModuleIdentifier("qn".to_string().into())
                && pulse.pulse_type == PulseType::High
        }) {
            println!("qn sent a high to th at {press_count}");
        }
        if pulses.iter().any(|pulse| {
            pulse.to == ModuleIdentifier("th".to_string().into())
                && pulse.from == ModuleIdentifier("xf".to_string().into())
                && pulse.pulse_type == PulseType::High
        }) {
            println!("xf sent a high to th at {press_count}");
        }
        if pulses.iter().any(|pulse| {
            pulse.to == ModuleIdentifier("rx".to_string().into())
                && pulse.pulse_type == PulseType::Low
        }) {
            return press_count;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const REAL_INPUT: &str = include_str!("../input.txt");
    const GIVEN_INPUT: &str = r#########"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#########;

    const GIVEN_INPUT_2: &str = r#########"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#########;

    #[test]
    fn test_given_1() {
        assert_eq!(part1(GIVEN_INPUT), 32000000)
    }

    #[test]
    fn test_given_1_2() {
        assert_eq!(part1(GIVEN_INPUT_2), 11687500)
    }

    #[test]
    fn test_real_1() {
        assert_eq!(part1(REAL_INPUT), 856482136)
    }

    #[test]
    #[ignore]
    fn test_given_2() {
        assert_eq!(part2(GIVEN_INPUT), 167409079868000)
    }
}
