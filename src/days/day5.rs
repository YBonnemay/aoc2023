use crate::utils::input_process::{input_to_lines, line_to_ints};
use std::ops::Range;

#[derive(Debug, Clone)]
struct Map {
    range: Range<i64>,
    back: i64,
}

impl Map {
    fn get_back(&self, seed: i64) -> i64 {
        seed + self.back
    }
}

fn process_seeds_range(seeds_range: Vec<Range<i64>>, map_vectors: &[Vec<Map>]) -> i64 {
    for end_value in 0..5000000000 {
        let starting_point = end_value;
        let mut current_value = end_value;
        for maps in map_vectors.iter().rev() {
            for map in maps.iter() {
                if map.range.contains(&current_value) {
                    current_value = map.get_back(current_value);
                    break;
                }
            }
        }

        if seeds_range
            .iter()
            .any(|range| range.contains(&current_value))
        {
            return starting_point;
        }
    }
    0
}

fn format_data(data: &mut Vec<String>) -> (Vec<Vec<Map>>, Vec<Range<i64>>) {
    let seeds_data = line_to_ints::<i64>(&data.remove(0), ' ');

    let seed_ranges = seeds_data
        .chunks(2)
        .map(|chunks| Range {
            start: chunks[0],
            end: chunks[0] + chunks[1] + 1,
        })
        .collect();

    let mut mapppings: Vec<Vec<_>> = Vec::new();
    for line in data {
        if line.is_empty() {
            mapppings.push(Vec::new());
            continue;
        }

        let ch = line.chars().next().expect("Err: no first char");

        if ch.is_alphabetic() {
            continue;
        }

        let current_mappings = mapppings.last_mut().expect("Err: empty mappings");
        let data = line_to_ints::<i64>(line, ' ');
        current_mappings.push(Map {
            range: Range {
                start: data[0],
                end: data[0] + data[2] + 1,
            },
            back: data[1] - data[0],
        });
    }

    (mapppings, seed_ranges)
}

fn day5(input: &str) -> i64 {
    let mut data = input_to_lines(input);
    let (mapppings, seed_ranges) = format_data(&mut data);
    process_seeds_range(seed_ranges, &mapppings)
}

pub fn run() {
    let input = "./days/day5/input.txt";
    let part_one = day5(input);
    println!("\n day5 part one done with input {input} result part_one {part_one}");
}
