use diesel::{prelude::*, r2d2};
use lazy_static::lazy_static;
use crate::models::api_error::ApiError;
use std::env;

type Pool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;
type Dbconnection = r2d2::PooledConnection<r2d2::ConnectionManager<MysqlConnection>>;

lazy_static! {

    static ref POOL: Pool = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
        let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
        Pool::new(manager).expect("Failed creating db pool")
    };

}

pub fn connection() -> Result<Dbconnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}