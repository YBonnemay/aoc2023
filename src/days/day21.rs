use crate::{days::day9::extrapolate_numbers_right, utils::input_process::input_to_lines};
use itertools::Itertools;
use std::fmt;

use super::day9::get_vectors;

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq)]
struct Position {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
struct Map {
    terrains: Vec<Vec<Terrain>>,
    height: usize,
    width: usize,
}

#[derive(Clone, Ord, Eq, PartialOrd, PartialEq, Debug, Copy)]
enum TerrainId {
    Visited,
    Empty,
    Fixed,
}

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq)]
struct Terrain {
    id: TerrainId,
    distance: u64,
    i: usize,
    j: usize,
}

impl Terrain {
    fn default() -> Self {
        Self {
            id: TerrainId::Empty,
            i: 0,
            j: 0,
            distance: u64::MAX,
        }
    }
    fn new(ch: char, i: usize, j: usize) -> Self {
        let id = match ch {
            '.' => TerrainId::Empty,
            '#' => TerrainId::Fixed,
            'O' => TerrainId::Visited,
            'S' => TerrainId::Empty,
            _ => {
                panic!("Err: unmanaged character : {ch}")
            }
        };

        Self {
            id,
            i,
            j,
            distance: u64::MAX,
        }
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self.id {
            TerrainId::Empty => ".".to_string(),
            TerrainId::Fixed => "#".to_string(),
            TerrainId::Visited => "O".to_string(),
        };

        write!(f, "{ch}")
    }
}

fn get_candidates(map: &mut Map, starting_terrain: &Terrain) -> Vec<Terrain> {
    let Terrain { i, j, .. } = *starting_terrain;
    map.terrains[i].get_mut(j).expect("Err: no terrain").id = TerrainId::Empty;

    let positions = [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)];

    let terrains = positions
        .iter()
        .filter_map(|position| {
            let new_i = (i as i32 + position.0) as usize;
            let new_j = (j as i32 + position.1) as usize;
            if (0..(map.height)).contains(&new_i) && (0..(map.width)).contains(&new_j) {
                return Some((new_i, new_j));
            }
            None
        })
        .filter_map(|position| {
            let terrain = map.terrains[position.0]
                .get_mut(position.1)
                .expect("Err:  no terrain");
            if terrain.id != TerrainId::Empty {
                return None;
            }
            terrain.id = TerrainId::Visited;
            Some(*terrain)
        })
        .collect_vec();

    terrains
}

#[allow(dead_code)]
fn display_map(map: &Map) {
    for line in map.terrains.iter() {
        let displayed_line = line.iter().join("");
        println!("{displayed_line}");
    }
    println!();
}

#[allow(dead_code)]
fn process_lines_part_1(lines: &Vec<String>, repeats: usize) -> usize {
    // 64 for part 1
    let iterations = 65 + 131 * repeats;

    let times = 1 + repeats * 2;

    let height = lines.len() * times;
    let width = lines.first().expect("Err: no first line").len() * times;
    let mut terrains = vec![vec![Terrain::default(); width]; height];

    let mut repeated_lines: Vec<String> = vec![];
    for _ in 0..times {
        repeated_lines.append(&mut lines.clone());
    }

    for (i, line) in repeated_lines.iter().enumerate().collect_vec().iter() {
        for (j, ch) in line.repeat(times).chars().enumerate().collect_vec() {
            let current_terrain = Terrain::new(ch, *i, j);
            terrains[*i][j] = current_terrain;
        }
    }

    let terrains_vec = terrains.iter().flatten().collect_vec();
    let starting_terrain = **terrains_vec
        .get(terrains_vec.len() / 2)
        .expect("Err: no terrain");

    let mut map = Map {
        height,
        width,
        terrains,
    };

    let mut terrains = vec![starting_terrain];

    for _ in 0..iterations {
        let next_positions = vec![];
        let mut reduced: Vec<Terrain> = terrains.iter().fold(next_positions, |mut acc, terrain| {
            let mut next = get_candidates(&mut map, terrain);
            acc.append(&mut next);
            acc
        });
        reduced.sort();
        reduced.dedup();
        terrains = reduced;
    }

    // display_map(&map);

    terrains.len()
}

fn part_2(lines: &Vec<String>) -> i64 {
    let exponents = [0, 1, 2, 3];
    let mut datum_ints: Vec<i64> = exponents
        .iter()
        .map(|exponent| process_lines_part_1(lines, *exponent as usize) as i64)
        .collect_vec();

    for _ in 4..=202300 {
        let vectors = get_vectors(&datum_ints);
        let extra = extrapolate_numbers_right(&vectors);
        datum_ints.push(extra);
        datum_ints.remove(0);
    }
    let result = datum_ints.last().expect("Err: no last");
    println!(" extra {:?}.", datum_ints.last().expect("Err: no last"));
    *result
}

pub fn run() {
    let input = "./days/day21/input.txt";
    let lines = input_to_lines(input);
    let result = part_2(&lines);
    println!("\n day21 done, with result {result}.");
}
