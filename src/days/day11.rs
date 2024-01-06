use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum SpaceType {
    Galaxy,
    Void,
}

#[derive(Debug, Clone)]
struct Space {
    i: usize,
    j: usize,
    value: usize,
    id: SpaceType,
}

impl Space {
    fn default() -> Space {
        Space {
            i: 0,
            j: 0,
            value: 1,
            id: SpaceType::Void,
        }
    }
}

fn transpose(spaces: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let width = spaces.first().expect("Err: no first").len();
    let height = spaces.len();

    let mut cols: Vec<Vec<Space>> = vec![vec![Space::default(); height]; width];
    for i in 0..height {
        for j in 0..width {
            cols[j][i] = spaces[i][j].clone();
        }
    }
    cols
}

fn expand(rows: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    rows.into_iter().fold(Vec::new(), |mut acc, row| {
        let galaxies: Vec<&Space> = row
            .iter()
            .filter(|space| space.id == SpaceType::Galaxy)
            .collect();
        if galaxies.is_empty() {
            let mut added_row = row.clone();
            added_row = added_row
                .iter_mut()
                .map(|space| {
                    space.value = 1000000;
                    space.clone()
                })
                .collect();
            acc.push(added_row);
        } else {
            acc.push(row);
        }
        acc
    })
}

fn input_to_spaces(input: &Vec<String>) -> Vec<Vec<Space>> {
    let width = input.first().expect("Err: no first").len();
    let height = input.len();
    let mut spaces: Vec<Vec<Space>> = vec![vec![Space::default(); width]; height];

    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                spaces[i][j] = Space {
                    i,
                    j,
                    value: 1,
                    id: SpaceType::Galaxy,
                };
            }
        }
    }
    spaces
}

fn get_galaxies(input: &Vec<String>) -> usize {
    let spaces = input_to_spaces(input);
    let expanded_rows = expand(spaces);
    let cols = transpose(expanded_rows);
    let expanded_cols = expand(cols);

    let mut galaxies: Vec<Space> = Vec::new();
    for (i, row) in expanded_cols.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
            if space.id == SpaceType::Galaxy {
                let total_i = row[0..j].iter().map(|space| space.value).sum::<usize>();
                let total_j = expanded_cols[0..i]
                    .iter()
                    .map(|row| row[j].value)
                    .sum::<usize>();

                galaxies.push(Space {
                    i: total_i,
                    j: total_j,
                    value: 1,
                    id: SpaceType::Galaxy,
                })
            }
        }
    }

    // Compute all distances
    galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .map(|(galaxies_left, galaxies_rigth)| {
            galaxies_left.i.abs_diff(galaxies_rigth.i) + galaxies_left.j.abs_diff(galaxies_rigth.j)
        })
        .sum::<usize>()
        / 2
}

fn process_lines(lines: &Vec<String>) -> usize {
    get_galaxies(lines)
}

pub fn run() {
    let input = "./days/day11/input.txt";
    let data = input_to_lines(input);
    let result = process_lines(&data);
    println!("\n day11 done with result {result}.");
}
