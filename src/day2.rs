use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Game {
    id: u32,
    possible: bool,
    power: i32,
}

const MAX_BLUE: i32 = 14;
const MAX_GREEN: i32 = 13;
const MAX_RED: i32 = 12;

fn process_line(line: &str) -> Game {
    // Do _not_ like regexps.
    let mut game: Vec<&str> = line.split([':', ';']).collect();
    let mut subsets: Vec<&str> = game.remove(0).split(' ').collect();

    let id = subsets
        .remove(1)
        .parse::<u32>()
        .expect("Err: Could not parse number");

    let mut blue_max = 0;
    let mut green_max = 0;
    let mut red_max = 0;

    for subset in game {
        for cube in subset.split([',']) {
            let mut number_color: Vec<&str> = cube.split(' ').collect();
            let color = number_color.pop().expect("No color");
            let number = number_color
                .pop()
                .expect("No number")
                .parse::<i32>()
                .expect("Err: Could not parse number");

            match color {
                "blue" => {
                    blue_max = cmp::max(blue_max, number);
                }
                "green" => {
                    green_max = cmp::max(green_max, number);
                }
                "red" => {
                    red_max = cmp::max(red_max, number);
                }
                _ => {}
            }
        }
    }

    Game {
        id,
        possible: blue_max <= MAX_BLUE && green_max <= MAX_GREEN && red_max <= MAX_RED,
        power: blue_max * green_max * red_max,
    }
}

fn day2(input: &str) -> (u32, i32) {
    let fpath = Path::new(input);
    let file = File::open(fpath).unwrap();
    let lines = io::BufReader::new(file).lines();

    lines
        .map(|line| process_line(&line.expect("Err: could not use line")))
        .fold((0, 0), |(possible, powers), game| {
            let mut new_possible = possible;
            if game.possible {
                new_possible += game.id;
            }

            (new_possible, powers + game.power)
        })
}

pub fn run() {
    let input = "./days/day2/input.txt";
    let (possible, powers) = day2(input);
    println!("day2 done with input {input} result (possible, powers): {possible}, {powers}");
}
