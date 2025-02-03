use axum::extract::State;
use axum::response::Redirect;
use axum::Form;
use tower_cookies::Cookie;

use super::*;


pub async fn register_post(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Form(info): Form<FormAccount>,
) -> Result<Redirect, FError> {
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

pub async fn register_get(State(pool): State<PgPool>, cookies: Cookies) -> Markup {
    html! {
        (DOCTYPE);
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

pub async fn login_get(State(pool): State<PgPool>, cookies: Cookies) -> Markup {
    html! {
        (DOCTYPE);
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

pub fn get_id(cookies: &Cookies) -> Result<uuid::Uuid, FError> {
    let cookie = cookies.get(COOKIE_UUID_NAME).ok_or(FError::MissingCookie)?;
    let uuid_str: String = jwt::verify(cookie.value())?;
    Ok(uuid::Uuid::parse_str(&uuid_str)?)
}

pub async fn get_acc(cookies: &Cookies, pool: &PgPool) -> Result<Account, FError> {
    let id = get_id(cookies)?;
    let name = sqlx::query!(
        r#"
SELECT (name) FROM inter.accounts
WHERE (id=$1)"#,
        id
    )
    .fetch_one(pool)
    .await?
    .name;
    Ok(Account { name, id })
}

pub async fn login_post(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Form(info): Form<FormAccount>,
) -> Result<Redirect, FError> {
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
