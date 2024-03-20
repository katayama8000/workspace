use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::Pool;
use sqlx::Row;
use std::env;
use std::error;

use crate::config::connect;
mod config;

#[derive(Clone)]
struct AppState {
    pool: Pool<sqlx::Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let pool = connect().await.expect("database should connect");
    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(handle_get_version))
        .route("/env", get(handle_get_env))
        .route("/user", get(handle_get_user))
        .route("/bill", get(handle_get_monthly_bill))
        .route("/bill/details", get(handle_get_bill_details))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("expecr to bind port");
    println!(
        "🔥🔥🔥Listening on: {}🔥🔥🔥",
        listener.local_addr().expect("expect to get local address")
    );
    axum::serve(listener, app).await.expect("server should run");

    Ok(())
}

async fn handle_get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Version: {}", version)
}

async fn handle_get_env() -> String {
    let env = env::var("ENV").expect("ENV must be set in the .env file");
    format!("Environment: {}", env)
}

async fn handle_get_user(State(state): State<AppState>) -> impl IntoResponse {
    let email = "john.doe@example.com";
    let row = match sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Failed to fetch user: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch").into_response();
        }
    };

    let id: i32 = row.get("id");
    let name: String = row.get("name");
    println!("User {}: {}", id, name);

    (StatusCode::OK).into_response()
}

async fn handle_get_monthly_bill(State(state): State<AppState>) -> impl IntoResponse {
    let month = "June";

    let row = match sqlx::query("SELECT * FROM monthlybills WHERE month = $1")
        .bind(month)
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Failed to fetch monthly bill: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch").into_response();
        }
    };

    let id: i32 = row.get("id");
    // FIXME: This is not working
    // let amount: f64 = row.get::<_, f64>("total_amount").to_f64().unwrap_or(0.0);
    let status: String = row.get("status");

    println!("Monthly bill {}: {}", id, status);

    (StatusCode::OK).into_response()
}

async fn handle_get_bill_details(State(state): State<AppState>) -> impl IntoResponse {
    let id = 1;
    let bill_id = 1;

    let row = match sqlx::query("SELECT * FROM billdetails WHERE id = $1 AND bill_id = $2")
        .bind(id)
        .bind(bill_id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Failed to fetch bill details: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch").into_response();
        }
    };

    let id: i32 = row.get("id");
    let name: String = row.get("name");
    // FIXME: This is not working
    // let price: f64 = row.get("price");

    println!("Monthly bill {}: {}", id, name);

    (StatusCode::OK).into_response()
}
