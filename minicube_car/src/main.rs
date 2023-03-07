use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use car_generator::get_car_info;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Car Info Generator")
}

#[get("/{make}/{model}")]
async fn car(
    query: web::Path<(String, String)>,
) -> impl Responder {
    let (make, model) = query.into_inner();
    let info = get_car_info(make.to_string(), model.to_string()).await;
    HttpResponse::Ok().json(info)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(car))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}