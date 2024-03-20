use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use chrono_tz::{Asia::Tokyo, Tz};
use sqlx::Row;
use std::env;

use crate::AppState;

pub async fn handle_get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Version: {}", version)
}

pub async fn handle_get_env() -> String {
    let env = env::var("ENV").expect("ENV must be set in the .env file");
    format!("Environment: {}", env)
}

pub async fn handle_get_user(State(state): State<AppState>) -> impl IntoResponse {
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

pub async fn handle_get_monthly_bill(
    State(state): State<AppState>,
    Path(month): Path<String>,
) -> impl IntoResponse {
    println!("Month: {}", month);
    let month = "June";
    // get tokyo time
    let utc = Utc::now().naive_utc();
    let jst: DateTime<Tz> = Tokyo.from_utc_datetime(&utc).with_timezone(&Tokyo);
    println!("Tokyo time: {}", jst);
    let jst_month = jst.month();
    println!("Tokyo month: {}", jst_month);

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

pub async fn handle_get_bill_details(State(state): State<AppState>) -> impl IntoResponse {
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
