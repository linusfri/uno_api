use actix_web::{web, HttpResponse, Result};

use crate::models::{game::{Game, NewGame}, api_error::ApiError};

pub async fn get_game(path: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let game = Game::get_game(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(game))
}

pub async fn create_game(game: web::Json<NewGame>) -> Result<HttpResponse, ApiError> {
    let game = game.into_inner();

    let res = Game::create_game(game).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_all_games() -> Result<HttpResponse, ApiError> {
    let games = Game::get_all_games().await?;
    Ok(HttpResponse::Ok().json(games))
}



pub fn game_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_game))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

    cfg.service(
        web::resource("/all")
            .route(web::get().to(get_all_games))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(get_game))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

}
