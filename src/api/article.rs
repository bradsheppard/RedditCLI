use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticlesResponse {
    pub data: ArticleData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleData {
    pub children: Vec<ArticleLink>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleLink {
    pub data: ArticleLinkData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleLinkData {
    pub title: String,
    pub name: String
}
