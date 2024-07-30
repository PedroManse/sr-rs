use crate::*;
use maud::*;
use self::Error::*;
use axum::{
    extract::*,
    response::{IntoResponse},
    routing::{get, post},
    Router,
};
use maud::html;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No such Clip #{0}")]
    NotFoundError(i32),
    #[error("You can't access Clip #{0}")]
    UnauthClip(i32),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

pub fn service() -> Router<PgPool> {
    Router::new()
        .route("/", get(index))
        .route("/send", post(send))
        .route("/get", get(query))
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        html! {
            h1 { "EasyClipBoard error" };
            h2 { (self) };
        }.into_response()
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

async fn query(
    State(pool): State<PgPool>,
    Query(params): Query<ECBGet>,
) -> Result<Markup, Error> {
    let code = params.code;
    let content = sqlx::query!("
SELECT (content)
FROM ecb.clip
WHERE id=$1
", code).fetch_one(&pool).await.or(Err(NotFoundError(code)))?.content;
    Ok(maud::html!{
        fieldset #"swap" {
            legend {"CLIP: #"(code)}
            p{ (content) };
        }
    })
}

async fn send(
    State(pool): State<PgPool>,
    Form(info): Form<ECBSend>,
) -> Result<Markup, Error> {
    let code = (fxhash::hash64(&info.content)%10000) as i32;
    sqlx::query!("
INSERT INTO ecb.clip (id, content)
VALUES ($1, $2)
ON CONFLICT (id)
DO UPDATE SET content=$2;
", code, &info.content).execute(&pool).await?;
    println!("{:?}", info);
    Ok(maud::html!{
        fieldset #"swap" {
            legend {"CLIP: #"(code)}
            p {(info.content)};
        }
    })
}

async fn index(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Markup {
    maud::html! {
    head {
        (HTMX);
        (CSS("/files/style.css"));
        (CSS("/files/css/ecb.css"));
    }
    body {
        (nav("/ecb", &cookies, &pool).await);
        div id="content" {
        div.ecb-half {
            fieldset {
                legend {"Write"}
                form
                    hx-post="/ecb/send"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    textarea name="content" {}
                    button {"create"}
                }

            }
            fieldset {
                legend {"Read"}
                form
                    hx-get="/ecb/get"
                    hx-target="#swap"
                    hx-swap="innerHTML"
                {
                    input.not-incremental name="code" type="number" {}
                    button {"search"}
                }
            }
        }
        div #swap { }
        }
    }
    }
}

