#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    latest: Option<u32>,
    personal_best: Option<u32>,
    personal_top_three: Vec<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let latest = scores.last().copied();

        let mut personal_top_three = scores.to_vec();
        personal_top_three.sort_unstable();
        let personal_top_three: Vec<u32> = personal_top_three.into_iter().rev().take(3).collect();

        let personal_best = personal_top_three.first().copied();

        HighScores {
            scores,
            latest,
            personal_best,
            personal_top_three,
        }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.latest
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.personal_best
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.personal_top_three.clone()
    }
}
