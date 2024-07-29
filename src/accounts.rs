use self::Error::*;
use crate::*;
use axum::{
    extract::*,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use maud::html;
use tower_cookies::{Cookie, Cookies};

pub fn service() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register_post))
        .route("/register", get(register_get))
        .route("/login", get(login_get))
        .route("/login", post(login_post))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
}

impl Render for Error {
    fn render(&self) -> Markup {
        let error_desc = match self {
            SqlError(e) => e.to_string(),
        };
        html! {
            h1 {"Erro:"}
            h2 { (error_desc) }
            a href="/" {"home"}
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.render().into_response()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct FormAccount {
    name: String,
    password: String,
}

pub async fn register_post(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Form(info): Form<FormAccount>,
) -> Result<Redirect, Error> {
    let hashed = hash(&info.password);
    let id = sqlx::query!(
        r#"
INSERT INTO inter.accounts (name, password)
VALUES ($1, $2)
RETURNING id"#,
        info.name,
        &hashed
    )
    .fetch_one(&pool)
    .await?
    .id;

    cookies.add(
        Cookie::build((COOKIE_UUID_NAME, id.to_string()))
            .path("/")
            .secure(false)
            .http_only(true)
            .into(),
    );
    Ok(Redirect::to("/"))
}

pub async fn register_get() -> Markup {
    maud::html! {
        head {
            (CSS("/files/style.css"));
        }
        body {
            (nav("/accounts/register"));
            div.center #"content" {
                form action="/accounts/register" method="POST" {
                    label for="name" {"Name:"}
                    input id="name" type="text" placeholder="name" name="name" {}
                    br { }
                    label for="passwd" {"Password:"}
                    input id="passwd" type="password" placeholder="password" name="password" {}
                    br { }
                    button {"Register"}
                }
            }
        }

    }
}

async fn login_get() -> Markup {
    html! {
        head {
            (CSS("/files/style.css"));
        }
        body {
            (nav("/accounts/login"));
            div.center #"content" {
                form action="/accounts/login" method="POST" {
                    label for="name" {"Name:"}
                    input id="name" type="text" placeholder="name" name="name" {}
                    br { }
                    label for="passwd" {"Password:"}
                    input id="passwd" type="password" placeholder="password" name="password" {}
                    br { }
                    button {"Login"}
                }
            }
        }
    }
}

pub async fn login_post(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Form(info): Form<FormAccount>,
) -> Result<Redirect, Error> {
    let hashed = hash(&info.password);
    let id = sqlx::query!(
        r#"
SELECT (id) FROM inter.accounts
WHERE (password=$1 AND name=$2)"#,
        &hashed,
        info.name,
    )
    .fetch_one(&pool)
    .await?
    .id;

    cookies.add(
        Cookie::build((COOKIE_UUID_NAME, id.to_string()))
            .path("/")
            .secure(false)
            .http_only(true)
            .into(),
    );
    Ok(Redirect::to("/"))
}
