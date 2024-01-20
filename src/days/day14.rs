use crate::utils::input_process::input_to_lines;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Ord, Eq, PartialOrd, PartialEq, Debug, Copy)]
enum TerrainId {
    Movable,
    Empty,
    Fixed,
}

#[derive(Clone, Ord, Eq, PartialOrd, PartialEq)]
enum Compass {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq)]
struct Terrain {
    id: TerrainId,
}

#[derive(Debug)]
struct ResultData {
    index: usize,
    value: usize,
}

impl Terrain {
    fn new(ch: char) -> Self {
        match ch {
            'O' => Self {
                id: TerrainId::Movable,
            },
            '.' => Self {
                id: TerrainId::Empty,
            },
            '#' => Self {
                id: TerrainId::Fixed,
            },
            _ => {
                panic!("Err: unmanaged character : {ch}")
            }
        }
    }
}

fn set_line<T: Copy>(direction: &Compass, zone: &mut [Vec<T>], index: &usize, update_line: &[T]) {
    match direction {
        Compass::North | Compass::South => zone.iter_mut().enumerate().for_each(|(i, line)| {
            line[*index] = *(update_line.get(i).expect("Err: no input index"));
        }),
        Compass::East | Compass::West => {
            zone[*index] = update_line.to_vec();
        }
    }
}

fn get_zone(lines: &mut [String]) -> Vec<Vec<Terrain>> {
    lines
        .iter()
        .map(|line| line.chars().map(Terrain::new).collect::<Vec<Terrain>>())
        .collect::<Vec<Vec<Terrain>>>()
}

fn sort_line(direction: &Compass, line: Vec<Terrain>) -> Vec<Terrain> {
    let chunks_vec = line
        .split(|&terrain| terrain.id == TerrainId::Fixed)
        .map(|chunks| {
            let mut sortable_chunks = Vec::from(chunks);
            sortable_chunks.sort_unstable_by(|a, b| {
                if *direction == Compass::North || *direction == Compass::West {
                    a.partial_cmp(b).unwrap()
                } else {
                    b.partial_cmp(a).unwrap()
                }
            });
            sortable_chunks.to_vec()
        })
        .intersperse(vec![Terrain {
            id: TerrainId::Fixed,
        }])
        .flatten()
        .collect::<Vec<Terrain>>();

    chunks_vec
}

fn compute_zone_result(zone: &[Vec<Terrain>]) -> usize {
    let rates = (1..(zone.len() + 1)).rev();

    zone.iter()
        .zip(rates)
        .map(|(line, rate)| {
            line.iter()
                .filter_map(|terrain| match terrain.id {
                    TerrainId::Movable => Some(rate),
                    _ => None,
                })
                .sum::<usize>()
        })
        .sum()
}

fn process_direction(direction: &Compass, zone_updated: &mut [Vec<Terrain>]) {
    let width = zone_updated.first().expect("Err: no first").len();
    let height = zone_updated.len();
    let range = match direction {
        Compass::North => 0..width,
        Compass::South => 0..width,
        Compass::East => 0..height,
        Compass::West => 0..height,
    };

    range.for_each(|index| {
        let line = if *direction == Compass::North || *direction == Compass::South {
            zone_updated
                .iter()
                .map(|line| *line.get(index).expect("Err: no index"))
                .collect_vec()
        } else {
            (*zone_updated.get(index).expect("Err: no index")).clone()
        };

        let sorted_line = sort_line(direction, line);
        set_line(direction, zone_updated, &index, &sorted_line);
    });
}

fn process_lines(lines: &mut [String]) -> usize {
    let mut zone = get_zone(lines);
    let mut results: HashMap<String, ResultData> = HashMap::new();
    let end_value = 1000000000;
    let mut index = 0;
    for i in 0..end_value {
        index = i;
        for direction in [Compass::North, Compass::West, Compass::South, Compass::East] {
            process_direction(&direction, &mut zone);
        }
        let value = compute_zone_result(&zone);
        match results.try_insert(get_string(&zone), ResultData { index, value }) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }

    let cycle_start_index = results.get(&get_string(&zone)).expect("Err: no data").index;
    let cycle_len = index - cycle_start_index;
    let index_in_cycle = (end_value - cycle_start_index - 1) % cycle_len;

    let index_result = results
        .values()
        .filter(|res| res.index == (index_in_cycle + cycle_start_index))
        .collect_vec();

    index_result[0].value
}

fn get_string(zone: &Vec<Vec<Terrain>>) -> String {
    let mut line = "".to_string();
    for terrains in zone {
        line.push_str(
            &terrains
                .iter()
                .map(|terrain| match terrain.id {
                    TerrainId::Empty => '.',
                    TerrainId::Fixed => '#',
                    TerrainId::Movable => '0',
                })
                .collect::<String>(),
        );
        line.push('\n');
    }
    line
}

// In this episode, gratuitous templated fuckeries
// Heroically clawed from the Compiler!
// How sad that
// They didn not help
pub fn run() {
    let input = "./days/day14/input.txt";
    let mut data = input_to_lines(input);
    let result = process_lines(&mut data);
    println!("\n day14 done with result {result}.");
}
