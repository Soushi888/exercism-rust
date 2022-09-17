#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

type Frame = Vec<u16>;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: Frame,
    score: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }

        match self.current_frame.len() {
            0 => {
                self.current_frame.push(pins);

                let last_frame = self.frames.last_mut();
                if let Some(frame) = last_frame {
                    if frame.iter().sum::<u16>() == 10 {
                        frame.push(pins);
                    }
                }
            }
            1 => {
                let first_roll = self.current_frame[0];

                if first_roll + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }

                self.current_frame.push(pins);
                self.frames.push(self.current_frame.clone());
                self.current_frame.clear();
            }
            _ => unreachable!(),
        }


        if self.frames.len() == 10 {
            self.calculate_score();
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.score
    }

    fn calculate_score(&mut self) {
        let mut score = 0;
        for frame in self.frames.iter() {
            score += frame.iter().sum::<u16>();
        }
        self.score = Some(score);
    }
}
