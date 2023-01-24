#[macro_use]
extern crate rocket;
extern crate diesel;

mod connection;
mod post;
mod schema;

use crate::connection::Connection;
use crate::post::Post;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::Config;

#[get("/")]
fn index() -> &'static str {
  "success true"
}

#[get("/posts")]
async fn show_posts(connection: Connection) -> String {
  Post::create(
    Post {
      id: 1,
      title: "title 1".to_string(),
      body: "content of posts".to_string(),
      posted: false,
    },
    &connection,
  );

  return "marche pas".to_string();
}

#[get("/posts/<post_id>")]
fn show_post(post_id: i32) -> Json<Post> {
  return Json(Post {
    id: post_id,
    title: "title 1".to_string(),
    body: "content of posts".to_string(),
    posted: false,
  });
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  let _rocket = rocket::build()
    .attach(Connection::fairing())
    .attach(AdHoc::config::<Config>())
    .mount("/api", routes![index, show_posts, show_post])
    .launch()
    .await?;

  Ok(())
}
