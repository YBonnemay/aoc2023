use crate::utils::input_process::input_to_lines;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Condition {
    variable: char,
    operand_char: char,
    threshold: u64,
    next_name: String,
}

// Should macro these
impl Condition {
    fn update_ranges_not_met(&self, ranges: &mut Ranges) {
        match self.variable {
            'x' => {
                if self.operand_char == '<' {
                    ranges.x.0 = self.threshold.max(ranges.x.0)
                } else {
                    ranges.x.1 = self.threshold.min(ranges.x.1)
                }
            }
            'm' => {
                if self.operand_char == '<' {
                    ranges.m.0 = self.threshold.max(ranges.m.0)
                } else {
                    ranges.m.1 = self.threshold.min(ranges.m.1)
                }
            }
            'a' => {
                if self.operand_char == '<' {
                    ranges.a.0 = self.threshold.max(ranges.a.0)
                } else {
                    ranges.a.1 = self.threshold.min(ranges.a.1)
                }
            }
            's' => {
                if self.operand_char == '<' {
                    ranges.s.0 = self.threshold.max(ranges.s.0)
                } else {
                    ranges.s.1 = self.threshold.min(ranges.s.1)
                }
            }
            _ => {}
        }
    }

    fn update_ranges_met(&self, ranges: &mut Ranges) {
        match self.variable {
            'x' => {
                if self.operand_char == '<' {
                    ranges.x.1 = (self.threshold - 1).min(ranges.x.1)
                } else {
                    ranges.x.0 = (self.threshold + 1).max(ranges.x.0)
                }
            }
            'm' => {
                if self.operand_char == '<' {
                    ranges.m.1 = (self.threshold - 1).min(ranges.m.1)
                } else {
                    ranges.m.0 = (self.threshold + 1).max(ranges.m.0)
                }
            }
            'a' => {
                if self.operand_char == '<' {
                    ranges.a.1 = (self.threshold - 1).min(ranges.a.1)
                } else {
                    ranges.a.0 = (self.threshold + 1).max(ranges.a.0)
                }
            }
            's' => {
                if self.operand_char == '<' {
                    ranges.s.1 = (self.threshold - 1).min(ranges.s.1)
                } else {
                    ranges.s.0 = (self.threshold + 1).max(ranges.s.0)
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Ranges {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Ranges {
    fn get_total(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn parse_rules(lines: &[String]) -> HashMap<String, Vec<Condition>> {
    let mut rules: HashMap<String, Vec<Condition>> = HashMap::new();
    // I still do not like regexps, mind.
    let re: Regex = Regex::new(r"^([xmas])([<>])(\d+):(\w+)$").unwrap();
    for line in lines {
        let line = line[0..line.len() - 1].to_string();
        let (name, conditions_string) = line.split_once('{').expect("Err: no {");
        let mut conditions: Vec<Condition> = vec![];
        for condition in conditions_string.split(',') {
            match re.captures(condition) {
                Some(captures) => {
                    let (_, [variable, operand, threshold, next]) = captures.extract();
                    let operand_char = operand.chars().next().expect("Err: op");
                    conditions.push(Condition {
                        variable: variable.chars().next().expect("Err: var"),
                        operand_char,
                        threshold: threshold.parse::<u64>().unwrap(),
                        next_name: next.to_string(),
                    });
                }
                None => {
                    conditions.push(Condition {
                        variable: '_',
                        operand_char: '<',
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

fn process_rule(starting_point: (&str, usize), rules: &HashMap<String, Vec<Condition>>) -> Ranges {
    let mut ranges = Ranges {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut track = vec![starting_point];
    while let Some(name) = track.pop() {
        let conditions = &rules[name.0];
        (0..=name.1).for_each(|index| {
            if index == name.1 {
                conditions[index].update_ranges_met(&mut ranges);
            } else {
                conditions[index].update_ranges_not_met(&mut ranges);
            }
        });

        // Iterating like a caveman.
        'outer: for (rule_name, rule_conditions) in rules.iter() {
            for (i, rule_condition) in rule_conditions.iter().enumerate() {
                if rule_condition.next_name == "in" {
                    break 'outer;
                }

                if rule_condition.next_name == name.0 {
                    track.push((rule_name, i));
                    break 'outer;
                }
            }
        }
    }
    ranges
}

fn process_lines(lines: &[String]) -> u64 {
    let index = lines.iter().position(|line| line.is_empty()).unwrap();
    let rules = lines[0..index].to_vec();
    let rules = parse_rules(&rules);
    let mut starting_points: Vec<(&str, usize)> = vec![];

    for (name, conditions) in rules.iter() {
        for (i, condition) in conditions.iter().enumerate() {
            if condition.next_name == *"A" {
                starting_points.push((name, i));
            }
        }
    }

    starting_points
        .iter()
        .map(|starting_point| process_rule(*starting_point, &rules).get_total())
        .sum()
}

pub fn run() {
    let input = "./days/day19/input.txt";
    let lines = input_to_lines(input);
    let result = process_lines(&lines);
    println!("\n day19 done with result {result}.");
}
