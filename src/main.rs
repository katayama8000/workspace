use axum::extract::State;
use shuttle_secrets::SecretStore;

mod handler;
mod models;
mod schema;

use handler::get_birds;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn get_connection_pool(database_url: String) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

use axum::{routing::get, Router};

#[derive(Clone)]
struct AppState {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let database_url = secret_store
        .get("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = get_connection_pool(database_url);
    let state = AppState { pool };
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/birds", get(get_birds))
        .with_state(state);
    Ok(router.into())
}

async fn hello_world(State(state): State<AppState>) -> &'static str {
    "Hello, world!"
}
