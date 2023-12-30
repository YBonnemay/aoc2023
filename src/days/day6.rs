use crate::utils::input_process::{input_to_lines, line_to_ints};

#[derive(Debug, Clone)]
struct Race {
    time: i64,
    distance: i64,
}

fn format_data(data: &[String]) -> Vec<Race> {
    let times: Vec<_> = line_to_ints::<i64>(&data[0], ' ');
    let distances: Vec<_> = line_to_ints::<i64>(&data[1], ' ');

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn format_data_part_two(data: &[String]) -> Race {
    let time = data[0][11..]
        .replace(' ', "")
        .parse::<i64>()
        .expect("Err: time parse");

    let distance = data[1][11..]
        .replace(' ', "")
        .parse::<i64>()
        .expect("Err: distance parse");

    Race { time, distance }
}

fn process_race(race: &Race) -> i64 {
    let best_time = race.time / 2;
    let odd = (race.time + 1) % 2;

    let mut records = 0;
    for idx in (0..best_time + 1).rev() {
        if idx * (race.time - idx) <= race.distance {
            break;
        }
        records += 1;
    }

    (records * 2) - odd
}

fn process_races(races: Vec<Race>) -> i64 {
    races.iter().map(process_race).product()
}

fn day6(input: &str) -> (i64, i64) {
    let data = input_to_lines(input);
    let races: Vec<Race> = format_data(&data);
    let result_part_one = process_races(races);

    let race = format_data_part_two(&data);
    let result_part_two = process_race(&race);
    (result_part_one, result_part_two)
}

pub fn run() {
    let input = "./days/day6/input.txt";
    let (result_part_one, result_part_two) = day6(input);
    println!("\n day6 part one done with result_part_one {result_part_one} result result_part_two {result_part_two}");
}
