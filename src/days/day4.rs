use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Data {
    card_total: i32,
    winning: HashSet<i32>,
    mine: HashSet<i32>,
}

fn process_data_part_one(datas: &[Data]) -> i32 {
    datas
        .iter()
        .map(|data| {
            let intersection: Vec<_> = data.winning.intersection(&data.mine).collect();
            let intersection_len = intersection.len();
            if intersection_len > 0 {
                2_i32.pow((intersection.len() - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

// TODO: clean that copy mess
fn process_data_part_two(datas: &Vec<Data>) -> i32 {
    let mut local = datas.clone();

    for idx in 0..local.len() {
        let intersection: Vec<_> = local[idx].winning.intersection(&local[idx].mine).collect();
        let intersection_len = intersection.len();
        let current_total = local[idx].card_total;
        for update_value in local.iter_mut().skip(idx + 1).take(intersection_len) {
            update_value.card_total += current_total;
        }
    }

    local.iter().map(|data| data.card_total).sum()
}

fn day4(input: &str) -> (i32, i32) {
    let fpath = Path::new(input);
    let file = File::open(fpath).unwrap();
    let lines = io::BufReader::new(file).lines();

    let lines: Vec<Data> = lines
        .into_iter()
        .map(|f| f.expect("Err: No line"))
        .map(|line| {
            let data = (line.split(':').collect::<Vec<&str>>())[1];
            let data_strings: Vec<&str> = data.split(" | ").collect();
            let winning = data_strings[0]
                .split(' ')
                .filter_map(|number| number.parse::<i32>().ok())
                .collect();

            let mine = data_strings[1]
                .split(' ')
                .filter_map(|number| number.parse::<i32>().ok())
                .collect();

            Data {
                winning,
                mine,
                card_total: 1,
            }
        })
        .collect();

    (process_data_part_one(&lines), process_data_part_two(&lines))
}

pub fn run() {
    let input = "./days/day4/input.txt";
    let (part_one, part_two) = day4(input);
    println!(
        "\n day4 part one done with input {input} result part_one {part_one},  part_two {part_two}"
    );
}
