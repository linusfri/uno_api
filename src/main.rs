use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use uno_api::routes::player;
use uno_api::models::state;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_data = state::State {name: "App".to_string()};

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(web::scope("/player").configure(player::player_config))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
