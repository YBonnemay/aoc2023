use crate::utils::input_process::input_to_lines;
use core::fmt;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Hash, Copy)]
enum PlaceId {
    Path,
    Forest,
    Slope,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Place {
    i: usize,
    j: usize,
    id: PlaceId,
    direction: Direction,
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self.id {
            PlaceId::Path => ".".to_string(),
            PlaceId::Forest => "#".to_string(),
            PlaceId::Slope => match self.direction {
                Direction::North => '?'.to_string(),
                Direction::South => 'v'.to_string(),
                Direction::East => '>'.to_string(),
                Direction::West => '<'.to_string(),
            },
        };

        write!(f, "{ch}")
    }
}

fn get_at_direction(
    current_place: &Place,
    direction: &Direction,
    map: &mut Map,
) -> Option<(Place, Direction)> {
    let Place { mut i, mut j, .. } = current_place;

    match direction {
        Direction::North => {
            i = i.checked_sub(1)?;
        }
        Direction::South => {
            i += 1;
            if i >= map.height {
                // println!("None :height check");
                return None;
            }
        }
        Direction::East => {
            j += 1;
            if j >= map.width {
                // println!("None : width check");
                return None;
            }
        }
        Direction::West => {
            j = j.checked_sub(1)?;
        }
    }

    let place = map.places[i][j].clone();

    match place.id {
        PlaceId::Slope => {
            if &place.direction != direction {
                return None;
            }
        }
        PlaceId::Forest => {
            return None;
        }
        _ => {}
    }

    Some((place.clone(), *direction))
}

fn get_opposite(lhs: &Direction) -> Direction {
    match lhs {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
        Direction::South => Direction::North,
    }
}

fn get_successors(
    place_from: &Place,
    map: &mut Map,
    step_count: usize,
    direction_from: Direction,
) -> Vec<(Place, usize, Direction)> {
    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .filter(|direction| get_opposite(direction) != direction_from)
    .filter_map(|direction| get_at_direction(place_from, direction, map))
    .map(|(place, coming_from)| (place, step_count + 1, coming_from))
    .collect_vec()
}

impl Place {
    fn new(i: usize, j: usize, ch: char) -> Self {
        let id = match ch {
            '#' => PlaceId::Forest,
            '.' => PlaceId::Path,
            '<' | '>' | 'v' => PlaceId::Slope,
            _ => panic!("Err: wrong char {ch}"),
        };

        let direction = match ch {
            '<' => Direction::West,
            '>' => Direction::East,
            'v' => Direction::South,
            _ => Direction::North,
        };

        Place {
            i,
            j,
            id,
            direction,
        }
    }
}

struct Map {
    places: Vec<Vec<Place>>,
    height: usize,
    width: usize,
}

fn walk(map: &mut Map) -> usize {
    let places = map.places.clone();
    let mut successors: Vec<(Place, usize, Direction)> =
        vec![(places[0][1].clone(), 0, Direction::South)];

    let mut walks: Vec<usize> = vec![];

    while let Some((successor, cost, direction_from)) = successors.pop() {
        let current_successors = get_successors(&successor, map, cost, direction_from);

        if current_successors.is_empty() {
            walks.push(cost);
        }

        for (current_successor, cost, direction_from) in current_successors {
            successors.push((current_successor, cost, direction_from));
        }
    }

    *walks.iter().max().expect("Err: no max")
}

fn process_input(inputs: Vec<String>) -> usize {
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

    let width = inputs.first().expect("Err: no input").len();
    let height = inputs.len();

    let mut map = Map {
        places,
        height,
        width,
    };

    walk(&mut map)
}

pub fn run() {
    let input = "./days/day23/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day17 done with result {result}.");
}
