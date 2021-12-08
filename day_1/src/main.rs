use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// In Rusbook: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// It is said that: "This process is more efficient than creating a String in memory especially working with larger files."

fn main() -> io::Result<()> {
    const FILENAME: &str = "./resources/day_1.txt";

    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);

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
