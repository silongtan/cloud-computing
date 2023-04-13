use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use reqwest::Error;

#[get("/")]
async fn get_random_word() -> impl Responder {
    match fetch_random_word().await {
        Ok(word) => HttpResponse::Ok().body(format!("Random word: {}", word)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch random word"),
    }
}

async fn fetch_random_word() -> Result<String, Error> {
    let response = reqwest::get("https://random-word-api.vercel.app/api?words=1").await?;
    let word = response.text().await?;
    Ok(word)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_random_word))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}