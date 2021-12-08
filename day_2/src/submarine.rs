
pub struct Submarine {
    aim: i32,
    depth: i32,
    horizontal_pos: i32
}

// - forward X increases the horizontal position by X units.
// - down X increases the depth by X units.
// - up X decreases the depth by X units.

// ------------- Part two ---------------
// In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:

// - down X increases your aim by X units.
// - up X decreases your aim by X units.
// - forward X does two things:
//      - It increases your horizontal position by X units.
//      - It increases your depth by your aim multiplied by X.

impl Submarine {
    pub fn new() -> Submarine {
        Submarine{
            aim: 0,
            depth: 0,
            horizontal_pos: 0
        }
    }

    pub fn forward(&mut self, x: i32) {
        self.horizontal_pos += x;
        self.depth += self.aim * x;
    }

    pub fn up(&mut self, x: i32) {
        self.aim -= x;
        // self.depth -= x; // Part 1
    }

    pub fn down(&mut self, x: i32) {
        self.aim += x;
        // self.depth += x; // Part 1
    }

    pub fn get_result(&mut self) -> i32{
        self.depth * self.horizontal_pos
    }
}


