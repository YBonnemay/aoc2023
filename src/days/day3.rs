use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

struct Input {
    lines: Vec<String>,
    width: i32,
    max_height: usize,
    max_width: usize,
    stars_groups: HashMap<i32, Vec<i32>>,
}

fn is_symbol(char_at: CharAt) -> bool {
    !char_at.ch.is_ascii_digit() && char_at.ch != '.'
}

fn group_to_number(group: &[CharAt]) -> i32 {
    group
        .iter()
        .fold(String::new(), |mut acc, char_at| {
            acc.push(char_at.ch);
            acc
        })
        .parse()
        .expect("Err: could not parse number")
}

fn make_groups(i_pos: usize, line: &str) -> Vec<Vec<CharAt>> {
    let mut data_grouped: Vec<Vec<CharAt>> = Vec::new();
    for (key, group) in &line
        .chars()
        .enumerate()
        .group_by(|(_j, ch)| (*ch).is_ascii_digit())
    {
        if key {
            data_grouped.push(
                group
                    .collect::<Vec<(usize, char)>>()
                    .iter()
                    .map(|t| {
                        let (j_pos, ch) = *t;
                        CharAt { j_pos, ch, i_pos }
                    })
                    .collect(),
            );
        }
    }
    data_grouped
}

impl Input {
    fn new(lines: &mut Lines<BufReader<File>>) -> Input {
        let lines = lines.collect::<Result<Vec<_>, _>>().unwrap();
        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let stars_groups = HashMap::new();
        Input {
            lines,
            width,
            max_width: (width - 1) as usize,
            max_height: (height - 1) as usize,
            stars_groups,
        }
    }

    fn get_line(
        &self,
        i_pos: usize,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = CharAt> + '_ {
        let line = &self.lines[i_pos];

        line[start..end + 1]
            .chars()
            .enumerate()
            .map(move |(j, ch)| CharAt {
                j_pos: j + start,
                i_pos,
                ch,
            })
    }

    fn has_symbol_near(&mut self, i: &usize, char_ats: &[CharAt]) -> i32 {
        let left = match char_ats[0].j_pos.checked_sub(1) {
            Some(start) => start,
            None => char_ats[0].j_pos,
        };

        let right = std::cmp::min(char_ats.last().unwrap().j_pos + 1, self.max_width);

        let above = if *i > 0 {
            Some(self.get_line(*i - 1, left, right))
        } else {
            None
        };

        let below = if *i < self.max_height {
            Some(self.get_line(*i + 1, left, right))
        } else {
            None
        };

        let left_iterator = Some(self.get_line(*i, left, left + 1));
        let right_iterator = Some(self.get_line(*i, right - 1, right));

        let iter_char_ats = above
            .into_iter()
            .flatten()
            .chain(below.into_iter().flatten())
            .chain(left_iterator.into_iter().flatten())
            .chain(right_iterator.into_iter().flatten());

        let mut total_part = 0;
        // Get rid of collect
        for symbol in iter_char_ats.collect::<Vec<CharAt>>() {
            if is_symbol(symbol) && total_part == 0 {
                total_part = group_to_number(char_ats);
            }

            if symbol.ch == '*' {
                let index = symbol.i_pos as i32 * self.width + symbol.j_pos as i32;
                let value = group_to_number(char_ats);
                self.stars_groups
                    .entry(index)
                    .or_insert(Vec::from([]))
                    .push(value);
            }
        }

        // Output here and in stars_groups
        total_part
    }

    fn scan_lines_part_one(&mut self) -> i32 {
        let mut lines_total = 0;
        let line_groups: Vec<_> = self
            .lines
            .iter()
            .enumerate()
            .map(|(i, line)| (i, make_groups(i, line)))
            .collect();

        for (i, line_group) in line_groups.iter() {
            for group in line_group {
                lines_total += self.has_symbol_near(i, group);
            }
        }
        lines_total
    }
}

#[derive(Debug, Clone, Copy)]
struct CharAt {
    j_pos: usize,
    i_pos: usize,
    ch: char,
}

fn day3(input: &str) -> (i32, i32) {
    let fpath = Path::new(input);
    let file = File::open(fpath).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut input = Input::new(&mut lines);
    let part_one = input.scan_lines_part_one();
    let mut part_two = 0;
    for vals in input.stars_groups.values() {
        if vals.len() == 2 {
            part_two += vals[0] * vals[1];
        }
    }
    (part_one, part_two)
}

pub fn run() {
    let input = "./days/day3/input.txt";
    let (part_one, part_two) = day3(input);
    println!(
        "\n day3 part one done with input {input} result part_one {part_one} and part_two {part_two}"
    );
}
