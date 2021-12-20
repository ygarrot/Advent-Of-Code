use std::fs::File;
use std::io::{self, prelude::*, BufReader};

type Pos = (usize, usize);
type Positions = Vec<Pos>;
type Vals = Vec<i64>;
type Map = Vec<Vals>;

const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    part_1(& mut BufReader::new(File::open(FILENAME)?)).unwrap();
    Ok(())
}

const LIGHT: char = '#';

// -2. -2

fn display(map: &Map) {
    for line in 0..map.len() {
        for x in 0..map[0].len() {
            let mut c = '#';

            if map[line][x] == 0 { 
                c = '.';
            }
            print!("{}", c);
        }
        println!();
    }
}
fn get_index(pos: &Vals) -> i64 {
    let mut acc: i64 = -1;
    pos.iter().fold(0, |total, x| {acc +=1; total + (x * i64::pow(2, acc as u32)) })
}

fn generate_infinity(map: Map) -> Map {
    let mut new_map = map.clone();
    let empty_line = vec![0;map[0].len() + 4];

    new_map = new_map.iter().map(|x| {
        let mut y = x.clone();
        y.insert(0, 0);
        y.insert(1, 0);
        y.insert(y.len(), 0);
        y.insert(y.len(), 0);
        y
    }).collect();
    new_map.insert(0, empty_line.clone());
    new_map.insert(1, empty_line.clone());
    new_map.insert(new_map.len(), empty_line.clone());
    new_map.insert(new_map.len(), empty_line.clone());
    new_map
}
// fn apply_algo(pos: Pos, algo) {
    // let nine_pix = vec![
    //                        (-1, -1), (-1, 0), (-1, 1),
    //                        (0, -1), (0, 0), (0, 1),
    //                        (1, -1), (1, 0), (1, 1),
    //     ];


    // for 
// }

fn part_1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut encryption_alg = String::new();
    let mut map: Map = Vec::new();

    reader.read_line(&mut encryption_alg)?;

    println!("alg: {}", encryption_alg);

    for line in reader.lines().skip(1) {
        let l = line.unwrap();
        println!("{}", l);
        map.push(l.chars().map(|x| if x == LIGHT {1} else {0}).collect());
     }
    println!("{:?}", map);
    display(&map);
    let new_map = generate_infinity(map);
    display(&new_map);
    Ok(())
}
