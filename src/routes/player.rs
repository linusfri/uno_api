use actix_web::{web, HttpResponse, Result};
use chrono::*;
use crate::models::{player::{Player, PartialPlayer}, api_error::ApiError};

pub async fn get_player(path: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let player = Player::get_player(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(player))
}

pub async fn get_players() -> Result<HttpResponse, ApiError> {
    let players = Player::get_all_players().await?;

    Ok(HttpResponse::Ok().json(players))
}

pub async fn get_player_points(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let points_for_player = Player::get_player_points(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(points_for_player))
}

pub async fn create_player(player: web::Json<PartialPlayer>) -> Result<HttpResponse, ApiError> {
    let player = player.into_inner();

    Player::create_player(player).await?;
    Ok(HttpResponse::Ok().json("player created"))
}

pub async fn delete_player(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    let affected_rows = Player::delete_player(id).await?;
    Ok(HttpResponse::Ok().json(format!("Player with id: {} deleted. Number of rows affected: {}", id, affected_rows)))
}

pub async fn update_player(id: web::Path<i32>, player: web::Json<PartialPlayer>) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let player = player.into_inner();

    let affected_rows = Player::update_player(id, player).await?;
    Ok(HttpResponse::Ok().json(format!("Player with id: {} updated. Number of rows affected: {}", id, affected_rows)))
}

pub fn player_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_player))
            .route(web::get().to(get_players))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(get_player))
            .route(web::delete().to(delete_player))
            .route(web::put().to(update_player))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

    cfg.service(
        web::resource("/{id}/points")
            .route(web::get().to(get_player_points))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}
