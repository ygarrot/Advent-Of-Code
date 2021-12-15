use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// struct Pos(usize, usize);
// struct Adj(i32, i32);
type Pos = (usize, usize);
type Adj = (i32, i32);

type Neighboors = Vec<Adj>;
type Positions = Vec<Pos>;

type Board = Vec<Positions>;
type Visited = Positions;

fn generate_coo() -> Neighboors {
    const COO: [i32;3] = [-1, 0, 1];

    let mut res: Neighboors = COO.iter().map(|y| COO.iter().map(|x| (*y, *x)).collect())
                            .collect::<Vec<Neighboors>>()
                            .into_iter().flatten().collect::<Neighboors>();
    res.remove(4);
    res
}
// const FILENAME: &str = "./resources/day_15.txt";
const FILENAME: &str = "./resources/example.txt";

// struct Board {
// }

fn walk(board: & Board, visited: &mut Visited, pos: Pos) -> i64 {
    let mut lowest = i64::MAX;
    let neighboors = generate_coo();

    // let add = |x: (i32, i32), y: (i32, i32)| (x.0 + y.0, x.1 + y.1);

    visited.push(pos);
    for neighboor in neighboors {
        let next_coo = ((pos.0 as i32 + neighboor.0) as usize, (pos.1 as i32 + neighboor.1) as usize);
        // let next_coo = add(pos as (i32, i32), neighboor) as (usize, usize);
        let (x, y) = next_coo;
        let next = board[y][x];
        if !visited.contains(&(x, y)) {
            lowest = i64::min(lowest, walk(board, visited, next))
        }
    }
    lowest
}

// struct Room<'a> {
//     weight: i64,
//     neighboor: Vec<&'a Room<'a>>
// }

// impl <'a> Room<'a> {
//     fn new() {
//     }

//     fn walk(self) -> i64 {
//         let mut lowest = i64::MAX;

//         for n in self.neighboor {
//             let current = n.walk();
//             lowest = i64::max(current, lowest);
//         }
//         lowest
//     }
// }

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<i64> {
    for line in reader.lines() {

    }
    Ok(40)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = exo1(& mut reader);
        assert_eq!(result.unwrap(), 40);
    }
}
