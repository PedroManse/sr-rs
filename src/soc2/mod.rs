use axum::response::IntoResponse;
use crate::*;

pub mod models;
pub mod service;

#[derive(thiserror::Error, Debug)]
pub enum SocError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
}

type Result<T> = std::result::Result<T, SocError>;

impl ApiError for SocError { }
impl FrontError for SocError {}
impl IntoResponse for SocError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Self::SqlxError(_)=>self.build_error(),
            Self::UrlError(_)=>self.build_error(),
        }
    }
}
