pub mod accounts;
pub mod ecb;
pub mod jwt;
pub mod crypt;
pub mod meet;
//TODO all errors to string + StatusCode instead of markup

use tower_cookies::Cookies;
const COOKIE_UUID_NAME: &str = "SRRS_USER_COOKIE";
const ARGON_SALT: &'static str =
    dotenv_codegen::dotenv!("ARGON_SALT", "SALT must be defined for argon2d");

trait DescribeError {
    fn describe(&self) -> (String, axum::http::StatusCode);
    fn message(&self) -> String {
        self.describe().0
    }
    fn code(&self) -> axum::http::StatusCode {
        self.describe().1
    }
}

pub fn hash<P>(password: P) -> [u8; 32]
where P:AsRef<[u8]>
{
    let mut out = [0; 32];
    let a2 = argon2rs::Argon2::default(argon2rs::Variant::Argon2d);
    a2.hash(&mut out, password.as_ref(), ARGON_SALT.as_bytes(), &[], &[]);
    out
}

pub use sqlx::postgres::PgPool;
pub async fn acquire_pool() -> Result<PgPool, Error> {
    dotenvy::dotenv()?;
    let url = std::env::var("DATABASE_URL").unwrap();
    PgPool::connect(&url).await.map_err(Error::from)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
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

fn simple_nav_item(
    user_url: &str,
    check_url: &str,
    content: &str,
) -> Markup {
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
            (simple_nav_item(
                url, "/meet/user", "Meet",
            ));
            (ecb::get_nav(url));
            (accounts::get_nav(url, cookies, pool).await);
        }
    }
}
