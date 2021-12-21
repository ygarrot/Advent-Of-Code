use std::fs::File;
use std::io::{self, prelude::*, BufReader};
#[macro_use] extern crate scan_fmt;

const FILENAME: &str = "./resources/day_21.txt";
// const FILENAME: &str = "./resources/example.txt";


struct DeterministicDice {
    counter: i64,
}

type Rolls = Vec<i64>;

pub trait Roll {
    fn roll(&mut self, epoch: i64) -> Rolls;
}

impl Roll for DeterministicDice {
    fn roll(&mut self, epoch: i64) -> Rolls {
        self.counter += epoch;
        ((self.counter-epoch)..self.counter).collect()
    }
}

#[derive(Debug)]
struct Player {
    rank: i64,
    position: i64,
    score: i64
}

impl Player {
    fn calc_score(&mut self, rolls: Rolls) {
        self.position = (self.position + rolls.iter().sum::<i64>()) % 10;
        self.score += if self.position == 0 {10} else {self.position};
    }
}

struct Game {
    players: Vec<Player>,
    dice: DeterministicDice,
}

impl Game {
    fn play(&mut self) -> &Player {
        let mut turn = 0;
        loop {
            self.players[turn].calc_score(self.dice.roll(3));
            if self.players[turn].score >= 1000 {
                turn = (turn + 1) % self.players.len();
                return &self.players[turn];
            }
            turn = (turn + 1) % self.players.len();
        }
    }
}

fn main() -> io::Result<()> {
    part_1(& mut BufReader::new(File::open(FILENAME)?)).unwrap();
    Ok(())
}


fn part_1<R: BufRead>(reader: &mut R) -> io::Result<()> {
    let mut game = Game { players: Vec::new(), dice: DeterministicDice { counter: 1 } };

    for line in reader.lines() {
        match scan_fmt!(&line?, "Player {} starting position: {}", i64, i64) {
            Ok((rank, position)) => game.players.push(Player{ rank, position, score: 0 }),
            _ => ()
        }
    }
    let winner = game.play();
    // println!("{:?}", game.players[0]);
    // println!("{:?}", game.dice.counter);
    // println!("{:?}", game.play().iter().fold(0, |total, p| total + p.));
    println!("result = {:?}", winner.score * (game.dice.counter - 1));

    Ok(())
}

