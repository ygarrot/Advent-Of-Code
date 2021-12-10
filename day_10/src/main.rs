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

// const FILENAME: &str = "./resources/day_10.txt";
const FILENAME: &str = "./resources/example.txt";
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
    i_start: usize,
    start: char,
    expect: char,
    i: usize,
    found: char,
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

// fn find_match(buf: String) -> Vec<char> {
//     let mut ret: Vec<char>= buf.chars().collect();
//     let mut i: usize = 0;

//     println!("{}", buf);
//     while (true) {
//         if (i >= ret.len()){
//             break;
//         }
//         let x = ret[i];

//         if BRACK_LIST.contains_key(&x) {
//             let g_index = ret.iter().position(|c| *c == BRACK_LIST[&x]);
//             println!("{:?}", g_index);
//             match g_index {
//                 Some(n) => {
//                     println!("{:?} : {:?}", BRACK_LIST[&x], g_index);
//                     ret[g_index.unwrap()] = ' ';
//                     ret[i] = ' ';
//                 },
//                 None => ()
//             }
//         }
//         i+=1;
//     }
//     println!("{:?}", ret);
//     ret
// }

fn match_b(buf: &mut String, mut token: Token, res: &mut Vec<Token>, ) -> Token {
    res.push(token.clone());
    println!("\t{:?}", token);
    if token.i >= buf.len(){
        return token;
    }
    let cur = buf.as_bytes()[token.i];
    token.found = cur as char;
    if BRACK_LIST.contains_key(&token.found) {
        let ret = match_b(buf, Token {
            start: token.found,
            expect: BRACK_LIST[&token.found],
            i_start: token.i,
            i: token.i + 1,
            found: ' ',
            code: ValidationCode::VALID
        }, res);
        // match ret.code {
        //     ValidationCode::ERROR => return ret,
        //     ValidationCode::VALID => ()
        // }
        // token.i = ret.i;
    }
    if !BRACK_LIST.contains_key(&token.found) && token.found != token.expect {
        token.code = ValidationCode::ERROR;
        return token;
    }
    if token.found == token.expect {
        println!("REMOVE {:?}", buf);
        buf.remove(token.i);
        buf.remove(token.i_start);
        println!("AFTER {:?}", buf);
        token.i -= 1;
        // res.retain(|x| x.i_start == token.i_start);
        // res.retain(|x| x.i == token.i);
        // println!("{:?}", buf);
        return token;
    }
    // if (token.start )
    // token.i += 1;
    println!("AA{:?} {:?}\t",  buf.len(), token);
    while (token.i <= buf.len()){
        println!("{:?} {:?}\t",  buf.len(), token);
        token = match_b(buf, token, res);
        token.i += 1;
    }
    token
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut i = 0;
    let mut res:i64 = 0;
    let mut part_2_res:Vec<i64> = Vec::new();

    for line in reader.lines() {
        let mut complete: Vec<Token> = Vec::new();
        let mut un = line.unwrap();
        let start = &(un.as_bytes()[0] as char);
        let token = Token { start: *start, expect: BRACK_LIST[&start], found: ' ', code: ValidationCode::VALID, i: 1, i_start: 0};
        println!("line:{:?} ===========\n\t {:?}", i, un);
        let mut kept = Vec::new();
        let result_token = match_b(&mut un, token, &mut kept);
        match result_token.code {
            ValidationCode::ERROR => (res += VAL[&result_token.found]),
            ValidationCode::VALID => ()
        };
        println!("\t {:?}", res);
        // println!("\t {:?}", result_token);
        i+=1;
    }
    part_2_res.sort();
    println!("{:?}", part_2_res);
    // println!("{:?}", part_2_res[part_2_res.len()/2]);

    println!("{:?}", res);
    Ok(())
}
