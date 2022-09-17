use Frame::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub enum Frame {
    Open(u16, u16),
    Spare(u16),
    Strike,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
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

        match self.frames.last_mut() {
            Some(Open(_, 0)) | Some(Spare(_)) | Some(Strike) => {
                if pins == 10 {
                    self.frames.push(Strike);
                    return Ok(());
                } else {
                    self.frames.push(Open(pins, 0));
                }
            }
            Some(Open(first, _)) => {
                let first = *first;

                match first + pins {
                    f if f > 10 => return Err(Error::NotEnoughPinsLeft),
                    10 => {
                        self.frames.pop();
                        self.frames.push(Spare(first))
                    }
                    _ => {
                        self.frames.pop();
                        self.frames.push(Open(first, pins));
                    }
                }
            }
            None => {
                if pins == 10 {
                    self.frames.push(Strike);
                } else {
                    self.frames.push(Open(pins, 0));
                }
            }
        };

        println!("{}, {:?}", self.frames.len(), self.frames.last().unwrap());
        if self.frames.len() > 10 {
            self.score = Some(self.frames.iter().map(|frame| match frame {
                Open(a, b) => a + b,
                Spare(a) => a + 10,
                Strike => 10,
            }).sum());
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.score
    }
}
