use crate::utils::input_process::input_to_lines;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Direction {
    left: String,
    right: String,
}

fn process_instructions(instructions: &String, maps: &HashMap<String, Direction>) -> i32 {
    let instructions_chars: Vec<_> = instructions.chars().collect();
    let loop_size = instructions_chars.len();
    let mut zzz_found = false;
    let mut current_idx = 0;
    let mut current_value = "AAA".to_string();
    while !zzz_found {
        let next_value = match instructions_chars[current_idx % loop_size] {
            'L' => &maps[&current_value].left,
            'R' => &maps[&current_value].right,
            _ => panic!("Err no instruction given"),
        };
        zzz_found = next_value == "ZZZ";
        current_value = (*next_value).clone();
        current_idx += 1;
    }
    current_idx as i32
}

fn day8(input: &str) -> i32 {
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

    process_instructions(instructions, &maps)
}

pub fn run() {
    let input = "./days/day8/input.txt";
    let result = day8(input);
    println!("\n day8 done with result_part_one {result} ");
}
