use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::models::api_error::ApiError;
use crate::models::database;
use crate::schema::games::{self};
use crate::schema::players::{self};

use super::player::{Player};

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Game {
    pub id: i32,
    pub winner: i32,
    pub timestamp: Option<NaiveDateTime>
}

#[derive(Deserialize, Serialize, Queryable)]
pub struct GameWithPlayer {
    pub game: Game,
    pub player: Player
}

#[derive(Deserialize, Serialize, Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::games)]
pub struct PartialGame {
    pub winner: i32
}

impl Game {
    pub async fn get_game(id: i32) -> Result<Self, ApiError> {
        let game = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");
    
            let game = games::table
                .filter(games::id.eq(id))
                .first(conn);

            game
        }).await?.unwrap();
        
        Ok(game)
    }
    
    pub async fn create_game(new_game: PartialGame) -> Result<String, ApiError> {
        let rows_affected = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");

            let res = diesel::insert_into(games::table)
                .values(new_game).execute(conn);

            res
        }).await??;
        

        Ok(format!("Game created, number of rows affected: {}", rows_affected))
    }

    pub async fn delete_game(id: i32) -> Result<usize, ApiError> {
        let rows_affected = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");
    
            let rows_affected = diesel::delete(
                games::table
                    .filter(games::id.eq(id))
            ).execute(conn)?;

            Ok(rows_affected)
        }).await?;

        rows_affected
    }

    pub async fn update_game(id: i32, game: PartialGame) -> Result<usize, ApiError> {
        let rows_affected = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");
    
            let rows_affected = diesel::update(games::table)
                .filter(games::id.eq(id))
                .set(game)
                .execute(conn)?;
            Ok(rows_affected)
        }).await?;

        rows_affected
    }

    pub async fn get_all_games() -> Result<Vec<GameWithPlayer>, ApiError> {
        let games = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");

            let games = games::table
                .inner_join(players::table)
                .load::<GameWithPlayer>(conn);

            let sorted_games = match games {
                Ok(mut games) => {
                    games.sort_by(|a, b| a.game.id.cmp(&b.game.id));

                    games
                },
                Err(_) => vec![]
            };

            sorted_games
        }).await?;
        
        Ok(games)
    }
    
}