use diesel::prelude::*;
use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub content: String,
  pub posted_at: Timestamp,
}
