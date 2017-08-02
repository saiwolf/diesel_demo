#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{NewPost};

pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str, user_id: i32) -> usize {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
        user_id: user_id,
    };

    diesel::insert(&new_post)
        .into(posts::table)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connectiong to {}", database_url))
}
