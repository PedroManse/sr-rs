use crate::*;
use std::fmt::{Display, self};
use axum::response::IntoResponse;

use self::models::{ItemRef, ItemType};

pub mod models;
pub mod items; // basic Item CRUD
pub mod service;

// export routes and navbar
pub struct SocModule;

type Result<T> = std::result::Result<T, BackError>;

#[derive(thiserror::Error, Debug)]
pub struct WrongItemType {
    got: ItemType,
    expected: ItemType,
    item_id: ItemRef,
}

impl Display for WrongItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WrongItemType: expected {:?}, but got {:?} for item #{}", self.expected, self.got, self.item_id.0)
    }
}

HTTPError!( backend BackError {

    SQLX = sqlx::Error,
    URL = url::ParseError,
    ItemType = WrongItemType
} );

HTTPError!( frontend FrontError  { } );

pub enum SocError {
    BackError(BackError),
    FrontError(FrontError),
}

impl APIError for BackError { }
impl Display for FrontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for FrontError { }
impl HTMLError for FrontError {}
impl IntoResponse for SocError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BackError(e)=>e.build_error(),
            Self::FrontError(e)=>e.render_error(),
        }
    }
}

