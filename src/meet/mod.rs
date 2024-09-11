use crate::*;
use axum::Router;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    UserError(#[from] user::Error),
    #[error(transparent)]
    GroupError(#[from] group::Error),
}

pub mod user;
pub mod group;

pub fn service() -> Router<PgPool> {
    Router::new()
        .nest("/user", user::service())
        .nest("/group", group::service())
}
