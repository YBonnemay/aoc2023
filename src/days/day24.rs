use std::fmt;
use std::ops::Add;
use std::ops::Sub;

use crate::utils::input_process::input_to_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Point {
    px: i128,
    py: i128,
    pz: i128,
}

impl Point {
    fn sum(self) -> i128 {
        self.px + self.py + self.pz
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            px: self.px - other.px,
            py: self.py - other.py,
            pz: self.pz - other.pz,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            px: self.px + other.px,
            py: self.py + other.py,
            pz: self.pz + other.pz,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    vx: i128,
    vy: i128,
    vz: i128,
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            vx: self.vx - other.vx,
            vy: self.vy - other.vy,
            vz: self.vz - other.vz,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            vx: self.vx + other.vx,
            vy: self.vy + other.vy,
            vz: self.vz + other.vz,
        }
    }
}

fn gcd(x: i128, y: i128) -> i128 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl Vector {
    fn cross_product(self, other: Point) -> Vector {
        Vector {
            vx: self.vy * other.pz - self.vz * other.py,
            vy: self.vz * other.px - self.vx * other.pz,
            vz: self.vx * other.py - self.vy * other.px,
        }
    }

    fn cross_product_vector(self, other: Vector) -> Vector {
        Vector {
            vx: self.vy * other.vz - self.vz * other.vy,
            vy: self.vz * other.vx - self.vx * other.vz,
            vz: self.vx * other.vy - self.vy * other.vx,
        }
    }

    fn int_normalize(self) -> Vector {
        let divisor = gcd(gcd(self.vx, self.vy), self.vz);
        Vector {
            vx: self.vx / divisor,
            vy: self.vy / divisor,
            vz: self.vz / divisor,
        }
    }

    fn sum(self) -> i128 {
        self.vx + self.vy + self.vz
    }
}

#[derive(Debug, Clone, Copy)]
struct Stone {
    point: Point,
    vector: Vector,
}

impl Sub for Stone {
    type Output = Stone;

    fn sub(self, other: Stone) -> Stone {
        Stone {
            point: self.point - other.point,
            vector: self.vector - other.vector,
        }
    }
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
                .map(|item| item.parse::<i128>().expect("Err: point not an i128"))
                .collect_vec()[0..=2]
            else {
                panic!("Err: places");
            };

            let [vx, vy, vz] = v
                .split(", ")
                .map(|item| item.trim().parse::<i128>().expect("Err: place not an i128"))
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

fn sadly_brainy_solution_because_not_hunting_overflows(inputs: Vec<String>) -> i128 {
    let stones = get_places(&inputs);
    let stone_0 = stones.first().expect("Err: no first stone");
    let stone_1 = stones.get(1).expect("Err: no first stone");
    let stone_2 = stones.get(2).expect("Err: no first stone");
    let stone_centered_1 = *stone_1 - *stone_0;
    let stone_centered_2 = *stone_2 - *stone_0;

    // using mainly solution found online
    let p3 = stone_centered_1.point;
    let p4 = stone_centered_2.point;
    let v3 = stone_centered_1.vector;
    let v4 = stone_centered_2.vector;

    let q = v3.cross_product(p3).int_normalize();
    let r = v4.cross_product(p4).int_normalize();
    let s = q.cross_product_vector(r).int_normalize();

    let t = (p3.py * s.vx - p3.px * s.vy) / (v3.vx * s.vy - v3.vy * s.vx);
    let u = (p4.py * s.vx - p4.px * s.vy) / (v4.vx * s.vy - v4.vy * s.vx);

    let a = stone_0.point.add(p3).sum();
    let b = stone_0.point.add(p4).sum();
    let c = v3.sub(v4).sum();

    (u * a - t * b + u * t * c) / (u - t)
}

// By hook or by crook
// Should unify Point and Vector types.
pub fn run() {
    let input = "./days/day24/input.txt";
    let data = input_to_lines(input);
    let result = sadly_brainy_solution_because_not_hunting_overflows(data);
    println!("\n day24 done with result {result}.");
}
