use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const FILENAME: &str = "./resources/day_9.txt";
// const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

fn is_lower(board: Vec<Vec<i64>>, index: (usize, usize)) -> bool {
    let coo: [i32;3] = [-1, 0, 1];

    let (c_x, c_y) = index;
    for y in coo {
        let oy = c_y  as i32 + y;

        if oy < 0 || oy >= board.len() as i32 {
            continue
        }
        for x in coo {
            let ox = c_x  as i32 + x;

            if (x != 0 && y != 0) || (x == 0 && y == 0)
               || ox < 0 || ox >= board[0].len() as i32 {
                continue;
            }
            if board[oy as usize][ox as usize]
                <= board[c_y][c_x] {
                 return false;
            }
        }
    }
    true
}

fn find_lowest(board: Vec<Vec<i64>>) -> Vec<i64> {
    let mut ret: Vec<i64> = Vec::new();

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if is_lower(board.to_vec(), (x, y)){
                ret.push(board[y][x] + 1)
            }
        }
    }
    ret
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut total:Vec<Vec<i64>> = Vec::new();

    for line in reader.lines() {
        let current: Vec<i64> = line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        total.push(current);
    }
    // println!("{:?}", total[1][1]);
    let lowest = find_lowest(total);
    println!("{:?} {:?}", lowest, lowest.iter().sum::<i64>());
    Ok(())
}
