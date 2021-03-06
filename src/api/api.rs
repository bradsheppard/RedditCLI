use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::env;

use crate::state::{Subreddit, Article, Comment};

use super::article::ArticlesResponse;
use super::comment::{ListingResponse, Replies};
use super::oauth::OAuthResponse;
use super::subreddit::SubredditResponse;
use super::names::NamesResponse;


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
            .await?
            .json::<NamesResponse>()
            .await;

        match resp {
            Ok(r) => Ok(r.names),
            Err(r) => Err(r)
        }
    }

    pub async fn get_subreddit_articles(&self, subbreddit: &str) -> Result<Vec<Article>, reqwest::Error> {
        let client = reqwest::Client::new();

        let resp = client.get(format!("https://oauth.reddit.com/r/{subbreddit}/new"))
            .header(AUTHORIZATION, "Bearer ".to_owned() + &self.token)
            .header(USER_AGENT, "rcli")
            .query(&[("limit", "100")])
            .send()
            .await?
            .json::<ArticlesResponse>()
            .await;

        match resp {
            Ok(r) => {
                let result = r.data.children.iter()
                    .map(|x| 
                         Article {
                             id: x.data.id.to_owned(), 
                             title: x.data.title.to_owned()
                         })
                    .collect();

                return Ok(result);
            }
            Err(r) => Err(r)
        }
    }

    pub async fn get_article_comments(&self, subreddit_name: &str, article_id: &str) -> Result<Vec<Comment>, reqwest::Error> {
        let client = reqwest::Client::new();

        let resp = client.get(format!("https://oauth.reddit.com/r/{subreddit_name}/comments/{article_id}"))
            .header(AUTHORIZATION, "Bearer ".to_owned() + &self.token)
            .header(USER_AGENT, "rcli")
            .send()
            .await?
            .json::<Vec<ListingResponse>>()
            .await;

        let mut result = Vec::new();

        match resp {
            Ok(r) => {
                for listing in r {
                    ApiClient::recurse_comments(listing, &mut result)
                }

                return Ok(result);
            }
            Err(r) => Err(r)
        }
    }   

    fn recurse_comments(listing: ListingResponse, result: &mut Vec<Comment>) {
        for child in listing.data.children {
            let mut comment = Comment {body: "".to_owned(), replies: Vec::new()};
            match child.data.body {
                Some(body) => {
                    comment.body = body;
                }
                None => {}
            }
            match child.data.replies {
                None => {}
                Some(replies) => {
                    match replies {
                        Replies::String(_) => {}
                        Replies::ListingResponse(r) => {
                            ApiClient::recurse_comments(r, &mut comment.replies);
                        }
                    }
                }
            }
            result.push(comment);
        }
    }

    pub async fn get_subreddit_details(&self, subbreddit: &str) -> Result<Subreddit, reqwest::Error> {
        let client = reqwest::Client::new();

        let resp = client.get(format!("https://oauth.reddit.com/r/{subbreddit}/about"))
            .header(AUTHORIZATION, "Bearer ".to_owned() + &self.token)
            .header(USER_AGENT, "rcli")
            .send()
            .await?
            .json::<SubredditResponse>()
            .await;

        match resp {
            Ok(r) => {
                let data = r.data;
                let subreddit_detail = Subreddit {
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
            .send()
            .await?
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
        assert!(subreddits.len() >= 1);
    }

    #[tokio::test]
    async fn test_get_comments() {
        let client = ApiClient::new().await.ok().unwrap();
        let comments = client.get_article_comments("vim", "6rf9z6").await.unwrap();
        assert!(comments.len() >= 1);
    }

    #[tokio::test]
    async fn test_get_subbreddit_articles() {
        let client = ApiClient::new().await.ok().unwrap();
        let articles = client.get_subreddit_articles("vim").await.unwrap();
        assert!(articles.len() >= 1);
    }

    #[tokio::test]
    async fn test_get_subbreddit_details() {
        let client = ApiClient::new().await.ok().unwrap();
        let details = client.get_subreddit_details("vim").await.unwrap();
        assert!(details.name == "vim");
        assert!(details.description.len() >= 1);
    }
}

