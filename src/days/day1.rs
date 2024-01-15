use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Consting all that can be
const WORDS_1: [Word; 9] = [
    Word {
        word: "1",
        number: "1",
    },
    Word {
        word: "2",
        number: "2",
    },
    Word {
        word: "3",
        number: "3",
    },
    Word {
        word: "4",
        number: "4",
    },
    Word {
        word: "5",
        number: "5",
    },
    Word {
        word: "6",
        number: "6",
    },
    Word {
        word: "7",
        number: "7",
    },
    Word {
        word: "8",
        number: "8",
    },
    Word {
        word: "9",
        number: "9",
    },
];

const WORDS_2: [Word; 9] = [
    Word {
        word: "one",
        number: "1",
    },
    Word {
        word: "two",
        number: "2",
    },
    Word {
        word: "three",
        number: "3",
    },
    Word {
        word: "four",
        number: "4",
    },
    Word {
        word: "five",
        number: "5",
    },
    Word {
        word: "six",
        number: "6",
    },
    Word {
        word: "seven",
        number: "7",
    },
    Word {
        word: "eight",
        number: "8",
    },
    Word {
        word: "nine",
        number: "9",
    },
];

#[derive(Debug, Clone)]
struct Word<'a> {
    word: &'a str,
    number: &'a str,
}

#[derive(Debug)]
struct Position<'a> {
    word: &'a Word<'a>,
    first: usize,
    last: usize,
}

fn get_first_match(line: &str, words: &[Word]) -> u32 {
    // With big strings, should break early instead
    let findings: Vec<Position> = words
        .iter()
        .filter_map(|word| {
            let Some(first) = line.find(word.word) else {
                return None;
            };
            let Some(last) = line.rfind(word.word) else {
                return None;
            };
            Some(Position { first, last, word })
        })
        .collect();

    let Position {
        word: Word {
            number: number_first,
            ..
        },
        ..
    } = findings
        .iter()
        .min_by(|x, y| x.first.cmp(&y.first))
        .expect("Err: Could not find first");

    let Position {
        word: Word {
            number: number_last,
            ..
        },
        ..
    } = findings
        .iter()
        .max_by(|x, y| x.last.cmp(&y.last))
        .expect("Err: Could not find last");

    let together: u32 = format!("{number_first}{number_last}")
        .parse()
        .expect("Err: Could not parse number");

    together
}

fn day1(input: &str, words: &[Word]) -> u32 {
    let fpath = Path::new(input);
    let file = File::open(fpath).unwrap();
    let lines = io::BufReader::new(file).lines();

    let reduced: u32 = lines.fold(0, |acc, line| {
        acc + get_first_match(&line.expect("Err: could not use line"), words)
    });

    reduced
}

pub fn run() {
    let input = "./days/day1/input.txt";
    let words_1: Vec<Word> = Vec::from(WORDS_1);
    let mut words_2: Vec<Word> = Vec::from(WORDS_1);
    words_2.extend_from_slice(&WORDS_2);

    println!("run_day1");

    println!(
        "day 1 case 1 with input {} result: {}",
        input,
        day1(input, &words_1)
    );
    println!(
        "day 1 case 2 with input {} result: {}",
        input,
        day1(input, &words_2)
    );
}
