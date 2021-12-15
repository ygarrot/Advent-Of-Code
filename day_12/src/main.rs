use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, Clone)]
struct Room {
    pipe: Vec<String>,
    name: String,
    small: bool,
    visited: bool
}

impl Room {
    fn new(name: String) -> Room {
        Room {
            pipe: Vec::new(),
            name: name.to_string(),
            small: name.chars().fold(false, |res, c| res | c.is_lowercase()),
            visited: false
        }
    }
}

type Rooms = HashMap<String, Room>;

#[derive(Debug, Clone)]
struct Board {
    rooms: Rooms
}

impl Board{

    fn create_room(&mut self, name: &String) {
        self.rooms.entry(name.clone()).or_insert(Room::new(name.to_string()));
    }

    fn create_pipe(&mut self, a: & String, b: & String) {
        self.rooms.get_mut(a).unwrap().pipe.push(b.to_string());
        self.rooms.get_mut(a).unwrap().pipe.push(a.to_string());
    }

}

// const FILENAME: &str = "./resources/day_12.txt";
const FILENAME: &str = "./resources/example.txt";

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(FILENAME)?);

    exo1(& mut reader);
    Ok(())
}

fn rec(board: &mut Rooms, name: & String) {
    let room = board.get_mut(&name.clone()).unwrap();
    println!("{:?}", room);

    if (room.small && room.visited) {
        return ;
    }
    room.visited = true;
    for next in &room.pipe {
        rec(board, &next);
    }
}

fn exo1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut board: Board = Board{rooms:HashMap::new()};

    for line in reader.lines() {
        println!("{:?}", line);
        match scan_fmt!(&line?, "{}-{}", String, String) {
            Ok(rooms) => {
                board.create_room(&rooms.0);
                board.create_room(&rooms.1);
                board.create_pipe(&rooms.0, &rooms.1);
            },
            _ => ()
        }
    }
    let mut rooms: Rooms = board.rooms.clone();
    rec(&mut rooms, &"start".to_string());

    Ok(())
}
