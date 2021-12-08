use std::fs::File;
use std::io::{self, prelude::*, BufReader};
pub use self::submarine::Submarine;

mod submarine;

#[macro_use] extern crate scan_fmt;

fn main() -> io::Result<()> {
    const FILENAME: &str = "./resources/day_2.txt";

    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);

    exo1(& mut reader);
    Ok(())
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {

    let mut submarine = Submarine::new();

    for line in reader.lines() {
        let (s, x) = scan_fmt_some!(&line?, "{} {}", String, i32);
        match s.unwrap().as_ref() {
            "forward" => submarine.forward(x.unwrap()),
            "up" => submarine.up(x.unwrap()),
            "down" => submarine.down(x.unwrap()),
            _ => println!("unknown operation!")
        };
    }
    println!("{}", submarine.get_result());

    Ok(())
}

