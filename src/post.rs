use diesel;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use schema::posts;

use crate::connection::Connection;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "posts"]
#[serde(crate = "rocket::serde")]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub posted: bool,
}

impl Post {
  pub fn create(post: Post, connection: &Connection) {}
}
