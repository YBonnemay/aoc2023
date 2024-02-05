use std::ops::BitAnd;

use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

fn get_direction(st: &str) -> Direction {
    match st {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("Unknown Direction"),
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
struct Step {
    direction: Direction,
    number: i64,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
struct Place {
    i: i64,
    j: i64,
}

fn line_to_step(line: &String) -> Step {
    let (_, hex_direction) = line.split_once('#').expect("Err: hex not found");
    let direction = get_direction(&hex_direction[5..6]);
    let number = i64::from_str_radix(&hex_direction[0..5], 16).expect("Err: hex not number");
    Step { direction, number }
}

fn apply_step(i: i64, j: i64, step: &Step) -> Vec<Place> {
    let Step { direction, number } = step;

    match direction {
        Direction::Up => ((i - number)..=(i - 1))
            .rev()
            .map(|i| Place { i, j })
            .collect_vec(),
        Direction::Down => ((i + 1)..=(i + number))
            .map(|i| Place { i, j })
            .collect_vec(),
        Direction::Right => ((j + 1)..=(j + number))
            .map(|j| Place { i, j })
            .collect_vec(),
        Direction::Left => ((j - number)..=(j - 1))
            .rev()
            .map(|j| Place { i, j })
            .collect_vec(),
    }
}

fn process_input(lines: Vec<String>) -> i64 {
    let mut places = vec![Place { i: 0, j: 0 }];

    for line in lines.iter() {
        let Place { i, j, .. } = places.last().expect("Err: get place");
        let step = line_to_step(line);

        let mut new_places = apply_step(*i, *j, &step);
        places.append(&mut new_places);
    }

    let mut i_min = 0;
    let mut j_min = 0;
    let mut i_max = 0;
    let mut j_max = 0;

    places.iter().for_each(|place| {
        i_min = place.i.min(i_min);
        j_min = place.j.min(j_min);
        i_max = place.i.max(i_max);
        j_max = place.j.max(j_max);
    });

    places.iter_mut().for_each(|place| {
        place.i += i_min.abs();
        place.j += j_min.abs();
    });

    let area: i64 = places
        .iter()
        .tuple_windows()
        .map(|(place0, place1)| (place0.i + place1.i) * (place0.j - place1.j))
        .sum::<i64>()
        .abs()
        / 2;

    let length = places.len() as i64;

    area + (length / 2) + 1
}

pub fn run() {
    let input = "./days/day18/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day18 done with result {result}.");
}
