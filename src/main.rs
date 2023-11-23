use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Changed")
}

#[get("/fuck")]
async fn fuck() -> impl Responder {
    HttpResponse::Ok().body("Fuck")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fuck)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}