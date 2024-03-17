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
    let db_user = env::var("DB_USER").expect("DB_USER must be set in the .env file");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in the .env file");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set in the .env file");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set in the .env file");
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
        .route("/", get(|| async { println!("hello") }))
        .route("/items", get(handle_get_all_items))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handle_get_all_items(State(state): State<AppState>) -> impl IntoResponse {
    println!("GET /");
    let rows = match sqlx::query("SELECT * FROM playing_with_neon")
        .fetch_all(&state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to fetch items: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch").into_response();
        }
    };
    println!("Got {} items", rows.len());
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("Item {}: {}", id, name);
    }

    (StatusCode::OK).into_response()
}
