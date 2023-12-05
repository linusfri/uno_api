use std::fmt::Display;
use diesel::prelude::*;
use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::models::api_error::ApiError;
use crate::models::database;
use crate::schema::players::{self};


#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub registered: Option<NaiveDateTime>
}

impl Player {
    pub async fn get_player(id: i32) -> Result<Self, ApiError> {
        let conn = &mut database::connection()?;

        let player = players::table
            .filter(players::id.eq(id))
            .first(conn)?;
        
        Ok(player)
    }
    
    pub async fn create_player(new_player: PartialPlayer) -> Result<(), ApiError> {
        let conn = &mut database::connection()?;

        diesel::insert_into(players::table)
            .values(new_player).execute(conn)?;
        
        Ok(())
    }

    pub async fn delete_player(id: i32) -> Result<usize, ApiError> {
        let rows_affected = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");
    
            let rows_affected = diesel::delete(
                players::table
                    .filter(players::id.eq(id))
            ).execute(conn)?;

            Ok(rows_affected)
        }).await?;

        rows_affected
    }

    pub async fn update_player(id: i32, player: PartialPlayer) -> Result<usize, ApiError> {
        let rows_affected = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");
    
            let rows_affected = diesel::update(players::table)
                .filter(players::id.eq(id))
                .set(player)
                .execute(conn)?;
            Ok(rows_affected)
        }).await?;

        rows_affected
    }

    pub async fn get_all_players() -> Result<Vec<Player>, ApiError> {
        let players = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");

            let players = players::table
                .load(conn);

            players
        }).await?.unwrap();
        

        Ok(players)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(
            f,
            "{:#?}",
            self
        );
        
        match res {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PartialPlayer {
    pub name: String
}