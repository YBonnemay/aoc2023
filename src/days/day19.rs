use crate::utils::input_process::input_to_lines;
use itertools::Itertools;
use regex::Regex;
use std::cmp::PartialOrd;
use std::collections::HashMap;

const OK: char = 'A';
const KO: char = 'R';

#[derive(Debug)]
struct Condition {
    variable: char,
    operand: fn(&u64, &u64) -> bool,
    threshold: u64,
    next_name: String,
}

#[derive(Default, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn set_var(&mut self, ch: char, threshold: u64) {
        match ch {
            'x' => {
                self.x = threshold;
            }
            'm' => {
                self.m = threshold;
            }
            'a' => {
                self.a = threshold;
            }
            's' => {
                self.s = threshold;
            }
            _ => {
                println!("AA");
            }
        }
    }

    fn get_var(&self, ch: char) -> u64 {
        match ch {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => {
                panic!("Err: unmanaged char {ch}");
            }
        }
    }

    fn get_sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_parts(lines: &[String]) -> Vec<Part> {
    let mut parts: Vec<Part> = vec![];

    for line in lines {
        let line = line[1..line.len() - 1].to_string();

        let parts_string = line.split(',');
        let mut part = Part::default();

        for parts in parts_string {
            let (variable, threshold) = parts.split_once('=').expect("Err: no equal sign");
            part.set_var(
                variable.chars().next().expect("Err: not a char"),
                threshold.parse::<u64>().expect("Err: not an u64"),
            )
        }

        parts.push(part);
    }

    parts
}

fn parse_rules(lines: &[String]) -> HashMap<String, Vec<Condition>> {
    let mut rules: HashMap<String, Vec<Condition>> = HashMap::new();
    // I still do not like regexps.
    let re: Regex = Regex::new(r"^([xmas])([<>])(\d+):(\w+)$").unwrap();
    for line in lines {
        let line = line[0..line.len() - 1].to_string();
        let (name, conditions_string) = line.split_once('{').expect("Err: no {");
        let mut conditions: Vec<Condition> = vec![];
        for condition in conditions_string.split(',') {
            match re.captures(condition) {
                Some(captures) => {
                    let (_, [variable, operand, threshold, next]) = captures.extract();
                    conditions.push(Condition {
                        variable: variable.chars().next().expect("Err: var"),
                        operand: if operand.chars().next().expect("Err: op") == '<' {
                            PartialOrd::lt
                        } else {
                            PartialOrd::gt
                        },
                        threshold: threshold.parse::<u64>().unwrap(),
                        next_name: next.to_string(),
                    });
                }
                None => {
                    conditions.push(Condition {
                        variable: '_',
                        operand: std::cmp::PartialOrd::lt,
                        threshold: 0,
                        next_name: condition.to_string(),
                    });
                }
            }
        }
        rules.insert(name.to_string(), conditions);
    }

    rules
}

fn get_next_name<'a>(part: &Part, condition: &'a Condition) -> Option<&'a String> {
    let cond_var = condition.variable;

    if cond_var == '_' {
        return Some(&condition.next_name);
    }

    let value = part.get_var(cond_var);
    let have_to_name = condition.operand;
    if have_to_name(&value, &condition.threshold) {
        return Some(&condition.next_name);
    }

    None
}

fn process_lines(lines: Vec<String>) -> u64 {
    let index = lines.iter().position(|line| line.is_empty()).unwrap();

    let rules = lines[0..index].to_vec();
    let parts = lines[{ index + 1 }..].to_vec();
    let rules = parse_rules(&rules);
    let parts = parse_parts(&parts);

    let mut steps = parts
        .iter()
        .map(|part| (part, "in".to_string()))
        .collect_vec();

    let mut oks: Vec<u64> = vec![];

    while let Some((part, name)) = steps.pop() {
        let rules = &rules[&name];

        for condition in rules {
            if let Some(next_name) = get_next_name(part, condition) {
                // println!("processing {next_name}");
                match next_name.chars().next().expect("Err: no char in name") {
                    OK => {
                        oks.push(part.get_sum());
                    }
                    KO => {}
                    _ => {
                        steps.push((part, next_name.clone()));
                    }
                }
                break;
            }
        }
    }
    oks.iter().sum()
}

pub fn run() {
    let input = "./days/day19/input.txt";
    let lines = input_to_lines(input);
    let result = process_lines(lines);
    // let result = process_input(data);
    println!("\n day19 done with result {result}.");
}
