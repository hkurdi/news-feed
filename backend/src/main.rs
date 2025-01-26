use cache::Cache;
use model::Article;
use rocket::{fairing::AdHoc, futures::lock::Mutex, get, http::Status, routes, serde::json::Json, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio::task;
use job::run;

mod model;
mod cache;
mod job;

#[get("/news")]
async fn news_handler(cache: &State<Mutex<Cache>>) -> Result<Json<Vec<Article>>, Status> {
    let store = cache.lock().await;

    match store.get_news() {
        Ok(None) => Err(Status::NotFound),
        Ok(Some(res)) => Ok(Json(res)), 
        Err(_) => Err(Status::InternalServerError),
    }
    
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    task::spawn(async {
        run().await;
    });

    let cache = match Cache::new() {
        Ok(cache) => Mutex::new(cache),
        Err(err) => {
            eprintln!("❌ Failed to initialize cache: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .unwrap();

    rocket::build()
        .manage(cache) 
        .mount("/", routes![news_handler])
        .attach(cors)
        .attach(AdHoc::config::<rocket::Config>())
        .launch()
        .await?;

    Ok(())
}
