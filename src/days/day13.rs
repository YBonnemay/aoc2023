use crate::utils::input_process::input_to_lines;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Standard,
    Transposed,
}

// No one will even see that horror.
fn transpose(pattern: &[String]) -> Vec<String> {
    let width = pattern.first().expect("Err: no line").len();
    let height = pattern.len();
    let mut transposed: Vec<String> = vec![String::new(); width];
    for w in 0..width {
        for h in 0..height {
            transposed[w].push(pattern[h].chars().nth(w).expect("Err: no char"));
        }
    }

    transposed
}

fn differences(left: &str, right: &str) -> usize {
    left.chars()
        .zip(right.chars().rev())
        .map(|(l, r)| if l != r { 1_usize } else { 0_usize })
        .sum()
}

fn check_idx(line: &str, i: usize, len: usize) -> usize {
    let span_leng = i.min(len - i);
    let left_span = &line[(i - span_leng)..i];
    let right_span = &line[i..(i + span_leng)];
    differences(left_span, right_span)
}

fn process_pattern(pattern: &[String], mode: Mode) -> Option<usize> {
    let mut pattern = Vec::from(pattern);
    if mode == Mode::Transposed {
        pattern = transpose(&pattern);
    }

    let line = pattern.first().expect("Err: no line");
    let len = line.len();

    // let mut indices = scan_line(line);
    let mut hits: HashMap<usize, usize> = HashMap::new();

    for line in pattern.iter() {
        for idx in 0..line.len() {
            *hits.entry(idx).or_insert(0) += check_idx(line, idx, len);
        }
    }

    hits.retain(|_key, value| value == &1);
    let mut indice = *hits.keys().next()?;

    if mode == Mode::Transposed {
        indice *= 100;
    }

    Some(indice)
}

fn process_lines(lines: &mut [String]) -> usize {
    let patterns: Vec<Vec<String>> = lines.iter_mut().fold(vec![Vec::new()], |mut acc, line| {
        if line.is_empty() {
            acc.push(Vec::new());
        } else {
            acc.last_mut()
                .expect("Err: no last pattern line")
                .push(line.clone());
        }
        acc
    });

    patterns
        .into_iter()
        .map(|pattern| {
            process_pattern(&pattern, Mode::Standard).unwrap_or_else(|| {
                process_pattern(&pattern, Mode::Transposed)
                    .expect("Err: transpose fallback not found")
            })
        })
        .sum()
}

pub fn run() {
    let input = "./days/day13/input.txt";
    let mut data = input_to_lines(input);
    let result = process_lines(&mut data);
    println!("\n day13 done with result {result}.");
}
