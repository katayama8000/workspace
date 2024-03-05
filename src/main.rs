use shuttle_secrets::SecretStore;

mod handler;
mod models;
mod schema;

use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use axum::{routing::get, Router};

struct AppState {
    pool: Arc<Mutex<MysqlConnection>>,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // let database_url = secret_store
    //     .get("DATABASE_URL")
    //     .expect("DATABASE_URL must be set");

    let pool = establish_connection();

    let state = Arc::new(AppState {
        pool: Arc::new(Mutex::new(pool)),
    });

    let router = Router::new()
        .route("/", get(hello_world))
        .with_state(state.clone());

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
