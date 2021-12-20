use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    part_1(& mut BufReader::new(File::open(FILENAME)?)).unwrap();
    Ok(())
}

fn part_1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut encryption_alg = String::new();
    reader.read_line(&mut encryption_alg)?;

    println!("alg: {}", encryption_alg);

    for line in reader.lines().skip(1) {
        println!("{}", line.unwrap());
     }
    Ok(())
}
