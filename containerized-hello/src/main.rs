use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World from Acticx!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Hello, world from Local!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
