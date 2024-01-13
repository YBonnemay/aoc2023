use crate::utils::input_process::input_to_lines;

#[derive(Clone, Copy, PartialEq, Debug)]
enum EventId {
    Dot,
    Hash,
    Qmark,
}

#[derive(Clone, Copy, Debug)]
struct Event {
    id: EventId,
}

impl From<char> for Event {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Event { id: EventId::Dot },
            '#' => Event { id: EventId::Hash },
            '?' => Event { id: EventId::Qmark },
            _ => panic!("Err: unmanaged event {ch}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum StateId {
    Dot,
    Hash,
    End,
}

#[derive(Clone, Debug, Copy)]
struct State {
    id: StateId,
    value: i64,
}

impl State {
    fn new(ch: char) -> State {
        match ch {
            '.' => State {
                id: StateId::Dot,
                value: 0,
            },
            '#' => State {
                id: StateId::Hash,
                value: 0,
            },
            'e' => State {
                id: StateId::End,
                value: 0,
            },
            _ => panic!("Err: unmanaged state {ch}"),
        }
    }

    fn can_move_next(&self, event: &Event) -> bool {
        if self.id == StateId::End {
            return false;
        }

        match event.id {
            EventId::Dot => self.id == StateId::Dot,
            EventId::Hash => self.id == StateId::Hash,
            EventId::Qmark => true,
        }
    }
}

fn process_events(mut events: Vec<Event>, states: &Vec<State>) -> i64 {
    let mut states = states.clone();

    for event in events.iter_mut() {
        // caveman indices
        for i in (0..(states.len() - 1)).rev() {
            let [state, state_next] = states
                .get_many_mut([i, i + 1])
                .expect("Err: no current state");

            if state.value == 0 {
                continue;
            }

            if state_next.can_move_next(event) {
                state_next.value += state.value;
                if !(event.id == EventId::Qmark && state.id == StateId::Dot) {
                    state.value = 0;
                }
            } else if event.id == EventId::Hash || state.id != StateId::Dot {
                state.value = 0;
            }
        }
    }

    let line_result = states
        .iter()
        .rev()
        .take(3)
        .map(|state| state.value)
        .sum::<i64>();
    line_result
}

fn process_lines(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|input| {
            let (springs, groups) = input.split_once(' ').expect("Err: wrong input shape");
            const UNFOLDING: usize = 5;
            let groups = [groups; UNFOLDING].join(&','.to_string());

            let groups: Vec<&str> = groups.split(',').collect();
            let mut states: Vec<State> = vec![State {
                id: StateId::Dot,
                value: 1,
            }];
            for group in groups {
                let group_number = group.parse::<i64>().expect("Err: not a number");
                for _ in 0..group_number {
                    states.push(State::new('#'));
                }
                states.push(State::new('.'));
            }
            states.push(State {
                id: StateId::End,
                value: 0,
            });

            // Should pass iterator along instead
            let springs = [springs; UNFOLDING].join(&'?'.to_string());
            let events = springs.chars().map(Event::from).collect();
            process_events(events, &states)
        })
        .sum::<i64>()
}

// This one took some de-uglification.
// And still.
pub fn run() {
    let input = "./days/day12/input.txt";
    let data = input_to_lines(input);
    let result = process_lines(&data);
    println!("\n day12 done with result {result}.");
}
