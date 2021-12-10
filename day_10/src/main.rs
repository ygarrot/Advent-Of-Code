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

const FILENAME: &str = "./resources/day_10.txt";
// const FILENAME: &str = "./resources/example.txt";
// const FILENAME: &str = "./resources/test_1.txt";

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
    expect: char,
    found: char,
    i: usize,
    code: ValidationCode
}

// gerer le cas end bracket before start
fn match_b(buf: &String, mut token: Token) -> Token {
    if (token.i >= buf.len()){
        return token;
    }
    let cur = buf.as_bytes()[token.i];
    token.found = cur as char;

        // println!("{:?}", ret);
    if BRACK_LIST.contains_key(&token.found) {
        let ret = match_b(buf, Token {
            start: token.found,
            expect: BRACK_LIST[&token.found],
            i: token.i + 1,
            found: ' ',
            code: ValidationCode::VALID
        });
        match ret.code {
            ValidationCode::ERROR => return ret,
            ValidationCode::VALID => ()
        }
        token.i = ret.i;
    }
    if (!BRACK_LIST.contains_key(&token.found) && token.found != token.expect) {
        token.code = ValidationCode::ERROR;
        return token;
    }
    if token.found == token.expect {
        return token;
    }

    token.i += 1;
    match_b(buf, token)
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut i = 0;
    for line in reader.lines() {
        let un = line.unwrap();
        let start = &(un.as_bytes()[0] as char);
        let token =  Token { start: *start, expect: BRACK_LIST[&start], found: ' ', code: ValidationCode::VALID, i: 1};
        println!("line:{:?} ===========\n\t {:?}", i, un);
        println!("\t {:?}", match_b(&un, token));
        i+=1;
    }
    Ok(())
}
