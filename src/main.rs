#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod controller;
mod database;
mod models;
mod schema;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![
            controller::index,
            controller::new_sighting,
            controller::all_sightings,
            controller::delete_sighting
        ],
    )
}
