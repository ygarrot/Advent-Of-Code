// use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// use phf::{phf_map};
use ::phf::{phf_map, Map};


static BRACK_LIST: Map<char, char> = phf_map! {
    '{' => '}',
    '(' => ')',
    '<' => '>',
    '[' => ']',
};

// const FILENAME: &str = "./resources/day_10.txt";
const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

#[derive(Debug)]
enum ValidationCode {
    ERROR,
    VALID
}

#[derive(Debug)]
struct Token {
    start: char,
    // expect: char,
    found: char,
    i: usize,
    code: ValidationCode
}

// let BRACK_LIST: HashMap<char, char> = HashMap::from([
//     ('{', '}'),
//     ('(', ')'),
//     ('<', '>'),
//     ('[', ']'),
// ]);
// {([(<{}[<>[]}>{[]{[(<()>

// gerer le cas end bracket before start
fn match_b(buf: &String, token: Token) -> Token {

    let cur = buf.as_bytes()[token.i];
    if cur == token.start as u8 || (!BRACK_LIST.contains_key(&token.start) && cur != BRACK_LIST[&token.start] as u8) {
        return token;
    }
    if  cur == BRACK_LIST[&token.start] as u8 {
        return token;
    }
    match_b(buf, token)
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    for line in reader.lines() {
        let un = line.unwrap();
        let start = &(un.as_bytes()[0] as char);
        println!("{:?} {:?}", BRACK_LIST, start);
        let token =  Token { start: *start, found: BRACK_LIST[&start], code: ValidationCode::VALID, i: 1};
        println!("{:?}", match_b(&un, token));
    }
    Ok(())
}
