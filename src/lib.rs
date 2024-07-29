pub mod accounts;
pub mod ecb;

const COOKIE_UUID_NAME: &str = "SRRS_USER_COOKIE";
const ARGON_SALT: &'static str =
    dotenv_codegen::dotenv!("ARGON_SALT", "SALT must be defined for argon2d");

pub fn hash(password: &str) -> [u8; 32] {
    argon2rs::argon2d_simple(password, ARGON_SALT)
}

use sqlx::postgres::PgPool;
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

use maud::{html, Markup, Render};
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

pub fn nav(url: &str) -> Markup {
    let nav_link = |turl: &str, name: &str| {
        maud::html! {
            a."current-page"[url==turl] href={(turl)} {(name)}
        }
    };
    maud::html! {
        nav class="center" {
            (nav_link("/", "home"));
            (nav_link("/accounts/login", "login"));
            (nav_link("/accounts/register", "register"));
        }
    }
}
