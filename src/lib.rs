pub mod crypt;
pub mod ecb;
pub mod jwt;
pub mod meet;
pub mod accounts;
//TODO better HTMLError trait

use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::Router;
use serde_json::json;
use tower_cookies::Cookies;
pub use uuid::Uuid;
const COOKIE_UUID_NAME: &str = "SRRS_USER_COOKIE";
const ARGON_SALT: &str = env!("ARGON_SALT");

pub trait Routes {
    fn service() -> Router<PgPool>;
}

pub trait HTMLNav {
    fn render(
        url: &str,
        cookies: &Cookies,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Markup> + Send;
}

pub trait ApiError: Sized + std::error::Error {
    fn build_error(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(json!({
                "status_code": StatusCode::BAD_REQUEST.as_u16(),
                "error_message": self.to_string(),
            }).to_string()))
            .unwrap()
    }
}

pub trait FrontError: Sized + std::error::Error {
    fn render_error(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(html!{
                h1 {"Erro:"}
                h2 { (self) }
                a href="/" {"home"}
            }.into_string()))
            .unwrap()
    }
}

pub fn hash<P>(password: P) -> [u8; 32]
where
    P: AsRef<[u8]>,
{
    let mut out = [0; 32];
    let a2 = argon2rs::Argon2::default(argon2rs::Variant::Argon2d);
    a2.hash(&mut out, password.as_ref(), ARGON_SALT.as_bytes(), &[], &[]);
    out
}

pub use sqlx::postgres::PgPool;
pub async fn acquire_pool() -> Result<PgPool, RootError> {
    dotenvy::dotenv()?;
    let url = std::env::var("DATABASE_URL").unwrap();
    PgPool::connect(&url).await.map_err(RootError::from)
}

#[derive(thiserror::Error, Debug)]
pub enum RootError {
    #[error(transparent)]
    EnvError(#[from] dotenvy::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

use maud::{html, Markup, Render, DOCTYPE};
pub const HTMX: JS = JS("/files/js/htmx.min.js");
pub const HYPER: JS = JS("/files/js/hyperscript.min.js");

pub struct CSS(pub &'static str);
impl Render for CSS {
    fn render(&self) -> Markup {
        html! { link rel="stylesheet" type="text/css" href=(self.0) {} }
    }
}

pub struct JS(pub &'static str);
impl Render for JS {
    fn render(&self) -> Markup {
        html! { script type="application/javascript" src=(self.0) {} }
    }
}

fn simple_nav_item(user_url: &str, check_url: &str, content: &str) -> Markup {
    let here = user_url == check_url;
    maud::html! {
        span {
            a
                ."current-page"[here]
                href=(check_url)
                {(content)}
        }
    }
}

pub async fn nav(
    url: &str,
    cookies: &Cookies,
    pool: &PgPool,
) -> Markup {
    maud::html! {
        nav class="center" {
            (simple_nav_item(
                url, "/", "home",
            ));
            //(simple_nav_item(
            //    url, "/meet/user", "Meet",
            //));
            (ecb::Nav::render(url, cookies, pool).await)
            (accounts::AccountModule::render(url, cookies, pool).await)
        }
    }
}
