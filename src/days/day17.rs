use crate::utils::input_process::input_to_lines;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

const START_CONSTRAINT: usize = 4;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Place {
    i: usize,
    j: usize,
    cost: usize,
    same_direction: usize,
    direction: Option<Direction>,
}

impl Place {
    fn new(i: usize, j: usize, cost: usize) -> Self {
        Place {
            i,
            j,
            cost,
            same_direction: 0,
            direction: None,
        }
    }
}

fn get_opposite(lhs: &Direction) -> Direction {
    match lhs {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
        Direction::South => Direction::North,
    }
}

fn get_at_direction(place_from: &Place, direction: &Direction, map: &mut Map) -> Option<Place> {
    let Place { mut i, mut j, .. } = place_from;

    if let Some(direction_from) = place_from.direction {
        if direction_from == get_opposite(direction) {
            return None;
        }
    }

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

    let place = &mut map.places[i][j];

    if (((map.width) - START_CONSTRAINT)..(map.width - 1)).contains(&j)
        && (((map.height) - START_CONSTRAINT)..(map.height - 1)).contains(&i)
    {
        return None;
    }

    let same_direction: usize;

    if place_from.direction == Some(*direction) {
        same_direction = place_from.same_direction + 1;
        if same_direction > 10 {
            return None;
        }
    } else {
        if place_from.same_direction < START_CONSTRAINT && place_from.direction.is_some() {
            return None;
        }
        same_direction = 1;
    }

    place.same_direction = same_direction;
    place.direction = Some(*direction);
    Some(place.clone())
}

fn get_candidates(place_from: &Place, map: &mut Map) -> Vec<(Place, usize)> {
    let candidate_directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let candidates = candidate_directions
        .iter()
        .filter_map(|direction| get_at_direction(place_from, direction, map))
        .map(|place| (place.clone(), place.cost))
        .collect_vec();
    candidates
}

struct Map {
    places: Vec<Vec<Place>>,
    height: usize,
    width: usize,
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
                .map(|(j, ch)| {
                    Place::new(
                        i,
                        j,
                        ch.to_string().parse::<usize>().expect("Err: not an usize"),
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut map = Map {
        places,
        height,
        width,
    };

    let mut starting_place = map.places[0][0].clone();
    starting_place.direction = None;
    let Some((_, cost)) = dijkstra(
        &starting_place,
        |p| get_candidates(p, &mut map),
        |b| b.i == (height - 1) && b.j == (width - 1),
    ) else {
        panic!("no path found")
    };

    cost
}

// New rule : I can use libs, after all, when I am asked something as fun as "implement a Djikstra".
pub fn run() {
    let input = "./days/day17/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day17 done with result {result}.");
}
