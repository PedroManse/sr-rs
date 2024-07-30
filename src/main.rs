use axum::{extract, routing::get, Router};
use maud::Markup;
use sr_rs::*;
use tower_cookies::{Cookies, CookieManagerLayer};
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let pool = acquire_pool().await?;

    let app = Router::new()
        .route("/", get(index))
        .nest("/accounts", accounts::service())
        .nest("/ecb", ecb::service())
        .layer(CookieManagerLayer::new())
        .nest_service("/files", ServeDir::new("files"))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening in 0.0.0.0:8000");
    axum::serve(listener, app).await?;
    unreachable!()
}

async fn index(
    extract::State(pool): extract::State<PgPool>,
    cookies: Cookies,
) -> Result<Markup, Markup> {
    Ok(maud::html! {
        head {
            (CSS("/files/style.css"));
        }
        body {
            (nav("/", &cookies, &pool).await);
            div id="content" {
                h1{ "Hello!" };
            }
        }
    })
}

