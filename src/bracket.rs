use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    #[serde(rename = "Round 1")]
    pub round_1: Vec<String>,
    #[serde(rename = "Round 2")]
    pub round_2: Vec<String>,
    #[serde(rename = "Round 3")]
    pub round_3: Vec<String>,
    #[serde(rename = "Round 4")]
    pub round_4: Vec<String>,
    #[serde(rename = "Round 5")]
    pub round_5: Vec<String>,
    #[serde(rename = "Champion")]
    pub champion: Option<String>,
    #[serde(rename = "Wild Card")]
    pub wild_card: Option<String>,
}

impl Bracket {
    pub fn new(first_round: Vec<String>) -> Bracket {
        Bracket {
            round_1: first_round,
            round_2: vec![],
            round_3: vec![],
            round_4: vec![],
            round_5: vec![],
            champion: None,
            wild_card: None,
        }
    }

    fn compare_round(official: Vec<String>, user: Vec<String>) -> i32 {
        let mut round_score = 0;
        for i in 0..official.len() {
            if official[i] == user[i] {
                round_score += 10;
            }
        }
        round_score
    }

    pub fn calculate_score(&self, user_bracket: Bracket) -> i32 {
        let mut total_score = 0;

        let official = self.clone();
        total_score += Self::compare_round(official.round_1, user_bracket.round_1);
        total_score += Self::compare_round(official.round_2, user_bracket.round_2);
        total_score += Self::compare_round(official.round_3, user_bracket.round_3);
        total_score += Self::compare_round(official.round_4, user_bracket.round_4);
        total_score += Self::compare_round(official.round_5, user_bracket.round_5);

        if self.wild_card == user_bracket.wild_card {
            total_score += 20;
        }
        if self.champion == user_bracket.champion {
            total_score += 50;
        }
        total_score
    }
}
