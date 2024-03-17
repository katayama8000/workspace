use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use dotenv::dotenv;
use sqlx::Pool;
use sqlx::Row;
use std::env;
use std::error;

#[derive(Clone)]
struct AppState {
    pool: Pool<sqlx::Postgres>,
}

async fn connect() -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    dotenv().ok();
    let db_user = env::var("DB_USER_DEV").expect("DB_USER must be set in the .env file");
    let db_password =
        env::var("DB_PASSWORD_DEV").expect("DB_PASSWORD must be set in the .env file");
    let db_host = env::var("DB_HOST_DEV").expect("DB_HOST must be set in the .env file");
    let db_name = env::var("DB_NAME_DEV").expect("DB_NAME must be set in the .env file");
    let database_url = format!(
        "postgres://{}:{}@{}/{}?sslmode=require",
        db_user, db_password, db_host, db_name
    );
    println!("Connecting to {}", database_url);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let pool = connect().await.expect("database should connect");
    let state = AppState { pool };

    let app = Router::new()
        .route(
            "/",
            get(|| async { (StatusCode::OK, "Hello, World from Axum!") }),
        )
        .route("/version", get(handle_get_version))
        .route("/user", get(handle_get_user))
        .route("/bill", get(handle_get_monthly_bill))
        .route("/bill/details", get(handle_get_bill_details))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handle_get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Version: {}", version)
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
