use serde::{Deserializer, Serializer};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    #[serde(rename = "Round 1")]
    pub round_1: String,
    #[serde(rename = "Round 2")]
    pub round_2: String,
    #[serde(rename = "Round 3")]
    pub round_3: String,
    #[serde(rename = "Round 4")]
    pub round_4: String,
    #[serde(rename = "Round 5")]
    pub round_5: String,
    #[serde(rename = "Champion")]
    pub champion: String,
    #[serde(rename = "Wild Card")]
    pub wild_card: String,
}
