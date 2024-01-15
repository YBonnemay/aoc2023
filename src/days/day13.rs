use crate::utils::input_process::input_to_lines;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Standard,
    Transposed,
}

// No one will even see that horror.
fn transpose(pattern: &Vec<String>) -> Vec<String> {
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

fn equal(left: &str, right: &str) -> bool {
    !left.chars().zip(right.chars().rev()).any(|(l, r)| l != r)
}

fn scan_line(line: &String) -> Vec<usize> {
    let len = line.len();
    (1..len).filter_map(|i| check_idx(line, i, len)).collect()
}

fn check_idx(line: &String, i: usize, len: usize) -> Option<usize> {
    let span_leng = i.min(len - i);
    let left_span = &line[(i - span_leng)..i];
    let right_span = &line[i..(i + span_leng)];
    if equal(left_span, right_span) {
        return Some(i);
    }
    None
}

fn process_pattern(pattern: &Vec<String>, mode: Mode) -> Option<usize> {
    let mut pattern = pattern.clone();
    if mode == Mode::Transposed {
        pattern = transpose(&pattern);
    }

    let line = pattern.first().expect("Err: no line");
    let len = line.len();

    let mut indices = scan_line(line);

    for line in pattern.iter() {
        indices = indices
            .into_iter()
            .filter_map(|idx| check_idx(line, idx, len))
            .collect();
        println!("{:?}", indices);
    }

    let mut indice = *indices.first()?;

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

// This one took some de-uglification.
// And still.
pub fn run() {
    let input = "./days/day13/input.txt";
    let mut data = input_to_lines(input);
    let result = process_lines(&mut data);
    println!("\n day13 done with result {result}.");
}
