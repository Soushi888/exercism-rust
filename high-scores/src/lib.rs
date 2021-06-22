#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        self.0.as_slice()
    }

    pub fn latest(self) -> Option<u32> {
        self.0.last().cloned()
    }

    pub fn personal_best(self) -> Option<u32> {
        self.0.iter().max().cloned()
    }

    pub fn personal_top_three(self) -> Vec<u32> {
        let mut sorted_scores = self.0;
        sorted_scores.sort();

        sorted_scores.iter().rev().take(3).cloned().collect()
    }
}
