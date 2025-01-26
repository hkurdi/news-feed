use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub publishedAt: Option<String>,
    pub url: Option<String>,
    pub urlToImage: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub articles: Vec<Article>
}