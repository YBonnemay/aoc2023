use crate::utils::input_process::input_to_lines;
use itertools::Itertools;
use std::collections::HashSet;

type Point = (usize, usize, usize);
type Span = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Brick {
    x: Span,
    y: Span,
    z: Span,
    ch: char,
    support: bool,
    supports: Vec<char>,
}

impl Brick {
    fn new(start: Point, end: Point, ch: char) -> Self {
        Brick {
            x: (start.0, end.0),
            y: (start.1, end.1),
            z: (start.2, end.2),
            ch,
            support: false,
            supports: vec![],
        }
    }

    fn move_down(&mut self) -> bool {
        if let Some(lower) = self.z.0.checked_sub(1) {
            self.z.0 = lower;
            self.z.1 = self.z.1.checked_sub(1).expect("Err: outside of usize");
            return true;
        }
        false
    }

    fn move_up(&mut self) -> bool {
        self.z.0 += 1;
        self.z.1 += 1;
        true
    }

    fn interesect(&self, brick: &Brick) -> bool {
        if self.ch == brick.ch {
            return false;
        }

        let interesection = [self.x, self.y, self.z]
            .iter()
            .zip([brick.x, brick.y, brick.z].iter())
            .all(|(lhs, rhs)| lhs.0 <= rhs.1 && rhs.0 <= lhs.1);
        interesection
    }
}

fn str_to_point(coordinates: &str) -> Point {
    coordinates
        .split(',')
        .map(|ch| ch.parse::<usize>().expect("Err: nan"))
        .collect_tuple()
        .expect("Err: not a tuple")
}

fn add_gravity(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by(|lhs, rhs| lhs.z.0.cmp(&rhs.z.0));
    // Bricks supporting
    let mut bricks_supporting = bricks.clone();
    bricks_supporting.sort_by(|lhs, rhs| rhs.z.1.cmp(&lhs.z.1));

    let len = bricks.len();

    for i in 0..len {
        'outer: while bricks[i].move_down() {
            for j in 0..=i {
                if bricks[i].interesect(&bricks[j]) {
                    bricks[i].move_up();
                    break 'outer;
                }
            }
        }
    }
    bricks
}

fn get_supports<'a>(brick: &'a Brick, bricks: &'a mut [Brick]) -> Vec<&'a mut Brick> {
    let mut brick = brick.clone();
    brick.move_down();
    bricks
        .iter_mut()
        .filter(|current_brick| current_brick.interesect(&brick))
        .collect_vec()
}

fn get_supports_as_char<'a>(brick: &'a Brick, bricks: &'a mut [Brick]) -> HashSet<char> {
    let mut brick = brick.clone();
    brick.move_down();

    let chars = bricks
        .iter_mut()
        .filter(|current_brick| current_brick.interesect(&brick))
        .map(|current_brick| current_brick.ch)
        .collect_vec();

    HashSet::from_iter(chars.iter().cloned())
}

fn get_all_supports(bricks: Vec<Brick>) -> Vec<Vec<Brick>> {
    let height = bricks
        .iter()
        .map(|brick| brick.z.1)
        .max()
        .expect("Err: no max");

    let mut floors: Vec<Vec<Brick>> = vec![vec![]; height + 1];
    let mut ceilings: Vec<Vec<Brick>> = vec![vec![]; height + 1];

    for brick in bricks {
        floors[brick.z.0].push(brick.clone());
        ceilings[brick.z.1].push(brick);
    }

    for i in 1..(floors.len()) {
        let bricks_above = floors.get(i).expect("Err: no floors");
        let bricks_below = ceilings.get_mut(i - 1).expect("Err: no floors");
        for brick_above in bricks_above {
            let mut supports = get_supports(brick_above, bricks_below);
            // If only support, mark unsafe
            let len = supports.len();
            if len == 1 {
                supports.get_mut(0).expect("Err: zero").support = true;
            }
        }
    }

    ceilings
}

#[allow(dead_code)]
fn find_disintegrable_part_1(bricks: Vec<Brick>) -> usize {
    let ceilings = get_all_supports(bricks);

    ceilings
        .iter()
        .flatten()
        .filter(|brick| !brick.support)
        .count()
}

fn build_fallen_list(bricks: Vec<Brick>, fallen_brick: Brick) -> HashSet<char> {
    let mut fallen_bricks: HashSet<char> = HashSet::from([fallen_brick.ch]);
    let height = bricks
        .iter()
        .map(|brick| brick.z.1)
        .max()
        .expect("Err: no max");

    let mut floors: Vec<Vec<Brick>> = vec![vec![]; height + 1];
    let mut ceilings: Vec<Vec<Brick>> = vec![vec![]; height + 1];

    for brick in bricks {
        floors[brick.z.0].push(brick.clone());
        ceilings[brick.z.1].push(brick);
    }

    for i in 1..(floors.len()) {
        let bricks_above = floors.get(i).expect("Err: no floors");
        let bricks_below = ceilings.get_mut(i - 1).expect("Err: no floors");
        for brick_above in bricks_above {
            let supports = get_supports_as_char(brick_above, bricks_below);
            if supports.is_subset(&fallen_bricks) {
                fallen_bricks.insert(brick_above.ch);
            }
        }
    }

    fallen_bricks
}

fn find_chain_part_2(bricks: Vec<Brick>) -> usize {
    let ceilings = get_all_supports(bricks.clone());

    let supports = ceilings
        .iter()
        .flatten()
        .filter(|brick| brick.support)
        .collect_vec();

    supports
        .iter()
        .map(|support| build_fallen_list(bricks.clone(), (*support).clone()).len() - 1)
        .sum()
}

fn process_lines_part(lines: &[String]) -> usize {
    let bricks = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let ch = char::from_u32(i as u32 + 65).expect("Err: nan");
            let (starts, ends) = line.split_once('~').expect("Err: no tilde");
            let (start_x, start_y, start_z) = str_to_point(starts);
            let (end_x, end_y, end_z) = str_to_point(ends);
            Brick::new((start_x, start_y, start_z), (end_x, end_y, end_z), ch)
        })
        .collect_vec();

    let bricks = add_gravity(bricks);

    find_chain_part_2(bricks)
}

// This one was fun.
// Code is messy - some low hanging cleans, easy facto between part 1 and 2.
// Shoud come back.
// I could compress that getting rid of structs, but I like my structs.
pub fn run() {
    let input = "./days/day22/input.txt";
    let lines = input_to_lines(input);
    let result = process_lines_part(&lines);
    println!("\n day22 done, with result {result}.");
}
