use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// const FILENAME: &str = "./resources/day_10.txt";
const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

fn exo1<R: BufRead>(reader: &mut R) {
    for line in reader.lines() {
        println!("{:?}",line);
    }
}
