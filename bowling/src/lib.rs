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

        let last_frame = self.frames.last_mut();

        if let Some(last_frame) = last_frame {
            match last_frame[..] {
                [10] => { // Strike
                    if self.current_frame.is_empty() || self.current_frame.len() == 1 {
                        last_frame.push(pins);
                        self.current_frame.push(pins);
                        if self.current_frame.len() == 2 {
                            self.create_new_frame();
                        }
                    }
                }
                [first, second] => {
                    if first + second == 10 { // Spare
                        if self.current_frame.is_empty() {
                            last_frame.last_mut().unwrap().checked_add(pins).ok_or(Error::NotEnoughPinsLeft)?;
                            self.current_frame.push(pins);
                        } else {
                            self.current_frame.push(pins);
                            self.create_new_frame();
                        }
                    } else { // Open
                        self.current_frame.push(pins);
                        if self.current_frame.len() == 2 {
                            self.create_new_frame();
                        }
                    }
                }
                _ => {}
            }
        } else { // first frame
            self.current_frame.push(pins);
            if self.current_frame.len() == 2 || pins == 10 {
                self.create_new_frame();
            }
        }

        self.calculate_score();
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.score
    }

    fn create_new_frame(&mut self) {
        self.frames.push(self.current_frame.clone());
        self.current_frame.clear();
        println!("{} frame: {:?}", self.frames.len(), self.frames.last());
    }

    fn calculate_score(&mut self) {
        if self.frames.len() == 10 {
            let mut score = 0;
            for frame in self.frames.iter() {
                score += frame.iter().sum::<u16>();
            }
            self.score = Some(score);
        }
    }
}
