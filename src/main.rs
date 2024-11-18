use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use dotenv::dotenv;
use uno_api::models::{database, state};
use uno_api::routes::{game, player};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_data = state::State {
        name: "App".to_string(),
    };
    let db_conn = &mut database::connection().expect("Could not get db-connection");
    db_conn
        .run_pending_migrations(FileBasedMigrations::from_path("./migrations").unwrap())
        .expect("Error processing migrations");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(web::scope("/player").configure(player::player_config))
            .service(web::scope("/game").configure(game::game_config))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
