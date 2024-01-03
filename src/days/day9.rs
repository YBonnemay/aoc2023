use crate::utils::input_process::{input_to_lines, line_to_ints};

fn extrapolate_numbers_right(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|item| item.last().expect("Err: no last element"))
        .sum()
}

fn extrapolate_numbers_left(input: &[Vec<i64>]) -> i64 {
    input.iter().rev().fold(0, |mut acc, item| {
        acc = item.first().expect("Err: no first element") - acc;
        acc
    })
}

fn get_diff(input: &Vec<i64>) -> Vec<i64> {
    let len = input.len();
    let ends = &input[1..len];
    let starts = &input[0..len - 1];
    starts
        .iter()
        .zip(ends.iter())
        .map(|(start, end)| end - start)
        .collect()
}

fn get_vectors(input: &[i64]) -> Vec<Vec<i64>> {
    let mut vectors = Vec::new();
    let mut all_zeros = false;
    let mut current_vector = Vec::from(input);
    vectors.push(current_vector.clone());

    while !all_zeros {
        current_vector = get_diff(&current_vector);
        all_zeros = current_vector.iter().all(|item| item == &0);
        vectors.push(current_vector.clone());
    }

    vectors
}

fn day9(input: &str) -> (i64, i64) {
    let data = input_to_lines(input);

    data.iter()
        .fold((0, 0), |(mut acc_left, mut acc_right), datum| {
            let datum_ints = line_to_ints::<i64>(datum, ' ');
            let vectors = get_vectors(&datum_ints);
            acc_left += extrapolate_numbers_left(&vectors);
            acc_right += extrapolate_numbers_right(&vectors);
            (acc_left, acc_right)
        })
}

pub fn run() {
    // Did not like this one, the description misleads.
    let input = "./days/day9/input.txt";
    let (left, right) = day9(input);
    println!("\n day9 done with result_part_one {right} and two {left}.");
}
