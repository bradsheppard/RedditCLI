use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NamesResponse {
    pub names: Vec<String>
}
