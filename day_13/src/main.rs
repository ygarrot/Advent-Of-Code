extern crate regex;
use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[macro_use] extern crate scan_fmt;

const FILENAME: &str = "./resources/day_13.txt";
// const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

#[derive(Debug, Clone)]
struct Board {
    dot: Vec<(usize, usize)>,
    shape: (usize, usize)
}

impl Board {
    fn new (dot: Vec<(usize, usize)>, shape: (usize, usize)) -> Board {
        Board {
            dot: dot,
            shape: shape
        }
    }

    fn get_char(self, coo: (usize, usize)) -> char {
        return match self.dot.into_iter().find(|x| *x == coo){
           Some(_) => '#',
           None => ' '
        }
    }

    fn get_size(&mut self) {
        let get_highest = | a: (usize, usize), b: (usize, usize) | -> (usize, usize) { (cmp::max(a.0, b.0), cmp::max(a.1, b.1)) };
        self.shape = self.dot.iter().fold((0, 0), |total, k| get_highest(total, *k));
    }

    fn horizontal_fold(&mut self, n_fold: usize) {
        self.get_size();
        let condition = |x:(usize, usize)| -> bool {x.0 > n_fold};
        let fold = |x:(usize, usize)| -> (usize, usize) {(self.shape.0 - x.0, x.1)};
        self.dot = self.dot.iter().map(|x| if condition(*x) {fold(*x)} else {*x}).collect();
    }

    fn vertical_fold(&mut self, n_fold: usize) {
        self.get_size();
        let condition = |x:(usize, usize)| -> bool {x.1 > n_fold};
        let fold = |x:(usize, usize)| -> (usize, usize) {(x.0, self.shape.1 - x.1)};
        self.dot = self.dot.iter().map(|x| if condition(*x) {fold(*x)} else {*x}).collect();
    }

    fn display(self) {
        // println!("\n\n{:?}", self.shape);
        let map = (0..self.shape.1 + 1).map(|y|
                         (0..self.shape.0 + 1).map(|x| self.clone().get_char((x, y)))
                         .collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        for line in map.iter() {
            // println!("{:?}", line);
            println!("{:?}", line.iter().collect::<String>());
        }
    }
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let get_highest = | a: (usize, usize), b: (usize, usize) | -> (usize, usize) { (cmp::max(a.0, b.0), cmp::max(a.1, b.1)) };
    let mut t = Board::new(Vec::new(), (0,0));

    for line in reader.lines() {

        match line {
            Ok(l) => match scan_fmt!(&l, "{},{}", usize, usize) {
                Ok(x) => {
                    t.dot.push(x);
                    continue
                },
                _ => {
                    match scan_fmt!(&l, "fold along {}={}", char, usize) {
                        Ok(c) => {
                            match c.0 {
                                'x' => t.horizontal_fold(c.1),
                                'y' => t.vertical_fold(c.1),
                                _ => ()
                            }
                        },
                        _ => (t.shape = t.dot.iter().fold((0, 0), |total, k| get_highest(total, *k)))
                    }
                }
            },
            _ => ()
        }
    }
    let highest = t.dot.iter().fold((0, 0), |total, k| get_highest(total, *k));
    t.shape = highest;
    let mut dedup = Vec::new();

    for x in t.dot.iter() {
        match dedup.clone().into_iter().find(|a| a == x) {
            Some(_) => (),
            None => dedup.push(*x)
        }
    }
    println!("");
    t.display();
    Ok(())
}
