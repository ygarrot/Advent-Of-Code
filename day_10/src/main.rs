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

static VAL: Map<char, i64> = phf_map! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137
};

static COMPLETE_SCORE: Map<char, i64> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4
};

const FILENAME: &str = "./resources/day_10.txt";
// const FILENAME: &str = "./resources/example.txt";
// const FILENAME: &str = "./resources/test_1.txt";

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
// fn find_match(buf: &String, mut token: Token, complete: &mut Vec<Token>) -> Token {
//     if token.found == token.expect {
//         return token;
//     }
//     if BRACK_LIST.contains_key(&token.found) {
//         match_b(buf, token)
//     }
//     while 
//     token.i += 1;
//     complete.push(token)
//     match_b(buf, token)
// // }

#[derive(Debug, Clone)]
struct Lowest {
    start: i64,
    end: i64,
    diff: i64,
}

fn find_match(buf: &mut String) -> String {
    let mut i: usize = 0;
    let mut stop: bool = false;

    println!("{}", buf);
    while (true) {
        let mut best = Lowest {start: 999, end: 999, diff: 999};
        i=0;
        stop=true;

        while (true) {
            if (i >= buf.len()) {
                break;
            }
            let x = buf.chars().nth(i).unwrap();
            if BRACK_LIST.contains_key(&x) {
                let g_index = buf.find(BRACK_LIST[&x]);
                match g_index {
                    Some(n) => {
                        if i64::abs(i as i64 - n as i64) < best.diff {
                            stop=false;
                            best.start = i as i64;
                            best.end = n as i64;
                            best.diff = i64::abs(i as i64 - n as i64);
                        }
                    },
                    None => ()
                }
            }
            i+=1;
        }

        if (stop){
            break;
        } else {
            // println!("{:?} {:?} {:?}", best)
            println!(" ret {:?}", buf);
            buf.remove(best.start as usize);
            buf.remove((best.end - 1) as usize);
        }
    }
    buf.to_string()
}

fn match_b(buf: &String, mut token: Token) -> Token {
    if (token.i >= buf.len()){
        return token;
    }
    let cur = buf.as_bytes()[token.i];
    token.found = cur as char;
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
    if !BRACK_LIST.contains_key(&token.found) && token.found != token.expect {
        token.code = ValidationCode::ERROR;
        return token;
    }
    if token.found == token.expect {
        return token;
    }

    token.i += 1;
    match_b(buf, token)
}

fn calculate(mat: String) -> i64 {
    mat.chars().rev().fold(0, |total, x| (total * 5) + COMPLETE_SCORE[&BRACK_LIST[&x]] as i64)
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut i = 0;
    let mut res:i64 = 0;
    let mut part_2_res:Vec<i64> = Vec::new();

    for line in reader.lines() {
        let mut complete: Vec<Token> = Vec::new();
        let mut un = line.unwrap();
        let start = &(un.as_bytes()[0] as char);
        let token = Token { start: *start, expect: BRACK_LIST[&start], found: ' ', code: ValidationCode::VALID, i: 1};
        let result_token = match_b(&un, token);
        match result_token.code {
            ValidationCode::ERROR => (res += VAL[&result_token.found]),
            ValidationCode::VALID => {part_2_res.push(calculate(find_match(&mut un)));continue;}
        };
        // println!("line:{:?} ===========\n\t {:?}", i, un);
        // println!("\t {:?}", result_token);
        i+=1;
    }
    part_2_res.sort();
    // println!("{:?}", part_2_res);
    println!("{:?}", part_2_res[part_2_res.len()/2]);

    println!("{:?}", res);
    Ok(())
}
