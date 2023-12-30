use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn line_to_ints<T: FromStr>(string: &String, ch: char) -> Vec<T> {
    string
        .split(ch)
        .filter_map(|number| number.parse::<T>().ok())
        .collect()
}

pub fn input_to_lines(input: &str) -> Vec<String> {
    let fpath = Path::new(input);
    let file = File::open(fpath).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.collect::<Result<Vec<String>, _>>().unwrap()
}
