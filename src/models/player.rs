use std::fmt::Display;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::models::api_error::ApiError;
use crate::models::database;
use crate::schema::players;


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
    
    pub async fn create_player(new_player: NewPlayer) -> Result<(), ApiError> {
        let conn = &mut database::connection()?;

        diesel::insert_into(players::table)
            .values(new_player).execute(conn)?;
        
        Ok(())
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

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPlayer {
    pub name: String
}