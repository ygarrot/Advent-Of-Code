use std::cmp;
use std::fs::File;
use std::format;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

const FILENAME: &str = "./resources/day_14.txt";
// const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

fn create_pair(buf: & String, rule_insertion: & HashMap<String, char>) -> String {
    let slice = &buf[0..2];
    let (a, b) = (slice.chars().nth(0).unwrap(), slice.chars().nth(1).unwrap());

    if buf.len() <= 2 {
        return format!("{}{}{}", a, rule_insertion[slice], b);
    }
    return format!("{}{}{}", a, rule_insertion[slice], create_pair(&buf[1..].to_string(), rule_insertion));
}

fn get_result(formula: & String) {
    let mut result: Vec<(usize, char)> = Vec::new();

    let mut dedup: Vec<char> = Vec::new();

    for x in formula.chars() {
        match dedup.clone().into_iter().find(|a| a == &x) {
            Some(_) => (),
            None => dedup.push(x)
        }
    }

    println!("length: {:?}", dedup);
    for c in dedup {
        result.push((formula.matches(c).count(), c));
    }
    result.sort();
    // println!("{:?}", result);
    println!("{:?}", result[result.len() - 1].0 - result[0].0);
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    const steps: usize = 40;
    let mut rule_insertion: HashMap<String, char> = HashMap::new();
    let mut base_formula: String = String::new();

    reader.read_line(&mut base_formula);
    base_formula = base_formula.trim().to_string();

    for line in reader.lines() {
        match scan_fmt!(&line?, "{} -> {}", String, char) {
            Ok(rule) => {rule_insertion.insert(rule.0, rule.1);continue;},
            _ => ()
        }
    }
    for i in 0..steps {
        println!("{:?}", i);
        base_formula = create_pair(&base_formula, &rule_insertion);
        // println!("{:?}", base_formula);
        // get_result(&base_formula);
    }
    get_result(&base_formula);
    Ok(())
}
