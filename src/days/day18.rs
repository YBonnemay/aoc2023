use std::ops::BitAnd;

use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

fn get_direction(st: &str) -> Direction {
    match st {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "R" => Direction::Right,
        "L" => Direction::Left,
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
struct Step {
    direction: Direction,
    number: i32,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
struct Place {
    i: i32,
    j: i32,
}

fn line_to_step(line: &String) -> Step {
    let step_line = line.split(' ').collect_vec();
    Step {
        direction: get_direction(step_line[0]),
        number: step_line[1].parse::<i32>().expect("Could not parse number"),
    }
}

fn apply_step(i: i32, j: i32, step: &Step) -> Vec<Place> {
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

fn display(height: i32, width: i32, places: &Vec<Place>) {
    let mut lines = vec![".".repeat(width as usize); height as usize];
    for place in places {
        let line = lines
            .get_mut(place.i as usize)
            .expect("Err: no line to update");

        line.replace_range(place.j as usize..place.j as usize + 1, "X");
    }

    for line in lines {
        println!("{line}");
    }
}

fn process_input(lines: Vec<String>) -> i32 {
    // println!("{:?}", lines);

    let step = line_to_step(&lines[0]);

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

    // display(i_max + i_min.abs() + 1, j_max + j_min.abs() + 1, &places);

    let area: i32 = places
        .iter()
        .tuple_windows()
        .map(|(place0, place1)| {
            // println!("{:?}", place1);
            (place0.i + place1.i) * (place0.j - place1.j)
        })
        .sum::<i32>()
        .abs()
        / 2;

    let length = places.len() as i32;

    // println!("\nlength {length} area {area}");

    area + (length / 2) + 1
}

pub fn run() {
    let input = "./days/day18/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day18 done with result {result}.");
}
