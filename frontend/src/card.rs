use yew::{Callback, Component, Context, Html, html};
use crate::props::Props;

pub struct Card {
    props: Props
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone()
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Props {
            urlToImage,
            title,
            publishedAt,
            author,
            description,
            url,
        } = &self.props;

        let image_src = if urlToImage.is_empty() {
            "https://upload.wikimedia.org/wikipedia/commons/1/14/No_Image_Available.jpg".to_string() 
        } else {
            urlToImage.clone()
        };

        let auth: String = if author.is_empty() {
            "📰 Unknown".to_string()
        } else {
            format!("📰 {}", author)
        };

        let onclick = {
            let url = url.clone();
            Callback::from(move |_| {
                web_sys::window().unwrap().location().set_href(&url).unwrap();
            })
        };

        html! {
            <div class="news-card" {onclick}>
                <img src={image_src} alt={title.clone()} class="news-image" />
                <div class="news-content">
                    <h4 class="news-title">{title.clone()}</h4>
                    <p class="news-description">{description.clone()}</p>
                    <p class="news-meta">{format!("📅 {}", publishedAt)}</p>
                    <p class="news-meta">{auth}</p>
                </div>
            </div>
        }
    }
}
