use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// In Rusbook: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// It is said that: "This process is more efficient than creating a String in memory especially working with larger files."

fn main() -> io::Result<()> {
    const FILENAME: &str = "./resources/day_1.txt";

    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);

    // exo1(& mut reader);
    exo2(& mut reader);
    Ok(())
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut count: i32 = 0;
    let mut last: i32 = std::i32::MAX;

    for line in reader.lines() {
        let current: i32 = line?.parse().unwrap();
        if current > last {
            count+=1;
        }
        last = current;
    }

    println!("value increased: {}", count);
    Ok(())
}

fn exo2<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut count: i32 = 0;
    let mut array: [i32; 3] = [0;3];
    let mut count: i32 = 0;
    let mut last: i32 = std::i32::MAX;

    for (i, line) in reader.lines().enumerate() {
        let current: i32 = line?.parse().unwrap();
        array[0] = current;

        if i > 2 {
            // println!("{}: {} // {:?}", i, current, array);
            let sum: i32 = array.iter().sum();

            if sum > last {
                count+=1;
            }
            last = sum;
        }
        array.rotate_right(1);
    }
    println!("value increased: {}", count);

    Ok(())
}
