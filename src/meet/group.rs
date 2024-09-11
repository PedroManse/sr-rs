#[derive(thiserror::Error, Debug)]
pub enum Error {
}

use crate::*;
use axum::{
//    extract::*,
//    response::IntoResponse,
//    routing::{get, post},
    Router,
};

pub fn service() -> Router<PgPool> {
    Router::new()
}

