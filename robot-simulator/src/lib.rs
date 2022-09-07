// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        };

        Self { ..*self }
    }

    #[must_use]
    pub fn turn_left(&mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::East => self.direction = Direction::North,
            Direction::South => self.direction = Direction::East,
            Direction::West => self.direction = Direction::South,
        }

        Self { ..*self }
    }

    #[must_use]
    pub fn advance(&mut self) -> Self {
        match self.direction {
            Direction::North => self.position = (self.position.0, self.position.1 + 1),
            Direction::East => self.position = (self.position.0 + 1, self.position.1),
            Direction::South => self.position = (self.position.0, self.position.1 - 1),
            Direction::West => self.position = (self.position.0 - 1, self.position.1),
        };

        Self { ..*self }
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
