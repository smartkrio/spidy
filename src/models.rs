extern crate diesel;

use crate::schema::movies;
// use crate::schema::movies::dsl::*;

use diesel::{Queryable, Insertable};

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub pic: String,
    pub description: String,
    pub published: bool,
}

#[derive(Queryable, Insertable)]
#[table_name="movies"]
pub struct NewMovie{
    pub title: String,
    pub pic: String,
    pub description: String,
}
