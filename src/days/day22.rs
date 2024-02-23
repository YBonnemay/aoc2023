// use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    ops::RangeInclusive,
};

use itertools::Itertools;

use crate::utils::input_process::input_to_lines;

type Point = (usize, usize, usize);
type Span = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
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
            // println!("moving {:?} down", self.ch);
            return true;
        }
        false
    }

    fn move_up(&mut self) -> bool {
        self.z.0 += 1;
        self.z.1 += 1;
        // println!("moving {:?} up", self.ch);
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
        if interesection {
            // println!(
            //     "interesection found between  {:?} and {:?} at height {:?}",
            //     self, brick, self.z.0
            // );
            // println!("interesection  brick : {:?}", brick);
        }
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

fn display_bricks(bricks: &Vec<Brick>) {
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    let mut z_max: usize = 0;

    for brick in bricks {
        x_max = x_max.max(brick.x.1);
        y_max = y_max.max(brick.y.1);
        z_max = z_max.max(brick.z.1);
    }

    let mut lines_x: Vec<Vec<char>> = vec![vec!['.'; x_max + 1]; z_max + 1];
    let mut lines_y: Vec<Vec<char>> = vec![vec!['.'; y_max + 1]; z_max + 1];
    for brick in bricks {
        for (i, j) in (brick.x.0..=brick.x.1).cartesian_product(brick.z.0..=brick.z.1) {
            lines_x[j][i] = brick.ch;
        }
    }

    for brick in bricks {
        for (i, j) in (brick.y.0..=brick.y.1).cartesian_product(brick.z.0..=brick.z.1) {
            lines_y[j][i] = brick.ch;
        }
    }

    for (i, (line_x, line_y)) in lines_x.iter().zip(lines_y.iter()).enumerate().rev() {
        let displayed_line = [
            line_x.iter().join(""),
            line_y.iter().join(""),
            i.to_string(),
        ]
        .join(" ");
        println!("{displayed_line}");
    }
    println!();
}

fn add_gravity(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // Bricks falling
    bricks.sort_by(|lhs, rhs| lhs.z.0.cmp(&rhs.z.0));
    // println!("bricks 1 {:?}", bricks);

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

    // display_bricks(&bricks);

    // println!("bricks 2 {:?}", bricks);
    // println!();
    bricks
}

fn get_supports<'a>(brick: &'a Brick, bricks: &'a mut Vec<Brick>) -> Vec<&'a mut Brick> {
    let mut brick = brick.clone();
    brick.move_down();
    bricks
        .iter_mut()
        .filter(|current_brick| current_brick.interesect(&mut brick))
        .collect_vec()
}

fn find_disintegrable_part_1(bricks: Vec<Brick>) -> usize {
    // Bricks by start of z
    // display_bricks(&bricks);
    let height = bricks
        .iter()
        .map(|brick| brick.z.1)
        .max()
        .expect("Err: no max");

    println!("height {:?}", height);
    let mut floors: Vec<Vec<Brick>> = vec![vec![]; height + 1];
    let mut ceilings: Vec<Vec<Brick>> = vec![vec![]; height + 1];

    for brick in bricks {
        floors[brick.z.0].push(brick.clone());
        ceilings[brick.z.1].push(brick);
    }

    let supporting_bricks: Vec<Brick> = vec![];
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
        .iter()
        .flatten()
        .filter(|brick| !brick.support)
        .count()
}

fn find_chain_part_2(bricks: Vec<Brick>) -> usize {
    // Bricks by start of z
    // display_bricks(&bricks);
    let height = bricks
        .iter()
        .map(|brick| brick.z.1)
        .max()
        .expect("Err: no max");

    println!("height {:?}", height);
    let mut floors: Vec<Vec<Brick>> = vec![vec![]; height + 1];
    let mut ceilings: Vec<Vec<Brick>> = vec![vec![]; height + 1];

    for brick in bricks {
        floors[brick.z.0].push(brick.clone());
        ceilings[brick.z.1].push(brick);
    }

    let mut chains: Vec<VecDeque<char>> = vec![];

    // Go up. Each step, add to list of fallen. supports are filtered with it
    //

    for i in (1..(floors.len())).rev() {
        let bricks_above = floors.get(i).expect("Err: no floors");
        let bricks_below = ceilings.get_mut(i - 1).expect("Err: no floors");
        for brick_above in bricks_above {
            let mut supports = get_supports(brick_above, bricks_below);
            // If only support, mark unsafe
            let len = supports.len();
            if len == 1 {
                let support = supports.get_mut(0).expect("Err: zero");
                println!("support.ch {:?}", support.ch);
                let key_of_support = chains.iter().position(|chain| {
                    if let Some(front) = chain.front() {
                        return front == &support.ch;
                    }
                    false
                });

                match key_of_support {
                    Some(key) => chains
                        .get_mut(key)
                        .expect("Err: nothing at key")
                        .push_front(support.ch),
                    None => chains.push(VecDeque::from([support.ch])),
                }
            }
        }
    }

    println!("chains {:?}", chains);

    ceilings
        .iter()
        .flatten()
        .filter(|brick| !brick.support)
        .count()
}

fn process_lines_part(lines: &Vec<String>) -> usize {
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
    // display_bricks(&bricks);

    let bricks = add_gravity(bricks);
    // display_bricks(&bricks);

    find_disintegrable_part_1(bricks)
    // find_chain_part_2(bricks)
}

pub fn run() {
    let input = "./days/day22/input.txt";
    let lines = input_to_lines(input);
    let result = process_lines_part(&lines);
    println!("\n day22 done, with result {result}.");
}
