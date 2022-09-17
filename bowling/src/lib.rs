use Frame::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Frame {
    Open(u16, u16),
    Spare(u16),
    Strike,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: [Option<u16>; 2],
    score: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score.is_some() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.current_frame {
            [None, _] => {
                if pins == 10 {
                    self.frames.push(Strike);
                } else {
                    self.current_frame[0] = Some(pins);
                }
            }
            [Some(pins1), None] => {
                if pins1 + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }

                if pins1 + pins == 10 {
                    self.frames.push(Spare(pins1));
                } else {
                    self.frames.push(Open(pins1, pins));
                }

                self.current_frame = [None, None];
            }
            _ => {}
        }

        if self.frames.len() >= 10 {
            let last_frame = self.frames.last().unwrap();
            match last_frame {
                Spare(roll) => {
                    if self.current_frame[0].is_some() {
                        self.frames.push(Open(self.current_frame[0].unwrap() - roll, 0));
                        self.calculate_score();
                    }
                },
                _ => self.calculate_score()
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.score
    }

    fn calculate_score(&mut self) {
        self.score = Some(self.frames.iter().enumerate().map(|(i, frame)| {
            match frame {
                Open(a, b) => a + b,
                Spare(_) => {
                    match self.frames[if i == 9 { i } else { i + 1 }] {
                        Open(a, _) => 10 + a,
                        Spare(a) => 10 + a,
                        Strike => 20,
                    }
                }
                Strike => 20,
            }
        }).sum());
    }
}


