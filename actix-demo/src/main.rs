#[allow(unused_imports)]
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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

#[get("/hello-lambda")]
async fn hello_lambda() -> HttpResponse {
    // Set the API Gateway endpoint URL
    let url = "https://ikpidx5h91.execute-api.us-east-1.amazonaws.com/default/hello-lambda";
    // // Set the Authorization header with the API Gateway API key
    // let api_key = "YOUR_API_KEY";
    // let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap();

    // Create a `reqwest` client
    let client = reqwest::Client::new();

    // Send an HTTP POST request to the API Gateway endpoint
    let response = client
        .post(url)
        .header("content-type", "text/html")
        .send()
        .await;

    // Check if the request was successful and return the response
    match response {
        Ok(res) => HttpResponse::Ok().body(res.text().await.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_random_word).service(hello_lambda))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}