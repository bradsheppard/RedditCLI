use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthResponse {
    pub access_token: String
}
