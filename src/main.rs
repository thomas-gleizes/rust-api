#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::{content, status};

#[get("/")]
fn index() -> &'static str {
    "success  true"
}


#[get("/json")]
fn json() -> status::Custom<content::RawJson<&'static str>> {
    struct Data {
        success: bool,
    }

    struct JsonData {}

    status::Custom(Status::OK, Data { success: true })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/api", routes![index, json])
        .launch()
        .await?;

    Ok(())
}
