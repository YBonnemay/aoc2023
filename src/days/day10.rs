use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_directions() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
}

#[derive(Debug, Clone)]
struct Place {
    i: i32,
    j: i32,
    connections: Vec<Direction>,
    value: Option<i32>,
}

struct Map {
    places: Vec<Vec<Place>>,
    height: i32,
    width: i32,
}

impl Map {
    fn new(places: Vec<Vec<Place>>) -> Map {
        let height = places.len() as i32;
        let width = places[0].len() as i32;
        Map {
            places,
            height,
            width,
        }
    }

    fn get_place_at_direction(&self, place: &Place, direction: &Direction) -> Option<Place> {
        let Place { mut i, mut j, .. } = place;
        match direction {
            Direction::North => i -= 1,
            Direction::South => i += 1,
            Direction::East => j += 1,
            Direction::West => j -= 1,
        }

        // Check within map
        if i < 0_i32 || i > (self.height - 1) || j < 0_i32 || j > (self.width - 1) {
            return None;
        }

        let new_place = (self.places[i as usize][j as usize]).clone();

        // Check not visited
        if new_place.value.is_some() {
            return None;
        }

        // Check connected
        if !match direction {
            Direction::North => {
                place.connections.contains(&Direction::North)
                    && new_place.connections.contains(&Direction::South)
            }
            Direction::South => {
                place.connections.contains(&Direction::South)
                    && new_place.connections.contains(&Direction::North)
            }
            Direction::East => {
                place.connections.contains(&Direction::East)
                    && new_place.connections.contains(&Direction::West)
            }
            Direction::West => {
                place.connections.contains(&Direction::West)
                    && new_place.connections.contains(&Direction::East)
            }
        } {
            return None;
        }

        Some(new_place)
    }

    fn set_place(&mut self, place: Place) {
        let Place { i, j, .. } = place;
        self.places[i as usize][j as usize] = place;
    }

    fn get_move_candidates(&self, place: &Place) -> Vec<Place> {
        Direction::get_directions()
            .iter()
            .filter_map(|direction| self.get_place_at_direction(place, direction))
            .collect()
    }
}

fn char_to_place(ch: char, i: i32, j: i32) -> Place {
    match ch {
        '|' => Place {
            connections: vec![Direction::North, Direction::South],
            value: None,
            i,
            j,
        },
        '-' => Place {
            connections: vec![Direction::East, Direction::West],
            value: None,
            i,
            j,
        },
        'L' => Place {
            connections: vec![Direction::North, Direction::East],
            value: None,
            i,
            j,
        },
        'J' => Place {
            connections: vec![Direction::North, Direction::West],
            value: None,
            i,
            j,
        },
        '7' => Place {
            connections: vec![Direction::West, Direction::South],
            value: None,
            i,
            j,
        },
        'F' => Place {
            connections: vec![Direction::East, Direction::South],
            value: None,
            i,
            j,
        },
        '.' => Place {
            connections: vec![],
            value: None,
            i,
            j,
        },
        'S' => Place {
            connections: vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
            value: Some(0),
            i,
            j,
        },
        _ => {
            panic!("Err: unmanaged char {ch}")
        }
    }
}

fn get_places(lines: &[String]) -> Vec<Vec<Place>> {
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| char_to_place(ch, i as i32, j as i32))
                .collect::<Vec<Place>>()
        })
        .collect()
}

fn process_lines(lines: &[String]) -> (i32, i32) {
    let map_data = get_places(lines);
    let mut map = Map::new(map_data);

    // Starting place
    let mut place = map
        .places
        .iter()
        .flat_map(|line| line.iter())
        .find(|place| place.value == Some(0))
        .expect("Err: no starting point")
        .clone();

    let mut path_p2: Vec<Place> = Vec::new();

    while let Some(next_place) = map.get_move_candidates(&place).first_mut() {
        let next_value = place.value.expect("Err: no next value") + 1;
        next_place.value = Some(next_value);
        // The bad place, shoulddo
        map.set_place(next_place.clone());
        place = (*next_place).clone();
        path_p2.push(place.clone());
    }

    let area: i32 = path_p2
        .iter()
        .tuple_windows()
        .map(|(place0, place1)| (place0.i + place1.i) * (place0.j - place1.j))
        .sum::<i32>()
        .abs()
        / 2;

    let frontier = (path_p2
        .last()
        .expect("Err: no last")
        .value
        .expect("Err: no value")
        - 1)
        / 2;

    (frontier + 1, area - frontier)
}

pub fn run() {
    let input = "./days/day10/input.txt";
    let data = input_to_lines(input);
    let (frontier, area) = process_lines(&data);
    println!("\n day10 done with the frontier size {frontier} and enclosed area {area}.");
}
