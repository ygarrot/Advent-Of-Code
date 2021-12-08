use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use bit_reverse::ParallelReverse;

// In Rusbook: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// It is said that: "This process is more efficient than creating a String in memory especially working with larger files."

const FILENAME: &str = "./resources/day_3.txt";
const VEC_SIZE:usize = 12;
// const FILENAME: &str = "./resources/day_3.txt";
// const VEC_SIZE:usize = 12;
fn main() -> io::Result<()> {
    // const FILENAME: &str = "./resources/day_3.txt";

    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);

    exo1(& mut reader);
    Ok(())
}

fn count_odd(current: Vec<Vec<i64>>) -> Vec<i64>{
    let mut count:Vec<i64> = vec![0;VEC_SIZE];
    for vec in current {
        for n in 0..vec.len() {
            count[n] += vec[n];
        }
    }
    count
 }

fn to_num(tmp: Vec<i64>) -> i64 {
    let mut acc: i64 = -1;
    let mut vec = tmp.clone();
    vec.reverse();
    return vec.iter().fold(0, |total, x| {acc +=1; total + (x * i64::pow(2, acc as u32)) });
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut count:Vec<i64> = vec![0;VEC_SIZE];
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
    println!("{:?} {}", count, size);
    let maj: i64 = size / 2;
    count.reverse();

    let gamma_rate: i64 = to_num(count.iter().map(|x| (x > &maj) as i64).collect());

    let epsilon_rate: i64 = to_num(count.iter().map(|x| (x < &maj) as i64).collect());

    println!("{:?} {:?} {:?}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
    println!("------------------- PART 2 -----------------------");

    // ------------------- PART 2 -----------------------

    let mut ogr:Vec<Vec<i64>> = total.clone();
    let mut i: usize = 0;
    while ogr.len() >= 1 && i < VEC_SIZE {
        let maj: i64 = (ogr.len() / 2) as i64;
        let tmp: Vec<i64> = count_odd(ogr.clone())
            .iter()
            .map(|x| if (ogr.len() % 2) == 0 && x == &maj { 1 } else { (x > &maj) as i64 } )
            .collect();

        ogr.retain(|x| x[i] == tmp[i]);
        i += 1;
    }

    let mut csr:Vec<Vec<i64>> = total.clone();
    let mut i: usize = 0;
    while csr.len() > 1 && i < VEC_SIZE {
        let maj: i64 = (csr.len() / 2) as i64;

        let tmp: Vec<i64> = count_odd(csr.clone())
            .iter()
            .map(|x| if (csr.len() % 2) == 0 && x == &maj { 0 } else { if x > &maj {0} else {1} } )
            .collect();

        csr.retain(|x| x[i] == tmp[i]);
        i += 1;
    }

    let a = to_num(ogr[0].clone());
    let b = to_num(csr[0].clone());

    println!("ogr = {:?}", ogr);
    println!("ogr = {:?}", a);
    println!("csr = {:?}", b);
    println!("csr = {:?}", a * b);


    Ok(())
}
