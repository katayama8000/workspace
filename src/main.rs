use crate::config::connect;
use router::app;
use sqlx::Pool;
use std::error;

mod config;
mod handler;
mod router;

#[derive(Clone)]
struct AppState {
    pool: Pool<sqlx::Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let pool = connect().await.expect("database should connect");
    let state = AppState { pool };

    let app = app().with_state(state);

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
