use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use ::phf::{phf_map, Map};
use std::mem;

use modular_bitfield::prelude::*;

#[bitfield]
pub struct Literal {
    packet_version: B3,
    packet_id: B3,
    part_1: B5,
    part_2: B5,
    part_3: B5,
    rest: B3,
}

const FILENAME: &str = "./resources/example.txt";


// static BITS: Map<char, &str> = phf_map! {
//     '4' => "literal",
//     _ => "operator",
// }

// return the length of each type of operator/literal

fn get_len(token: String) -> i64 {
    if token.len() <= 0 {
        return 0;
    }
    let packet_version = &token[0..=2];
    let type_id = &token[3..=5];
    
    if type_id == 4 {
        let len= mem::size_of::<Literal>() * 8;
        packet_version + get_len(token[len..])
    }

}

static BITS: Map<char, &str> = phf_map! {
    '0' => "0000",
    '1' => "0001",
    '2' => "0010",
    '3' => "0011",
    '4' => "0100",
    '5' => "0101",
    '6' => "0110",
    '7' => "0111",
    '8' => "1000",
    '9' => "1001",
    'A' => "1010",
    'B' => "1011",
    'C' => "1100",
    'D' => "1101",
    'E' => "1110",
    'F' => "1111",
};

fn convert(s: String) -> String  {
    s.chars().map(|c| BITS.get(&c).unwrap().to_string()).collect()
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    // println!("{:?}", );
    // exo1(& mut reader);
    Ok(())
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    for line in reader.lines() {
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(convert("D2FE28".to_string()), "110100101111111000101000");
    }
}
