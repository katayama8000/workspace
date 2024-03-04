#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use self::models::*;
use self::schema::bird::dsl::*;

mod database;
mod models;
mod schema;

#[get("/")]
fn index() -> Json<Vec<Bird>> {
    let connection = &mut database::establish_connection();
    bird.load::<Bird>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
