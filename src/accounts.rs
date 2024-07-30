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
    #[error(transparent)]
    JWTError(#[from] jwt::Error),
    #[error(transparent)]
    UUIDError(#[from] uuid::Error),

    #[error("Missing cookie")]
    MissingCookie,
}

impl Render for Error {
    fn render(&self) -> Markup {
        // for special handling of errors
        let error_desc = match self {
            UUIDError(e)=> e.to_string(),
            SqlError(e) => e.to_string(),
            JWTError(e) => e.to_string(),
            MissingCookie=>"Missing cookie".to_owned(),
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

    let jwt_id = jwt::sign::<String>(id.to_string())?;
    cookies.add(
        Cookie::build((COOKIE_UUID_NAME, jwt_id))
            .path("/")
            .secure(false)
            .http_only(true)
            .into(),
    );
    Ok(Redirect::to("/"))
}

async fn register_get(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Markup {
    maud::html! {
        head {
            (CSS("/files/style.css"));
        }
        body {
            (nav("/accounts/register", &cookies, &pool).await);
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
                p {
                    "Already have an account?"
                    a href="/accounts/login" {"Login"}
                }
            }
        }

    }
}

async fn login_get(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Markup {
    html! {
        head {
            (CSS("/files/style.css"));
        }
        body {
            (nav("/accounts/login", &cookies, &pool).await);
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
                p {
                    "Don't have an account?"
                    a href="/accounts/register" {"Register"}
                }
            }
        }
    }
}

pub struct Account {
    pub name: String,
    pub id: uuid::Uuid,
}

pub fn get_id(
    cookies: &Cookies,
) -> Result<uuid::Uuid, Error> {
    let cookie = cookies
        .get(COOKIE_UUID_NAME)
        .ok_or(Error::MissingCookie)?;
    let uuid_str: String = jwt::verify(cookie.value())?;
    Ok(uuid::Uuid::parse_str(&uuid_str)?)
}

pub async fn get_acc(
    cookies: &Cookies,
    pool: &PgPool,
) -> Result<Account, Error> {
    let id = get_id(cookies)?;
    let name = sqlx::query!(
        r#"
SELECT (name) FROM inter.accounts
WHERE (id=$1)"#,
id
    )
    .fetch_one(pool).await?.name;
    Ok(Account{name, id})
}

pub async fn get_nav(
    url: &str,
    cookies: &Cookies,
    pool: &PgPool,
) -> Markup {
    match accounts::get_acc(cookies, pool).await {
        Ok(acc)=>html!{
            span.right {
                "Olá"
                a href="#" {(acc.name)}
            }
        },
        Err(_)=>html!{
            span.right {
                "Faça" a."current-page"[url=="/accounts/login"] href="/accounts/login" {"login"}
                " ou " a."current-page"[url=="/accounts/register"] href="/accounts/register" {"Registre-se"}
            }
        },
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

    let jwt_id = jwt::sign::<String>(id.to_string())?;
    cookies.add(
        Cookie::build((COOKIE_UUID_NAME, jwt_id))
            .path("/")
            .secure(false)
            .http_only(true)
            .into(),
    );
    Ok(Redirect::to("/"))
}

