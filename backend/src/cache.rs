use redis::{Client, Commands};
use serde_json;
use crate::model::Article;

pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new() -> Result<Self, redis::RedisError> {
        let client = redis::Client::open("redis://127.0.0.1")?;
        Ok(Cache { client })
    }

    pub fn store_news(&self, news: Vec<Article>) -> Result<(), redis::RedisError> {
        let mut con = self.client.get_connection()?;
        let data = serde_json::to_string(&news).expect("Failed to serialize news data");
        con.set_ex::<_, _, ()>("LATEST_NEWS", data, 600)?;
        Ok(())
    }

    pub fn get_news(&self) -> Result<Option<Vec<Article>>, redis::RedisError> {
        let mut con = self.client.get_connection()?;
        let value: Option<String> = con.get("LATEST_NEWS")?;

        if let Some(json) = value {
            let articles: Vec<Article> = serde_json::from_str(&json).expect("Failed to deserialize news data");
            Ok(Some(articles))
        } else {
            Ok(None)
        }
    }
}
