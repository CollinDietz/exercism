#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    top_three: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort_by(|a, b| b.cmp(a));

        HighScores {
            scores: scores.to_vec(),
            top_three: sorted_scores.iter().take(3).copied().collect(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top_three.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_three.clone()
    }
}
