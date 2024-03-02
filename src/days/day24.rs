use std::fmt;

use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

// const AREA_MIN: f64 = 7.0;
// const AREA_MAX: f64 = 27.0;

const AREA_MIN: f64 = 200000000000000.0;
const AREA_MAX: f64 = 400000000000000.0;

#[derive(Debug, Clone)]
struct Point {
    px: f64,
    py: f64,
    pz: f64,
}

#[derive(Debug, Clone)]
struct Vector {
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Debug, Clone)]
struct Stone {
    point: Point,
    vector: Vector,
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:}, {:}, {:} @ {:}, {:}, {:}",
            self.point.px,
            self.point.py,
            self.point.pz,
            self.vector.vx,
            self.vector.vy,
            self.vector.vz
        )
    }
}

fn get_places(inputs: &[String]) -> Vec<Stone> {
    inputs
        .iter()
        .map(|line| {
            let (p, v) = line.split_once('@').expect("Err: no @");
            let [px, py, pz] = p
                .trim()
                .split(", ")
                .map(|item| item.parse::<f64>().expect("Err: point not an f64"))
                .collect_vec()[0..=2]
            else {
                panic!("Err: places");
            };
            let [vx, vy, vz] = v
                .split(", ")
                .map(|item| item.trim().parse::<f64>().expect("Err: place not an f64"))
                .collect_vec()[0..=2]
            else {
                panic!("Err: places");
            };

            Stone {
                point: Point { px, py, pz },
                vector: Vector { vx, vy, vz },
            }
        })
        .collect_vec()
}

fn compute_intersection(lhs: &Stone, rhs: &Stone) -> Option<Point> {
    let p1 = &lhs.point;
    let p2 = &rhs.point;
    let n1 = &lhs.vector;
    let n2 = &rhs.vector;

    let dx = p2.px - p1.px;
    let dy = p2.py - p1.py;
    let det = n2.vx * n1.vy - n2.vy * n1.vx;
    let u = (dy * n2.vx - dx * n2.vy) / det;
    let v = (dy * n1.vx - dx * n1.vy) / det;

    // println!(" {:} ", u);
    // println!(" {:} ", v);
    if u < 0.0 || v < 0.0 {
        // println!("Past minus Hailstone A {:} ", lhs);
        // println!("Past minus Hailstone B {:} ", rhs);
        return None;
    }

    if u.is_nan() || v.is_nan() {
        // println!("Past nan Hailstone A {:} ", lhs);
        // println!("Past nan Hailstone B {:} ", rhs);
        return None;
    }

    if u.is_infinite() || v.is_infinite() {
        // println!("Never interact Hailstone A {:} ", lhs);
        // println!("Never interact Hailstone B {:} ", rhs);
        return None;
    }

    let px = p1.px + n1.vx * u;
    let py = p1.py + n1.vy * u;
    let pz = 0.0;

    if px < AREA_MIN || px > AREA_MAX || py < AREA_MIN || py > AREA_MAX {
        // println!("Out of area Hailstone A {:} ", lhs);
        // println!("Out of area Hailstone B {:} ", rhs);
        return None;
    }

    Some(Point { px, py, pz })
}

fn compute_interesections(inputs: Vec<Stone>) -> usize {
    inputs
        .clone()
        .iter()
        .combinations(2)
        .filter_map(|combination| {
            // if lhr == rhs {
            // return None;
            // }
            if let [lhr, rhs, ..] = combination[0..2] {
                match compute_intersection(lhr, rhs) {
                    Some(point) => {
                        // println!("Hailstone A {:} ", lhr);
                        // println!("Hailstone B {:} ", rhs);
                        // println!("intersection {:?} ", point);
                        // println!();
                        Some(point)
                    }
                    None => None,
                }
            } else {
                panic!("Err: refutable panic");
            }
        })
        .count()
}

fn process_input(inputs: Vec<String>) -> usize {
    let places = get_places(&inputs);
    compute_interesections(places)
}

pub fn run() {
    let input = "./days/day24/input.txt";
    // px py pz @ vx vy vz
    let data = input_to_lines(input);
    let result = process_input(data);
    println!("\n day24 done with result {result}.");
}

// NaN past
// inf never intersect
