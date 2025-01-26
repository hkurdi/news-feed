use std::{env, time::Duration};
use std::io::{self, Write};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::{cache::Cache, model::{ApiResponse, Article}};

pub async fn fetch_news() -> Result<Vec<Article>, reqwest::Error> {
    dotenv().ok();
    let api_key = env::var("NEWS_API_KEY").expect("NEWS_API_KEY not found in .env");

    let url = format!(
        "https://newsapi.org/v2/everything?q=latest&from=2025-01-01&sortBy=publishedAt&apiKey={}",
        api_key
    );

    let client = Client::new();
    let response: Value = client
        .get(&url)
        .header("User-Agent", "News-Feed-App")
        .send()
        .await?
        .json()
        .await?;

    let api_response: ApiResponse = serde_json::from_value(response)
        .expect("Failed to parse API response");

    let news: Vec<Article> = api_response.articles
        .into_iter()
        .take(20)
        .map(|article| Article {
            title: article.title,
            description: article.description,
            author: article.author,
            publishedAt: article.publishedAt,
            url: article.url,
            urlToImage: article.urlToImage,
        })
        .collect();

    Ok(news)
}

pub async fn run() {
    let scheduler = JobScheduler::new().await.unwrap();

    let job = Job::new_repeated_async(Duration::from_secs(30), |_uuid, _l| {
        Box::pin(async {
            match fetch_news().await {
                Ok(news) => {
                    println!("✅ Fetched {} news articles.", news.len());
                    io::stdout().flush().unwrap(); 
    
                    if let Ok(cache) = Cache::new() {
                        if let Err(err) = cache.store_news(news) {
                            eprintln!("❌ Failed to store news in cache: {}", err);
                            io::stderr().flush().unwrap(); 
                        }
                    } else {
                        eprintln!("❌ Failed to initialize cache");
                        io::stderr().flush().unwrap();
                    }
                },
                Err(err) => {
                    eprintln!("❌ Failed to fetch news: {}", err);
                    io::stderr().flush().unwrap();
                },
            }
        })
    }).unwrap();

    scheduler.add(job).await.unwrap();

    scheduler.start().await.unwrap();

    println!("✅ Scheduler started. Fetching news every 30 seconds...");

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
