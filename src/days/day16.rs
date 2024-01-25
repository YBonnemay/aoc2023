use std::collections::HashSet;

use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Antislash,
    Dash,
    Dot,
    Pipe,
    Slash,
}

#[derive(Debug, Clone)]
struct Place {
    i: usize,
    j: usize,
    shape: Shape,
    directions: Vec<Direction>,
}

impl Place {
    fn new(i: usize, j: usize, ch: char) -> Self {
        let shape = match ch {
            '/' => Shape::Slash,
            '\\' => Shape::Antislash,
            '.' => Shape::Dot,
            '-' => Shape::Dash,
            '|' => Shape::Pipe,
            _ => panic!("Err: unknown character"),
        };

        let beams: Vec<Direction> = Vec::new();
        Place {
            i,
            j,
            shape,
            directions: beams,
        }
    }
}

struct Map {
    places: Vec<Vec<Place>>,
    height: usize,
    width: usize,
}

// Ew. Should bit fiddle instead, perhaps, but then ew.
fn get_slash_directions(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::North => vec![Direction::East],
        Direction::West => vec![Direction::South],
        Direction::South => vec![Direction::West],
        Direction::East => vec![Direction::North],
    }
}

fn get_antislash_directions(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::North => vec![Direction::West],
        Direction::East => vec![Direction::South],
        Direction::South => vec![Direction::East],
        Direction::West => vec![Direction::North],
    }
}

fn get_dot_direction(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::North => vec![Direction::North],
        Direction::East => vec![Direction::East],
        Direction::South => vec![Direction::South],
        Direction::West => vec![Direction::West],
    }
}

fn get_pipe_direction(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::North => vec![Direction::North],
        Direction::East => vec![Direction::North, Direction::South],
        Direction::South => vec![Direction::South],
        Direction::West => vec![Direction::North, Direction::South],
    }
}

fn get_dash_direction(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::North => vec![Direction::East, Direction::West],
        Direction::East => vec![Direction::East],
        Direction::South => vec![Direction::East, Direction::West],
        Direction::West => vec![Direction::West],
    }
}

fn get_dir_shape(direction: Direction, shape: &Shape) -> Vec<Direction> {
    match shape {
        Shape::Antislash => get_antislash_directions(direction),
        Shape::Dash => get_dash_direction(direction),
        Shape::Dot => get_dot_direction(direction),
        Shape::Slash => get_slash_directions(direction),
        Shape::Pipe => get_pipe_direction(direction),
    }
}

impl Place {
    fn get_new_directions(&mut self) -> Vec<Direction> {
        let mut directions_next: Vec<Direction> = Vec::new();
        while let Some(directions) = self.directions.pop() {
            let new_directions = get_dir_shape(directions, &self.shape);
            directions_next.extend(new_directions);
        }
        directions_next
    }
}

fn get_place_direction(
    mut i: usize,
    mut j: usize,
    direction: &Direction,
    map: &Map,
) -> Option<Place> {
    match direction {
        Direction::North => {
            i = i.checked_sub(1)?;
        }
        Direction::South => {
            i += 1;
            if i >= map.height {
                return None;
            }
        }
        Direction::East => {
            j += 1;
            if j >= map.width {
                return None;
            }
        }
        Direction::West => {
            j = j.checked_sub(1)?;
        }
    }
    let new_place = get_place(i, j, map);

    Some(new_place.clone())
}

fn get_place(i: usize, j: usize, map: &Map) -> Place {
    map.places[i][j].clone()
}

fn process_input(inputs: Vec<String>) -> usize {
    let width = inputs.first().expect("Err: no input").len();
    let height = inputs.len();

    let places = inputs
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| Place::new(i, j, ch))
                .collect_vec()
        })
        .collect_vec();

    let map = Map {
        places,
        height,
        width,
    };
    let mut starting_place = get_place(0, 0, &map).clone();
    let starting_direction = Direction::East;

    let mut visited_places: HashSet<(usize, usize, Direction)> = HashSet::new();

    starting_place.directions.push(starting_direction.clone());
    visited_places.insert((starting_place.i, starting_place.j, starting_direction));

    let mut places = vec![starting_place];
    while let Some(mut place) = places.pop() {
        let next_directions = place.get_new_directions();
        for direction in next_directions {
            if let Some(mut next_place) =
                get_place_direction(place.i, place.j, &direction, &map).clone()
            {
                if visited_places.contains(&(next_place.i, next_place.j, direction.clone())) {
                    continue;
                }

                visited_places.insert((next_place.i, next_place.j, direction.clone()));
                next_place.directions.push(direction.clone());
                places.push(next_place);
            }
        }
    }

    let visited = visited_places
        .iter()
        .map(|(i, j, _)| (i, j))
        .unique()
        .collect_vec();

    visited.len()
}

pub fn run() {
    let input = "./days/day16/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day16 done with result {result}.");
}
