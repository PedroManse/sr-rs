use axum::response::IntoResponse;

pub mod models;
use super::*;

HTTPError!( backend BackError {
    SQLX = sqlx::Error
} );

HTTPError!( frontend FrontError {} );

pub enum WishError {
    BackError(BackError),
    FrontError(FrontError),
}

impl APIError for BackError { }
//impl HTMLError for FrontError {}

impl IntoResponse for WishError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BackError(x)=>x.build_error()
        }
    }
}

