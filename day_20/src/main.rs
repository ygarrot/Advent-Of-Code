use std::fs::File;
use std::io::{self, prelude::*, BufReader};

type Pos = (usize, usize);
type Positions = Vec<Pos>;
type Vals = Vec<i64>;
type Map = Vec<Vals>;

// const FILENAME: &str = "./resources/example.txt";
const FILENAME: &str = "./resources/day_20.txt";
const LIGHT: char = '#';

fn main() -> io::Result<()> {
    part_1(& mut BufReader::new(File::open(FILENAME)?)).unwrap();
    Ok(())
}

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

fn get_index(pos: &Vals) -> usize {
    let mut tmp = pos.clone();
    let mut acc: i64 = -1;
    tmp.reverse();
    tmp.iter().fold(0, |total, x| {acc +=1; total + (x * i64::pow(2, acc as u32)) }) as usize
}

fn generate_infinity(map: Map, default:i64) -> Map {
    let mut new_map = map.clone();

    new_map = new_map.iter().map(|x| {
        let mut y = x.clone();
        y.insert(0, default);
        y.insert(1, default);
        y.insert(y.len(), 0);
        y.insert(y.len(), 0);
        y
    }).collect();
    let empty_line = vec![default;new_map[0].len()];
    new_map.insert(0, empty_line.clone());
    new_map.insert(1, empty_line.clone());
    new_map.insert(new_map.len(), empty_line.clone());
    new_map.insert(new_map.len(), empty_line.clone());
    new_map
}

fn get_image(map: &Map, pos: Pos, default: i64) -> Vals {
    let nine_pix: Vec<(i32, i32)> = vec![
                           (-1, -1), (-1, 0), (-1, 1),
                           (0, -1), (0, 0), (0, 1),
                           (1, -1), (1, 0), (1, 1),
        ];

    // let y_len = map.len();
    // let x_len = map[0].len() ;
    // let not_in_map = |x, y| x < 0 || x >= x_len as i32 || y < 0 || y >= y_len as i32;

    return nine_pix.iter().map(|coo| {
        let y = (pos.0 as i32 + coo.0);
        let x = (pos.1 as i32 + coo.1);
        return map[y as usize][x as usize];
        // return if not_in_map(x, y) {default} else {map[y as usize][x as usize]};
    }).collect::<Vals>();
}

fn apply_algo(map: &Map, alg: &Vals, len: Pos, default: i64) -> Map {
    let mut output_image = Vec::new();

    output_image.push(vec![0;map[0].len()]);
    for line in 1..map.len() - 1 {
        output_image.push(vec![default;map[0].len()]);
        for x in 1..map[0].len() - 1 {
            let image = get_image(map, (line, x), default);
            let index = get_index(&image);
            output_image[line][x] = alg[index as usize];
            // println!("{:?}{:?} index:{:?}", (line, x), image, index);
            // println!("algo: {:?}", alg[index as usize]);
        }
    }
    output_image.push(vec![default;map[0].len()]);
    output_image
}

fn to_numeric_vec(l: String) -> Vals {
    l.chars().map(|x| if x == LIGHT {1} else {0}).collect()
}

fn part_1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut first_line = String::new();
    let mut alg: Vals;
    let mut map: Map = Vec::new();

    reader.read_line(&mut first_line)?;
    alg = to_numeric_vec(first_line);

    for line in reader.lines().skip(1) {
        let l = line.unwrap();
        map.push(to_numeric_vec(l));
     }
    display(&map);
    let epoc = 2;
    for i in 0..epoc {
        println!("================ epoc {} ===============", i);
        map = apply_algo(&generate_infinity(map.clone(),i&1), &alg, (map.len(), map[0].len()), i & 1);
        display(&map);
    }

    let solve_part_1 = map.into_iter().flatten().fold(0, |total, x| total + x);
    println!("{:?}", solve_part_1);
    Ok(())
}

