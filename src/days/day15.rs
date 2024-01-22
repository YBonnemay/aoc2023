use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

#[derive(Clone)]
struct Event {
    destination: usize,
    lens: Lens,
}

#[derive(Default, Clone, Debug)]
struct Lens {
    label: String,
    foc_len: Option<usize>,
}

#[derive(Default, Clone, Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn on_event(&mut self, lens: Lens) {
        let foc_len = lens.foc_len;
        let label = lens.label.clone();
        match foc_len {
            // foc_len : plus event
            Some(_) => match self.lenses.iter().position(|le| le.label == label.clone()) {
                Some(i) => self.lenses[i].foc_len = lens.foc_len,
                None => self.lenses.push(lens),
            },
            // No foc_len : minus event
            None => self.lenses.retain(|le| le.label != label.clone()),
        }
    }
}

fn process_input(inputs: Vec<String>) -> usize {
    inputs
        .iter()
        .map(|input| {
            let sequences = input.split(',').collect_vec();
            process_sequences(&sequences)
        })
        .sum()
}

fn process_label(sequence: &str) -> usize {
    let mut current_sum = 0;
    sequence.chars().for_each(|ch| {
        current_sum += ch as usize;
        current_sum *= 17;
        current_sum %= 256;
    });
    current_sum
}

fn parse_sequence(sequence: &str) -> Event {
    let toks = sequence.split(['=', '-']).collect_vec();
    let label = toks.first().expect("Err: no label");
    let destination = process_label(label);
    let foc_len = match toks.get(1) {
        Some(t) => ((**t).parse::<usize>()).ok(),
        None => None,
    };
    Event {
        destination,
        lens: Lens {
            label: String::from(*label),
            foc_len,
        },
    }
}

fn compute_result(lens_boxes: &[LensBox]) -> usize {
    lens_boxes
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.foc_len.expect("Err: no foc_len"))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn process_sequences(sequences: &[&str]) -> usize {
    let mut boxes = vec![LensBox::default(); 256];
    sequences.iter().for_each(|sequence| {
        // Bad, bad parsing.
        let event = parse_sequence(&String::from(*sequence));
        let lensbox = boxes
            .get_mut(event.destination)
            .expect("Err: no destination");
        lensbox.on_event(event.lens);
    });

    compute_result(&boxes)
}

pub fn run() {
    let input = "./days/day15/input.txt";
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day15 done with result {result}.");
}
