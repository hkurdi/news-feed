use serde::{Deserialize, Serialize};
use yew::Properties;


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub description: String,
    pub urlToImage: String,
    pub publishedAt: String,
    pub author: String,
    pub url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub publishedAt: Option<String>,
    pub url: Option<String>,
    pub urlToImage: Option<String>
}