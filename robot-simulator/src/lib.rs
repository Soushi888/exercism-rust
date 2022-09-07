use Direction::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { position: (x, y), direction }
    }

    #[must_use]
    pub fn turn_right(&mut self) -> Self {
        match self.direction {
            North => self.direction = East,
            East => self.direction = South,
            South => self.direction = West,
            West => self.direction = North,
        };

       self.clone()
    }

    #[must_use]
    pub fn turn_left(&mut self) -> Self {
        match self.direction {
            North => self.direction = West,
            East => self.direction = North,
            South => self.direction = East,
            West => self.direction = South,
        }

        self.clone()
    }

    #[must_use]
    pub fn advance(&mut self) -> Self {
        match self.direction {
            North => self.position = (self.position.0, self.position.1 + 1),
            East => self.position = (self.position.0 + 1, self.position.1),
            South => self.position = (self.position.0, self.position.1 - 1),
            West => self.position = (self.position.0 - 1, self.position.1),
        };

        self.clone()
    }

    #[must_use]
    pub fn instructions(&self, instructions: &str) -> Self {
        instructions.chars().fold(*self, |mut robot, instruction| {
            match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
