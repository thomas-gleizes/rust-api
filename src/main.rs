#[macro_use]
extern crate rocket;

use self::models::*;
use diesel::prelude::*;
use rust_api::*;

#[get("/")]
fn index() -> &'static str {
  "success true"
}

#[get("/posts")]
async fn show_posts() -> String {
  use self::schema::posts::dsl::*;

  let connection = &mut establish_connection();
  let results = posts
    .select((id, title, content, posted_at))
    .load::<Post>(&connection)
    .expect("Error loading posts");

  return format!("posts posted : {}", results.len());
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  let _rocket = rocket::build()
    .mount("/api", routes![index, show_posts])
    .launch()
    .await?;

  Ok(())
}
