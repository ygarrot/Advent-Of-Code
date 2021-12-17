#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// type Target = (i64, i64, i64, i64);

#[derive(Debug, PartialEq)]
struct Probe {
    x: i64,
    y: i64,
    x_velocity: i64,
    y_velocity: i64,
}

#[derive(Debug, PartialEq)]
struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Probe {
    fn tick(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        if (self.y_velocity >= 0) {
            self.y_velocity -= 1;
        }
        if (self.x_velocity != 0) {
            self.x_velocity -= if self.x_velocity > 0 {1} else {-1};
        }
    }
}

const FILENAME: &str = "./resources/example.txt";

type Coo = (i64, i64);
type Trajectory = Vec<Coo>;

fn display(trajectory: &Trajectory, target: &Target) {
    let max: Coo = trajectory.iter().fold((target.y_max,target.x_max),
    |total, x| (i64::max(total.0, x.0), i64::max(total.1, x.1)));
    let min: Coo = trajectory.iter().fold((target.y_min,target.x_min),
    |total, x| (i64::min(total.0, x.0), i64::min(total.1, x.1)));

    for y in  min.0..max.0 {
        for x in min.1..max.1 {
            if let Some(_) = trajectory.iter().find(|coo| coo.0 == x && coo.1 == y) {
                print!("#");
            } else if is_in_target(x, y, &target) {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }

}

fn is_in_target(x: i64, y: i64, target: &Target) -> bool {
    y > target.y_min
        && y < target.y_max
        && x > target.x_min
        && x < target.x_max
}

fn main() -> io::Result<()> {
    let mut target = get_target(& mut BufReader::new(File::open(FILENAME)?)).unwrap();
    let mut probe = Probe {x: 0, y: 0, x_velocity: 7, y_velocity: 2};
    let mut trajectory: Trajectory = Vec::new();

    println!("{:?}", target);

    while !is_in_target(probe.x, probe.y, &target) {
        probe.tick();
        trajectory.push((probe.x, probe.y));
        println!("{:?}", probe);
    }
    display(&trajectory, &target);
    Ok(())
}

fn get_target<R: BufRead>(reader: &mut R) -> io::Result<Target> {
    let mut result: Target = Target { x_min: 0, x_max: 0, y_min: 0, y_max: 0 };

    for line in reader.lines() {
        match scan_fmt!(&line?, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64) {
            Ok(rule) => result = Target{ x_min: rule.0, x_max: rule.1, y_min: rule.2, y_max: rule.3 },
            _ => ()
        }
     }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> io::Result<()> {
        assert_eq!(get_target(& mut BufReader::new(File::open("./resources/example.txt")?)).unwrap(),
        Target{x_min: 20, x_max: 30, y_min: -10, y_max: -5});
        Ok(())
    }
}
