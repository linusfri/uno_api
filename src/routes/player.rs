use actix_web::{web, HttpResponse, Result};

use crate::models::{player::{Player, NewPlayer}, api_error::ApiError};

pub async fn get_player(path: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let player = Player::get_player(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(player))
}

pub async fn create_player(player: web::Json<NewPlayer>) -> Result<HttpResponse, ApiError> {
    let player = player.into_inner();

    Player::create_player(player).await?;
    Ok(HttpResponse::Ok().json("player created"))
}

pub fn player_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_player))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(get_player))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}
