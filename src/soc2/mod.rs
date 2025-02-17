use std::fmt::Display;

use axum::response::IntoResponse;
use crate::*;

pub mod models;
pub mod service;

type Result<T> = std::result::Result<T, APIError>;

macro_rules! HTTPError {
    { backend $name:ident $($variant:ident = $from:path),* } => {
        #[derive(thiserror::Error, Debug)]
        pub enum $name {
            $(
                #[error(transparent)]
                $variant(#[from] $from),
            )*
        }
    };
    { frontend $name:ident $($variant:ident),* } => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant($variant),
            )*
        }

    };
}

HTTPError!{ backend BackError
    SQLX = sqlx::Error,
    URL = url::ParseError
}

HTTPError!( frontend FrontError 
  ExampleError  
);


#[derive(Debug)]
pub struct ExampleError {
    info: i64,
}

pub enum SocError {
    BackError(BackError),
    FrontError(FrontError),
}

impl APIError for BackError { }
impl Display for FrontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontError::ExampleError( ExampleError{info}  ) => {
                write!(f, "Example error :(")?;
                write!(f, "extra info: {info}")
            }
        }
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

