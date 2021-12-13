use std::collections::HashMap;
use std::cmp;
use std::iter::Map;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[macro_use] extern crate scan_fmt;

// const FILENAME: &str = "./resources/day_10.txt";
const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

#[derive(Debug, Clone)]
struct Board {
    // dot: Vec<i64>,
    dot: HashMap<(usize, usize), i32>,
    shape: (usize, usize)
}

// const X: usize = 1;
// const Y: usize = 0;

impl Board {
    fn new (dot: HashMap<(usize, usize), i32>, shape: (usize, usize)) -> Board {
        Board{
            dot:dot,
            shape:shape
        }
    }

    fn get_char(self, coo: (usize, usize)) -> char {
        return match self.dot.get(&coo){
           Some(_) => '#',
           None => '.'
        }
    }
    fn display(self) {
        println!("{:?}", self.shape);
        let map = (0..self.shape.1 + 1).map(|y|
                         (0..self.shape.0 + 1).map(|x| self.clone().get_char((x, y)))
                         .collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        for line in map.iter() {
            println!("{:?}", line);
        }
        // println!("{:?}", map);
    }
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    // let mut coo: Vec<(usize, usize)> = Vec::new();
    let mut coo = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(l) => match scan_fmt!(&l, "{},{}", usize, usize) {
                Ok(x) => {coo.insert(x, 1); continue},
                // Ok(x) => coo.push(x),
                _ => println!("invalid input: {:?}", &l)
            },
            _ => ()
        }
    }
    let get_highest = | a: (usize, usize), b: (usize, usize) | -> (usize, usize) { (cmp::max(a.0, b.0), cmp::max(a.1, b.1)) };
    let highest = coo.iter().fold((0, 0), |total, (k, _)| get_highest(total, *k));
    // println!("{:?}", highest);
    println!("{:?}", coo);
    let t = Board::new(coo, highest);
    t.display();
    // println!("{:?}", t);


    Ok(())
}
