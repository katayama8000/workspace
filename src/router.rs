use crate::{
    handler::{
        handle_get_bill_details, handle_get_env, handle_get_monthly_bill, handle_get_user,
        handle_get_version,
    },
    AppState,
};
use axum::{routing::get, Router};

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_version))
        .route("/env", get(handle_get_env))
        .route("/user", get(handle_get_user))
        .route("/bill/:month", get(handle_get_monthly_bill))
        .route("/bill/details", get(handle_get_bill_details)) // test
}
