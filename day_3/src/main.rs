use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use bit_reverse::ParallelReverse;

// In Rusbook: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// It is said that: "This process is more efficient than creating a String in memory especially working with larger files."

fn main() -> io::Result<()> {
    // const FILENAME: &str = "./resources/day_3.txt";
    const FILENAME: &str = "./resources/test.txt";

    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);

    exo1(& mut reader);
    Ok(())
}

fn count_odd(current: Vec<Vec<i64>>) -> Vec<i64>{
    let mut count:Vec<i64> = vec![0;5];
    for vec in current {
        for n in 0..vec.len() {
            count[n] += vec[n];
        }
    }
    count
 }

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut count:Vec<i64> = vec![0;5];
    let mut size: i64 = 0;
    let mut total:Vec<Vec<i64>> = Vec::new();

    for line in reader.lines() {
        let current: Vec<i64> = line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64).collect();

        for n in 0..current.len() {
            count[n] += current[n];
        }
        total.push(current);

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
    println!("------------------- PART 2 -----------------------");

    // ------------------- PART 2 -----------------------

    let mut i: usize = 0;
    while true {
        if total.len() <= 1 {
            break;
        }
        let maj: i64 = (total.len() / 2).try_into().unwrap();
        let tmp: Vec<i64> = count_odd(total.clone())
            .iter()
            .map(|x| if (total.len() % 2) == 0 && x == &maj { 1 } else { (x > &maj) as i64 })
            .collect();

        total.retain(|x| x[i] == tmp[i]);
        println!("total = {:?}", total);
        i += 1;
    }
    Ok(())
}
