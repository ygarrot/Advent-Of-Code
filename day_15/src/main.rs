extern crate termion;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use termion::{color, style};

// struct Pos(usize, usize);
// struct Adj(i32, i32);
type Pos = (usize, usize);
type Adj = (i32, i32);

type Neighboors = Vec<Adj>;
type Positions = Vec<Pos>;

type Line = Vec<i32>;
type Board = Vec<Line>;
type Visited = Positions;

fn generate_coo() -> Neighboors {
    return vec![(0, 1), (1, 0)];
}

const FILENAME: &str = "./resources/day_15.txt";
// const FILENAME: &str = "./resources/example.txt";

// struct Board {
// }

fn display(board: &Board, visited: &Visited, pos: Pos) {
    println!("=================================");
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if visited.contains(&(x, y)){ 
                    print!("{}", color::Fg(color::Red));
            }
            if (x, y) == pos{ 
                    print!("{}", color::Fg(color::Blue));
            }
            print!(".{}", color::Fg(color::White));
            // print!(".{}", termion::clear::All);
        }
        println!();
    }
    println!("=================================");
}

fn walk(board: & Board, visited: & Visited, pos: Pos) -> i32 {
    let mut lowest = i32::MAX;
    let res = board[pos.0][pos.1];
    let neighboors = generate_coo();

    let mut past_visited = visited.clone();

    // display(board, visited, pos);
    past_visited.push(pos);
    if (pos.0 == board.len() -1 && pos.1 == board[0].len()-1) {
        return res;
    }
    for neighboor in neighboors.clone() {
        let next_coo = ((pos.0 as i32 + neighboor.0), (pos.1 as i32 + neighboor.1));
        let (x, y) = next_coo;
        if  y < 0 || y >= board.len() as i32 || x < 0 || x >= board[0].len() as i32 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if !visited.contains(&(x, y)) {
            past_visited.push((x, y));
        }
    }

    for neighboor in neighboors {
        let next_coo = ((pos.0 as i32 + neighboor.0), (pos.1 as i32 + neighboor.1));
        let (x, y) = next_coo;

        if  y < 0 || y >= board.len() as i32 || x < 0 || x >= board[0].len() as i32 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);

        if !visited.contains(&(x, y)) {
            // past_visited.push((x, y));
            // println!("{:?}", past_visited);
            // println!("{:?}", past_visited.len());
            // println!("{:?} => {:?}", pos, next_coo);
            lowest = i32::min(lowest, walk(board, &past_visited, (x, y)));
        }
    }
    if (lowest == i32::MAX) {
        return res;
    }

    res + lowest
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    part1(& mut reader);
    Ok(())
}

fn part1<R: BufRead>(reader: &mut R) -> io::Result<i32> {
    let mut board: Board = Vec::new();
    for line in reader.lines() {
        let current: Line = line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        board.push(current);
    }
    let res = walk(&board, &mut Vec::new(), (0, 0));
    println!("{:?}", res);
    Ok(40)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_init() {
    //     assert_eq!(final_coo, part1());
    // }

    #[test]
    fn test_coo() {
        let final_coo = vec![
           (-1, -1), (-1, 0), (-1, 1),
           (0, -1), (0, 1),
           (1, -1), (1, 0), (1, 1),
        ];
        assert_eq!(final_coo, generate_coo());
    }

    #[test]
    fn example() {
        let mut reader = BufReader::new(File::open(FILENAME).unwrap());
        let result = part1(& mut reader);
        assert_eq!(result.unwrap(), 40);
    }
}
