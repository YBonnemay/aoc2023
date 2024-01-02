use crate::utils::input_process::input_to_lines;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Direction {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
struct State {
    idx: i64,
    value: String,
}

fn process_instructions(
    instructions: &str,
    maps: &HashMap<String, Direction>,
    state: &mut State,
) -> State {
    let instructions_chars: Vec<_> = instructions.chars().collect();
    let loop_size = instructions_chars.len();
    let mut zzz_found = false;
    let mut next_value = "".to_string();
    let State {
        idx: mut current_idx,
        value: ref mut current_value,
    } = state;

    while !zzz_found {
        next_value = match instructions_chars[(current_idx % loop_size as i64) as usize] {
            'L' => maps[current_value].left.clone(),
            'R' => maps[current_value].right.clone(),
            _ => panic!("Err no instruction given"),
        };
        zzz_found = next_value.ends_with('Z');
        *current_value = next_value.clone();
        current_idx += 1;
    }

    State {
        idx: current_idx,
        value: next_value,
    }
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn day8(input: &str) -> i64 {
    let mut data = input_to_lines(input);
    let instructions = &data.remove(0);
    data.remove(0);

    let mut maps: HashMap<String, Direction> = HashMap::new();

    for line in data.iter() {
        let alpha_values: String = line.chars().filter(|ch| ch.is_alphanumeric()).collect();
        maps.insert(
            alpha_values[0..3].to_string(),
            Direction {
                left: alpha_values[3..6].to_string(),
                right: alpha_values[6..9].to_string(),
            },
        );
    }

    // Nodes that end with an A
    let states = maps.iter().fold(Vec::new(), |mut acc, (k, _v)| {
        if k.ends_with('A') {
            let mut state = State {
                idx: 0,
                value: k.clone(),
            };
            let new_state = process_instructions(instructions, &maps, &mut state);
            acc.push(new_state.idx);
        }
        acc
    });

    lcm(&states)
}

pub fn run() {
    // Did not like this one, the description misleads.
    let input = "./days/day8/input.txt";
    let result = day8(input);
    println!("\n day8 done with result_part_one {result} ");
}
