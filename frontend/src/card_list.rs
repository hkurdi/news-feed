use reqwest::Client;
use yew::{Component, Context, Html, html};
use yew::html::Scope;
use crate::card::Card;
use crate::props::{Article, Props};

pub struct CardList {
    news: Vec<Article>,
    link: Scope<CardList>
}

pub enum Msg {
    FetchReady(Result<Vec<Article>, reqwest::Error>),
    Fetch
}

impl Component for CardList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        link.send_message(Msg::Fetch);
        Self {
            news: vec![],
            link
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let link = self.link.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let client = Client::new();
                    let response = client.get("http://localhost:8000/news")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<Article>>()
                        .await;
                    link.send_message(Msg::FetchReady(response));
                });
                true
            }
            Msg::FetchReady(response) => {
                if let Ok(articles) = response {
                    self.news = articles;
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
                <h1 class="app-title">{"📰 Latest News"}</h1>
                <div class="news-list">
                    { for self.news.iter().map(|news| html! {
                        <Card
                            urlToImage={news.urlToImage.clone().unwrap_or_default()}
                            title={news.title.clone().unwrap_or_default()}
                            publishedAt={news.publishedAt.clone().unwrap_or_default()}
                            author={news.author.clone().unwrap_or_default()}
                            description={news.description.clone().unwrap_or_default()}
                            url={news.url.clone().unwrap_or_default()}
                        />
                    }) }
                </div>
            </div>
        }
    }
}
