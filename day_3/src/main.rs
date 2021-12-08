use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use bit_reverse::ParallelReverse;

// In Rusbook: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// It is said that: "This process is more efficient than creating a String in memory especially working with larger files."

fn main() -> io::Result<()> {
    const FILENAME: &str = "./resources/day_3.txt";

    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);

    exo1(& mut reader);
    Ok(())
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut count:Vec<i64> = vec![0;12];
    let mut size: i64 = 0;

    for line in reader.lines() {
        let current: Vec<i64> = line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64).collect();

        for n in 0..current.len() {
            count[n] += current[n];
        }

        size += 1;
    }
    let mut acc: i64 = -1;
    println!("{:?} {}", count, size);
    let maj: i64 = size / 2;
    count.reverse();
    let gamma_rate: i64 = count.iter()
        .map(|x| (x > &maj) as i64)
        .fold(0, |total, x| {acc +=1; total + (x * i64::pow(2, acc as u32)) });
    acc = -1;
    let epsilon_rate: i64 = count.iter()
        .map(|x| (x < &maj) as i64)
        .fold(0, |total, x| {acc +=1; total + (x * i64::pow(2, acc as u32)) });
    println!("{:?} {:?} {:?}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
    Ok(())
}
