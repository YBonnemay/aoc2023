use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

fn process_input(inputs: Vec<String>) -> u32 {
    inputs
        .iter()
        .map(|input| {
            let sequences = input.split(',').collect_vec();
            process_sequences(&sequences)
        })
        .sum()
}

fn process_sequence(sequence: &str) -> u32 {
    let mut current_sum = 0;
    sequence.chars().for_each(|ch| {
        current_sum += ch as u32;
        current_sum *= 17;
        current_sum %= 256;
    });
    current_sum
}

fn process_sequences(sequences: &Vec<&str>) -> u32 {
    sequences
        .iter()
        .map(|sequence| process_sequence(sequence))
        .sum()
}

pub fn run() {
    let input = "./days/day15/example.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day15 done with result {result}.");
}
