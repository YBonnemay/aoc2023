use crate::utils::input_process::input_to_lines;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::graphmap::UnGraphMap;

const GOAL: (usize, usize) = (140, 139);

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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Place {
    i: usize,
    j: usize,
    id: PlaceId,
    direction: Direction,
}

impl Place {
    fn get_key(&self) -> usize {
        self.i * 1000 + self.j
    }
}

fn get_at_direction(
    current_place: &Place,
    direction: &Direction,
    map: &mut Map,
    cost: usize,
) -> Option<(Place, Direction, usize)> {
    let Place { mut i, mut j, .. } = current_place;

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

    let place = map.places[i][j];

    if let PlaceId::Forest = place.id {
        return None;
    }

    Some((place, *direction, cost + 1))
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
    direction_from: Direction,
    cost: usize,
) -> Vec<(Place, Direction, usize)> {
    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .filter(|direction| get_opposite(direction) != direction_from)
    .filter_map(|direction| get_at_direction(place_from, direction, map, cost))
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

fn walk_reduce(map: &mut Map) -> usize {
    let places = map.places.clone();
    let start = places[0][1];
    let end = places[GOAL.0][GOAL.1];

    let mut ungraph: UnGraphMap<usize, usize> = UnGraphMap::new();
    ungraph.add_node(start.get_key());
    ungraph.add_node(end.get_key());

    let mut successors: Vec<(Place, Place, Direction, usize)> =
        vec![(start, start, Direction::South, 0)];

    // Build faster graph
    while let Some((current_place, mut edge_start, direction_from, cost)) = successors.pop() {
        let current_place_key = current_place.get_key();

        let current_successors = get_successors(&current_place, map, direction_from, cost);
        let mut reset_costs = false;
        if current_successors.len() > 1 || current_place.get_key() == end.get_key() {
            if ungraph.contains_edge(edge_start.get_key(), current_place_key) {
                continue;
            };
            ungraph.add_edge(edge_start.get_key(), current_place_key, cost);
            edge_start = current_place;
            reset_costs = true;
        }

        for (current_successor, direction_from, cost) in current_successors {
            let cost = if reset_costs { 1 } else { cost };
            successors.push((current_successor, edge_start, direction_from, cost));
        }
    }

    // Use faster graph
    let ways = all_simple_paths::<Vec<_>, _>(&ungraph, start.get_key(), end.get_key(), 0, None)
        .collect::<Vec<_>>();

    ways.iter()
        .map(|path| {
            path.iter()
                .map_windows(|&[a, b]| ungraph.edge_weight(*a, *b).expect("Err: no edge"))
                .sum::<usize>()
        })
        .max()
        .expect("Err: no max")
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

    walk_reduce(&mut map)
}

// .. lib again. Because it is a wheel I should reinvent again.
pub fn run() {
    let input = "./days/day23/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day17 done with result {result}.");
}
