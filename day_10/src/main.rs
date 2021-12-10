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

#[derive(Debug, Clone)]
enum ValidationCode {
    ERROR,
    VALID
}

#[derive(Debug, Clone)]
struct Token {
    start: char,
    expect: char,
    found: char,
    i: usize,
    code: ValidationCode
}

// gerer le cas end bracket before start
fn match_b(buf: &String, mut token: Token) -> Token {

    let cur = buf.as_bytes()[token.i];
    token.found = cur as char;

    // if (token.i >= buf.len()){
    //     return token;
    // }

    if cur == token.start as u8
        || (!BRACK_LIST.contains_key(&token.start) && cur != BRACK_LIST[&token.start] as u8) {
        token.code = ValidationCode::ERROR;
        return token;
    }
    if  cur == token.expect as u8 {
        return token;
    }
    let current_i = token.i;
    token.i += 1;
    let found = match_b(buf, token.clone());
    match found.code {
        ValidationCode::VALID => {
            token.i = current_i;
            let start = buf.as_bytes()[token.i + 1] as char;
            token.i += 1;
            // println!("{:?} {:?}", start, token.i);
            // if !BRACK_LIST.contains_key(&start){
            //     token.code = ValidationCode::ERROR;
            //     return token;
            // }
            token.start = start;
            token.expect = BRACK_LIST[&token.start];
            token = match_b(buf, token)
        },
        ValidationCode::ERROR => {}
    }
    found
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut i = 0;
    for line in reader.lines() {
        let un = line.unwrap();
        let start = &(un.as_bytes()[0] as char);
        let token =  Token { start: *start, expect: BRACK_LIST[&start], found: ' ', code: ValidationCode::VALID, i: 1};
        println!("line:{:?} ===========\n\t {:?}\n\t {:?}", i, un, match_b(&un, token));
        i+=1;
    }
    Ok(())
}
