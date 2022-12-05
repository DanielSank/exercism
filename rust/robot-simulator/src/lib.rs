use strum_macros::FromRepr;


#[derive(Clone, Copy, PartialEq, Eq, Debug, FromRepr)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}


fn modulus(a: i8, b: i8) -> i8 {
    ((a % b) + b) % b
}


impl Direction{
    pub fn turn_right(self) -> Direction {
        Direction::from_repr(modulus(self as i8 + 1, 4) as usize).unwrap()
    }
    pub fn turn_left(self) -> Direction {
        Direction::from_repr(modulus(self as i8 - 1, 4) as usize).unwrap()
    }
    fn advance(&self) -> (i8, i8){
        use Direction::*;
        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }
}


pub struct Robot {
    direction: Direction,
    x: i32,
    y: i32,
}


impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{direction: d, x, y}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot {
            direction: self.direction.turn_right(),
            x: self.x,
            y: self.y,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot {
            direction: self.direction.turn_left(),
            x: self.x,
            y: self.y,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = self.direction.advance();
        Robot {
            direction: self.direction,
            x: self.x + x as i32,
            y: self.y + y as i32,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            robot = match instruction {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => panic!(),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
