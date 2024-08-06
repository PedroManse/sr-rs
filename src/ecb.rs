use self::Error::*;
use crate::*;
use axum::{
    extract::*,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use maud::html;
use maud::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No such Clip #{0}")]
    NotFoundError(i32),
    #[error("No such Named Clip #{0}")]
    NamedNotFoundError(String),
    #[error("You can't access Clip #{0}")]
    UnauthClip(i32),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

pub fn service() -> Router<PgPool> {
    Router::new()
        .route("/", get(index))
        .route("/random", post(send_random))
        .route("/random", get(query_random))
        .route("/named", post(send_named))
        .route("/named", get(query_named))
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        html! {
            h1 { "EasyClipBoard error" };
            h2 { (self) };
        }
        .into_response()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ECBSend {
    content: String,
}
#[derive(serde::Deserialize, Debug)]
pub struct ECBGet {
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
    Ok(maud::html! {
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
    Ok(maud::html! {
        fieldset #"swap" {
            legend {"CLIP: #"(code)}
            p {(info.content)};
        }
    })
}

#[derive(serde::Deserialize, Debug)]
pub struct ECBSendNamed {
    content: String,
    name: String,
}
#[derive(serde::Deserialize, Debug)]
pub struct ECBGetNamed {
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
    Ok(maud::html! {
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
    .or(Err(NamedNotFoundError(name.clone())))?
    .content;
    Ok(maud::html! {
        fieldset #"swap" {
            legend {"CLIP: \""(&name) "\""}
            p{ (content) };
        }
    })
}

async fn index(State(pool): State<PgPool>, cookies: Cookies) -> Markup {
    maud::html! {
    head {
        (JS("/files/js/ecb.js"));
        (HTMX);
        (CSS("/files/style.css"));
        (CSS("/files/css/ecb.css"));
    }
    body {
        (nav("/ecb", &cookies, &pool).await);
        div id="content" {
        div.ecb-half {
            select {
                option value="random" {"Random"}
                option value="named" {"Named"}
            }
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
                    input placeholder="name" name="name" type="string" {}
                    button {"search"}
                }
            }
        }
        div #swap { }
        }
    }
    }
}
