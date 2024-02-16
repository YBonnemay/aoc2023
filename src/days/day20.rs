use crate::utils::input_process::input_to_lines;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleId {
    Broadcaster,
    Flipflop,
    Conjunction,
    Output,
}

#[derive(Debug, Clone)]
struct Module {
    id: ModuleId,
    name: String,
    destinations: Vec<String>,
    incomings: HashMap<String, Pulse>,
    current_state: Pulse,
}

impl Module {
    fn on_pulse(&mut self, name_from: &String, pulse: Pulse) -> VecDeque<(String, Pulse, String)> {
        let name = self.name.clone();
        let tt: VecDeque<(String, Pulse, String)> = match &self.id {
            ModuleId::Broadcaster => self
                .destinations
                .iter()
                .map(|destination| (name.clone(), pulse, destination.clone()))
                .collect(),
            ModuleId::Flipflop => {
                if pulse == Pulse::Low {
                    self.current_state = match self.current_state {
                        Pulse::High => Pulse::Low,
                        Pulse::Low => Pulse::High,
                    };
                    return self
                        .destinations
                        .iter()
                        .map(|destination| (name.clone(), self.current_state, destination.clone()))
                        .collect();
                } else {
                    VecDeque::new()
                }
            }
            ModuleId::Conjunction => {
                self.incomings.insert(name_from.clone(), pulse);
                let new_state = if self
                    .incomings
                    .iter()
                    .all(|(_, pulse)| *pulse == Pulse::High)
                {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                return self
                    .destinations
                    .iter()
                    .map(|destination| (name.clone(), new_state, destination.clone()))
                    .collect();
            }
            ModuleId::Output => VecDeque::new(),
        };

        tt
    }
}

fn button_press(modules_destination: &mut HashMap<String, Module>) -> (u64, u64, u64) {
    let mut events: VecDeque<(String, Pulse, String)> = VecDeque::new();
    let mut events_numbers: (u64, u64, u64) = (0, 0, 0);

    events.push_front((
        "broadcaster".to_string(),
        Pulse::Low,
        "broadcaster".to_string(),
    ));

    while let Some((name_from, pulse, name)) = events.pop_front() {
        // println!("popping {name}");
        // println!("{:?} -{:?}-> {:?}", name_from, pulse, name);

        match pulse {
            Pulse::Low => {
                events_numbers.0 += 1;
            }
            Pulse::High => {
                events_numbers.1 += 1;
            }
        }

        if let Some(module) = modules_destination.get_mut(&name) {
            let tt = &mut module.on_pulse(&name_from, pulse);
            events.append(tt);
        } else if pulse == Pulse::Low {
            events_numbers.2 += 1;
        };
    }
    events_numbers
}

fn process_lines(lines: &Vec<String>) -> u64 {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in lines {
        let parts = line.split("->").collect_vec();
        let destinations = parts.get(1).expect("Err: no destinations");
        let destinations = destinations
            .split(',')
            .map(|destination| destination.trim().to_owned())
            .collect_vec();
        let lhs = (*parts.first().expect("Err: no source")).trim();

        let id = match lhs.chars().next().expect("Err: no first character") {
            'b' => ModuleId::Broadcaster,
            '%' => ModuleId::Flipflop,
            '&' => ModuleId::Conjunction,
            'o' => ModuleId::Output,
            _ => {
                panic!("Err: unmanaged line")
            }
        };

        let name: String = lhs.chars().filter(|ch| ch.is_alphanumeric()).collect();
        modules.insert(
            name.clone(),
            Module {
                destinations,
                id,
                name,
                current_state: Pulse::Low,
                incomings: HashMap::new(),
            },
        );
    }

    let mut modules_destination = modules.clone();
    for (name, module) in modules.iter() {
        for destination in &module.destinations {
            if let Some(module_destination) = modules_destination.get_mut(destination) {
                module_destination
                    .incomings
                    .insert(name.clone(), Pulse::Low);
            }
        }
    }

    let mut output_incoming = HashMap::new();
    output_incoming.insert("con".to_string(), Pulse::Low);

    modules_destination.insert(
        "output".to_string(),
        Module {
            destinations: vec![],
            id: ModuleId::Output,
            name: "output".to_string(),
            current_state: Pulse::Low,
            incomings: output_incoming,
        },
    );

    let mut result: (u64, u64) = (0, 0);
    for _i in 1..=1000 {
        let (low, high, rx) = button_press(&mut modules_destination);
        result.0 += low;
        result.1 += high;
        if rx > 0 {
            break;
        }
    }

    result.0 * result.1
}

pub fn run() {
    let input = "./days/day20/input.txt";
    let lines = input_to_lines(input);
    let result = process_lines(&lines);
    println!("\n day19 done with result {result}.");
}
