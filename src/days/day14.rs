use itertools::Itertools;
// use std::clone;

use crate::utils::input_process::input_to_lines;

type Zone<T> = dyn Iterator<Item = T>;

#[derive(Clone, Ord, Eq, PartialOrd, PartialEq, Debug, Copy)]
enum TerrainId {
    Movable,
    Empty,
    Fixed,
}

enum Compass {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Terrain {
    id: TerrainId,
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

/// .
///
/// # Panics
///
/// Panics if .
fn get_iterator<'a, T>(
    direction: Compass,
    zone: &'a Vec<Vec<T>>,
    index: &'a usize,
) -> impl Iterator<Item = &'a T> {
    let width = zone.first().expect("Err: no firs row");
    let height = zone.len();
    match direction {
        Compass::North => zone
            .iter()
            .map(|line| line.get(*index).expect("Err: no index")),
        Compass::South => todo!(),
        Compass::East => todo!(),
        Compass::West => todo!(),
    }
}

fn get_zone(lines: &mut [String]) -> Vec<Vec<Terrain>> {
    lines
        .iter()
        .map(|line| line.chars().map(Terrain::new).collect::<Vec<Terrain>>())
        .collect::<Vec<Vec<Terrain>>>()
}

fn sort_line<'a>(line: impl Iterator<Item = &'a Terrain>) -> Vec<&'a Terrain> {
    let chunks_vec = line
        .collect_vec()
        .split(|&terrain| terrain.id == TerrainId::Fixed)
        .map(|chunks| {
            let mut sortable_chunks = Vec::from(chunks);
            sortable_chunks.sort_unstable_by_key(|sortable_chunks| sortable_chunks.id);
            sortable_chunks.to_vec()
        })
        .intersperse(vec![&Terrain {
            id: TerrainId::Fixed,
        }])
        .flatten()
        .collect::<Vec<&Terrain>>();

    chunks_vec
    // intersperse(
    //     chunks_vec,
    //     vec![&Terrain {
    //         id: TerrainId::Fixed,
    //     }],
    // )
    // .flatten()
    // .collect::<Vec<&Terrain>>();
}

fn process_lines(lines: &mut [String]) -> usize {
    let zone = get_zone(lines);
    let width = zone.first().expect("Err: no first").len();
    // zone
    let mut result = 0;
    for j in 0..width {
        let line = get_iterator(Compass::North, &zone, &j);
        // println!("{:?}", line);
        let sorted_line = sort_line(line);
        let rates = (1..(sorted_line.len() + 1)).rev();

        for (terrain, rate) in sorted_line.iter().zip(rates) {
            match terrain.id {
                TerrainId::Movable => {
                    result += rate;
                }
                _ => {}
            }
        }
    }

    result
}

pub fn run() {
    let input = "./days/day14/input.txt";
    let mut data = input_to_lines(input);
    let result = process_lines(&mut data);
    println!("\n day14 done with result {result}.");
}

// fn sort_line<'a>(line: impl Iterator<Item = &'a Terrain>) -> () {
//     let chunks_vec = line
//         .collect_vec()
//         .split(|&terrain| terrain.id == TerrainId::Fixed)
//         .map(|chunks| {
//             let mut sortable_chunks = Vec::from(chunks);
//             sortable_chunks.sort_unstable_by_key(|sortable_chunks| sortable_chunks.id);
//             sortable_chunks.to_vec()
//         })
//         .collect::<Vec<Vec<&Terrain>>>();

//     intersperse(
//         chunks_vec,
//         vec![&Terrain {
//             id: TerrainId::Fixed,
//         }],
//     )
//     .flatten()
//     .collect::<Vec<&Terrain>>();
// }
