use axum::extract::State;
use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::{models::*, AppState};

pub async fn get_birds(State(state): State<AppState>) -> String {
    use crate::schema::bird::dsl::bird;
    let mut connection = state.pool.get().unwrap();
    let results = bird
        .load::<Bird>(&mut connection)
        .expect("Error loading birds");

    "Hello, world!".to_string()
    // Ok(Json(results))
}

pub async fn new_sighting(
    State(state): State<AppState>,
    sighting: Json<BirdSightingInput>,
) -> Json<BirdSighting> {
    use crate::schema::bird_sighting;

    let mut connection = state.pool.get().unwrap();
    diesel::insert_into(bird_sighting::table)
        .values(sighting.into_inner())
        .execute(&mut connection)
        .expect("Error adding sighting");

    Json(
        bird_sighting::table
            .order(bird_sighting::id.desc())
            .first(&mut connection)
            .unwrap(),
    )
}

pub async fn all_sightings(
    State(state): State<AppState>,
    bird: Option<i32>,
) -> Json<Vec<BirdSighting>> {
    let connection = &mut state.pool.get().unwrap();
    use crate::schema::bird_sighting::dsl::{bird_id, bird_sighting};

    let query_result: QueryResult<Vec<BirdSighting>> = match bird {
        Some(id) => bird_sighting.filter(bird_id.eq(id)).load(connection),
        None => bird_sighting.load(connection),
    };

    query_result.map(Json).expect("Error loading sightings")
}

pub async fn delete_sighting(State(state): State<AppState>, sighting_id: i32) -> NoContent {
    use crate::schema::bird_sighting::dsl::*;

    let connection = &mut state.pool.get().unwrap();
    diesel::delete(bird_sighting.filter(id.eq(sighting_id)))
        .execute(connection)
        .expect("Error deleting sighting");

    NoContent
}
