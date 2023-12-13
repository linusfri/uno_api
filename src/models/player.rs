use std::fmt::Display;
use diesel::prelude::*;
use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Local, TimeZone, Datelike, Duration};

use crate::models::api_error::ApiError;
use crate::models::database;
use crate::schema::players::{self};
use crate::schema::games::{self};


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

    pub async fn get_player_points(id: i32) -> Result<i64, ApiError> {
        let res: Result<Vec<i64>, ApiError> = web::block(move || {
            let conn =  &mut database::connection().expect("Could not get db-connection");

            let current_time = Local::now();
            let lower = Local.with_ymd_and_hms(current_time.year(), current_time.month(), 1, 0, 0, 0).unwrap().naive_local();
            let upper = lower.checked_add_months(chrono::Months::new(1)).unwrap().checked_sub_signed(Duration::seconds(1));

            let res = games::table
                .select(games::all_columns)
                .filter(games::winner.eq(id).and(games::timestamp.between(lower, upper)))
                .count()
                .load::<i64>(conn)?;

            Ok(res)
        }).await?;
    
        Ok(res.expect("Error when getting player points")[0])
    }

    pub async fn get_all_players() -> Result<Vec<Player>, ApiError> {
        let players:Vec<Player> = web::block(move || {
            let conn = &mut database::connection().expect("Could not get db-connection");

            let players = players::table
                .load(conn);

            players
        }).await?.unwrap();

        // TODO, get all points here, though it seems like i cant get my head around joining with Diesel

        // let players = web::block(move || {
        //     let conn = &mut database::connection().expect("Could not get db-connection");

        //     let join = players::table.inner_join(games::table.on(winner.eq(players::id)));

        //     let players = join
        //         .select((players::all_columns, count(games::id)))
        //         .load::<PlayerWithPoints>(conn);
        //     players
        // }).await?.unwrap();
        
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

// impl ValidGrouping<players::name> for players::id {
//     type IsAggregate = is_aggregate::Yes;
// }

#[derive(Deserialize, Serialize, Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PartialPlayer {
    pub name: String
}

pub struct PlayerWithPoints {
    pub id: i32,
    pub name: String,
    pub registered: Option<NaiveDateTime>,
    pub points: i32
}