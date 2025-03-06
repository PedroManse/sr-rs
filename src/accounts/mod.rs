use crate::*;
mod models;
mod service;
use axum::routing::{get, post};
use axum::response::IntoResponse;
use axum::Router;

pub use models::*;
pub use service::*;

pub struct AccountModule;

impl Module for AccountModule{
    fn service() -> Router<PgPool> {
        use service::*;
        Router::new()
            .route("/register", post(register_post))
            .route("/register", get(register_get))
            .route("/login", get(login_get))
            .route("/login", post(login_post))
    }
    async fn render(url: &str, cookies: &Cookies, pool: &PgPool) -> Markup {
        match accounts::get_acc(cookies, pool).await {
            Ok(acc) => html! {
                span.right {
                    "Olá"
                    a href="#" {(acc.name)}
                }
            },
            Err(_) => html! {
                span.right {
                    "Faça" a."current-page"[url=="/accounts/login"] href="/accounts/login" {"login"}
                    " ou " a."current-page"[url=="/accounts/register"] href="/accounts/register" {"Registre-se"}
                }
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AccError {
    #[error(transparent)]
    Api(AError),
    #[error(transparent)]
    Front(FError),
}

#[derive(thiserror::Error, Debug)]
pub enum AError {
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum FError {
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
    #[error(transparent)]
    JWTError(#[from] jwt::Error),
    #[error(transparent)]
    UUIDError(#[from] uuid::Error),

    #[error("Missing cookie")]
    MissingCookie,
}

impl crate::HTMLError for FError { }
impl crate::APIError for AError { }
impl IntoResponse for FError {
    fn into_response(self) -> axum::response::Response {
        self.render_error()
    }
}

impl IntoResponse for AccError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AccError::Front(e)=>e.render_error(),
            AccError::Api(e)=>e.build_error(),
        }
    }
}

