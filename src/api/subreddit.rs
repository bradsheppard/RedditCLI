use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubredditResponse {
    pub data: SubredditData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubredditData {
    pub description: String,
    pub subscribers: usize
}
