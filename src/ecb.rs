use self::Error::*;
use crate::*;
use axum::{
    extract::*,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use maud::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No such Clip #{0}")]
    NotFoundError(i32),
    #[error("No such Clip Named \"{0}\"")]
    NameNotFoundError(String),
    #[error("You can't access Clip #{0}")]
    UnauthClip(i32),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    CryptError(#[from] crypt::Error),
    #[error("Can't decrypt Clip, possibly wrong password")]
    FailedDecryption,
}

pub fn service() -> Router<PgPool> {
    Router::new()
        .route("/", get(index))
        .route("/random", post(send_random))
        .route("/random", get(query_random))
        .route("/named", post(send_named))
        .route("/named", get(query_named))
        .route("/private", post(send_private))
        .route("/private", get(query_private))
}

pub fn get_nav(
    url: &str,
) -> Markup {
    let selected = url.starts_with("/ecb");
    html!{
        span {
            a."current-page"[selected] href="/ecb" {"EasyClipBoard"}
        }
    }
}

impl DescribeError for Error {
    fn describe(&self) -> (axum::http::StatusCode, String) {
        (axum::http::StatusCode::BAD_REQUEST, self.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (code, desc) = self.describe();
        (code, html! {
            h1 { "EasyClipBoard error" };
            h2 { (desc) };
        }).into_response()
    }
}

#[derive(serde::Deserialize, Debug)]
struct ECBSend {
    content: String,
}
#[derive(serde::Deserialize, Debug)]
struct ECBGet {
    code: i32,
}

async fn query_random(
    State(pool): State<PgPool>,
    Query(params): Query<ECBGet>,
) -> Result<Markup, Error> {
    let code = params.code;
    let content = sqlx::query!(
        "
SELECT (content)
FROM ecb.random
WHERE id=$1
",
        code
    )
    .fetch_one(&pool)
    .await
    .or(Err(NotFoundError(code)))?
    .content;
    Ok(html! {
        fieldset #"swap" {
            legend {"CLIP: #"(code)}
            p{ (content) };
        }
    })
}

async fn send_random(
    State(pool): State<PgPool>,
    Form(info): Form<ECBSend>,
) -> Result<Markup, Error> {
    let code = (fxhash::hash64(&info.content) % 10000) as i32;
    sqlx::query!(
        "
INSERT INTO ecb.random (id, content)
VALUES ($1, $2)
ON CONFLICT (id)
DO UPDATE SET content=$2;
",
        code,
        &info.content
    )
    .execute(&pool)
    .await?;
    Ok(html! {
        fieldset #"swap" {
            legend {"CLIP: #"(code)}
            p {(info.content)};
        }
    })
}

#[derive(serde::Deserialize, Debug)]
struct ECBSendNamed {
    content: String,
    name: String,
}
#[derive(serde::Deserialize, Debug)]
struct ECBGetNamed {
    name: String,
}

async fn send_named(
    State(pool): State<PgPool>,
    Form(params): Form<ECBSendNamed>,
) -> Result<Markup, Error> {
    let name = params.name;
    let content = params.content;
    sqlx::query!("
INSERT INTO ecb.named (name, content)
VALUES ($1, $2)
ON CONFLICT (name)
DO UPDATE SET content=$2
", &name, &content).execute(&pool).await?;
    Ok(html! {
        fieldset #"swap" {
            legend {"CLIP: \"" (name) "\""}
            p {(content)};
        }
    })
}

async fn query_named(
    State(pool): State<PgPool>,
    Query(params): Query<ECBGetNamed>,
) -> Result<Markup, Error> {
    let name = params.name;
    let content = sqlx::query!("
SELECT (content)
FROM ecb.named
WHERE name=$1;
", &name)
    .fetch_one(&pool)
    .await
    .or(Err(NameNotFoundError(name.clone())))?
    .content;
    Ok(html! {
        fieldset #"swap" {
            legend {"CLIP: \""(&name) "\""}
            p{ (content) };
        }
    })
}

#[derive(serde::Deserialize, Debug)]
struct ECBSendPrivate{
    content: String,
    name: String,
    password: String,
}

#[derive(serde::Deserialize, Debug)]
struct ECBGetPrivate {
    name: String,
    password: String,
}

async fn send_private(
    State(pool): State<PgPool>,
    Form(params): Form<ECBSendPrivate>,
) -> Result<Markup, Error> {
    let name = params.name;
    let content = crypt::encrypt(&params.content, &params.password)?;
    sqlx::query!("
INSERT INTO ecb.private (name, content)
VALUES ($1, $2)
ON CONFLICT (name)
DO UPDATE SET content=$2;
", name, content).execute(&pool).await?;
    Ok(html! {
        fieldset #"swap" {
            legend {"CLIP: #"(name)}
            p {(params.content)};
        }
    })
}

async fn query_private(
    State(pool): State<PgPool>,
    Query(params): Query<ECBGetPrivate>,
) -> Result<Markup, Error> {
    let enc_cont = sqlx::query!("
SELECT (content)
FROM ecb.private
WHERE name=$1;
", &params.name).fetch_one(&pool)
        .await
        .or(Err(NameNotFoundError(params.name.clone())))?
        .content;
    let content = crypt::decrypt(enc_cont, params.password)?;
    let content = std::str::from_utf8(&content).or(Err(FailedDecryption))?;
    Ok(html! {
        fieldset #"swap" {
            legend {"CLIP: #"(params.name)}
            p {(content)};
        }
    })
}

async fn index(State(pool): State<PgPool>, cookies: Cookies) -> Markup {
    html! {
    head {
        (DOCTYPE);
        (JS("/files/js/ecb.js"));
        (HTMX);
        (CSS("/files/style.css"));
        (CSS("/files/css/ecb.css"));
    }
    body {
        (nav("/ecb", &cookies, &pool).await);
        div id="content" {
        div.ecb-half {

            fieldset {
                legend {"Select Clip type"}
                label for="select-random" {
                    input
                        value="random" name="select-clip"
                        type="radio" id="select-random"
                        checked {}
                    "Random"
                }
                label for="select-named" {
                    input
                        value="named" name="select-clip"
                        type="radio" id="select-named" {}
                    "Named"
                }
                label for="select-private" {
                    input
                        value="private" name="select-clip"
                        type="radio" id="select-private" {}
                    "Private"
                }
            }

            // writers
            fieldset
                class="stat send"
                id="write-random"
            {
                legend {"Write - Random"}
                form
                    hx-post="/ecb/random"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    textarea name="content" {}
                    button {"create"}
                }
            }
            fieldset
                class="stat invisible send"
                id="write-private"
            {
                legend {"Write - Private"}
                form
                    hx-post="/ecb/private"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    input placeholder="Clip name" name="name" type="text" {}
                    input placeholder="Clip password" name="password" type="password" {}
                    br {}
                    textarea name="content" {}
                    button {"create"}
                }
            }
            fieldset
                class="stat invisible send"
                id="write-named"
            {
                legend {"Write - Named"}
                form
                    hx-post="/ecb/named"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    input placeholder="Clip name" name="name" type="text" {}
                    br {}
                    textarea name="content" {}
                    button {"create"}
                }
            }

            // readers
            fieldset
                class="stat get"
                id="read-random"
            {
                legend {"Read - Random"}
                form
                    hx-get="/ecb/random"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    input.not-incremental placeholder="code" name="code" type="number" {}
                    button {"search"}
                }
            }
            fieldset
                class="stat get invisible"
                id="read-named"
            {
                legend {"Read - Named"}
                form
                    hx-get="/ecb/named"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    input placeholder="name" name="name" type="text" {}
                    button {"search"}
                }
            }
            fieldset
                class="stat get invisible"
                id="read-private"
            {
                legend {"Read - Private"}
                form
                    hx-get="/ecb/private"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    input placeholder="Clip name" name="name" type="text" {}
                    input placeholder="Clip password" name="password" type="password" {}
                    button {"search"}
                }
            }
        }
        div #swap { }
        }
    }
    }
}
