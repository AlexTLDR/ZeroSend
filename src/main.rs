use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name =req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
