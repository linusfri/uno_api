use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::models::api_error::ApiError;
use crate::models::database;
use crate::schema::games;

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Game {
    pub id: i32,
    pub winner: i32,
    pub timestamp: Option<NaiveDateTime>
}

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::games)]
pub struct NewGame {
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

    pub async fn get_all_games() -> Result<Vec<Game>, ApiError> {
        let games = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");

            let games = games::table
                .load(conn);

            games
        }).await?.unwrap();
        

        Ok(games)
    }
    
    pub async fn create_game(new_game: NewGame) -> Result<String, ApiError> {
        let rows_affected = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");

            let res = diesel::insert_into(games::table)
                .values(new_game).execute(conn);

            res
        }).await??;
        

        Ok(format!("Game created, number of rows affected: {}", rows_affected))
    }
}