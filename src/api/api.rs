use serde::{Serialize, Deserialize};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::env;

use crate::state::SubredditDetail;


#[derive(Serialize, Deserialize, Debug)]
struct OAuthResponse {
    access_token: String
}

#[derive(Serialize, Deserialize, Debug)]
struct NamesResponse {
    names: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct DetailResponse {
    data: DetailData
}

#[derive(Serialize, Deserialize, Debug)]
struct DetailData {
    description: String,
    subscribers: usize
}

pub struct ApiClient {
    token: String,
}

impl ApiClient {
    pub async fn new<'a>() -> Result<ApiClient, &'a str> {
        let key = env::var("REDDIT_API_KEY");
        let secret = env::var("REDDIT_API_SECRET");
   
        match (key, secret) {
            (Ok(k), Ok(s)) => {
                let token  = ApiClient::get_bearer_token(&k, &s).await.unwrap();
                return Ok(ApiClient { token });
            }
            _ => Err("Need to set env vars")
        }
    }

    pub async fn get_subreddits(&self, search_term: &str) -> Result<Vec<String>, reqwest::Error> {
        let client = reqwest::Client::new();

        let resp = client.get(format!("https://oauth.reddit.com/api/search_reddit_names?query={search_term}"))
            .header(AUTHORIZATION, "Bearer ".to_owned() + &self.token)
            .header(USER_AGENT, "rcli")
            .send()
            .await
            .unwrap()
            .json::<NamesResponse>()
            .await;

        match resp {
            Ok(r) => Ok(r.names),
            Err(r) => Err(r)
        }
    }

    pub async fn get_subreddit_details(&self, subbreddit: &str) -> Result<SubredditDetail, reqwest::Error> {
        let client = reqwest::Client::new();

        let resp = client.get(format!("https://oauth.reddit.com/r/{subbreddit}/about"))
            .header(AUTHORIZATION, "Bearer ".to_owned() + &self.token)
            .header(USER_AGENT, "rcli")
            .send()
            .await
            .unwrap()
            .json::<DetailResponse>()
            .await;

        match resp {
            Ok(r) => {
                let data = r.data;
                let subreddit_detail = SubredditDetail {
                    name: subbreddit.to_owned(),
                    description: data.description,
                    subscriber_count: data.subscribers
                };

                return Ok(subreddit_detail);
            }
            Err(r) => Err(r)
        }
    }

    async fn get_bearer_token(key: &str, secret: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        
        let token_result = client.post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(key, Some(secret))
            .body("grant_type=client_credentials")
            .send().await
            .unwrap()
            .json::<OAuthResponse>()
            .await;

        match token_result {
            Ok(r) => Ok(r.access_token),
            Err(r) => Err(r)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::ApiClient;

    #[tokio::test]
    async fn test_get_subbreddits() {
        let client = ApiClient::new().await.ok().unwrap();
        let subreddits = client.get_subreddits("guitar").await.unwrap();
        assert_eq!(subreddits.len() >= 1, true);
    }

}

