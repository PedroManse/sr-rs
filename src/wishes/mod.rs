use axum::response::IntoResponse;
use axum::routing::{get, post};

pub mod models;
pub mod services;

use super::*;
pub struct WishesModule;

impl Routes for WishesModule {
    fn service() -> Router<PgPool> {
        use services::*;
        Router::new()
            .route("/list/:list_id", get(get_list))
    }
}

type Result<T> = std::result::Result<T, BackError>;

HTTPError!( backend BackError {
    SQLX = sqlx::Error,
    URL = url::ParseError
} );



impl APIError for BackError { }

impl IntoResponse for BackError {
    fn into_response(self) -> axum::response::Response {
        self.build_error()
    }
}

