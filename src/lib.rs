pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::{
    models::{Movie, NewMovie},
    schema::movies,
};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_movie(conn: &PgConnection, title: &str, pic: &str, description: &str) -> Movie {
    let new_movie = NewMovie {
        title: String::from(title),
        pic: String::from(pic),
        description: String::from(description),
    };

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .get_result(conn)
        .expect("Error saving new movie")
}


