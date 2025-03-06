use axum::response::IntoResponse;
use axum::routing::{get, post, delete, put};

pub mod models;
pub mod services;

use super::*;
pub struct WishesModule;

impl Module for WishesModule {
    async fn render(
            url: &str,
            cookies: &Cookies,
            pool: &PgPool,
        ) -> Markup {
        let selected = url.starts_with("/wish");
        html!{
            span { a."current-page"[selected] href="/wish" {"wishlist"} }
        }
    }
    fn service() -> Router<PgPool> {
        use services::*;
        Router::new()
            .route("/list/", get(list_lists))
            .route("/list/", post(make_list))
            .route("/list/{list_id}", get(get_list))
            .route("/list/{list_id}", delete(delete_list))
            .route("/list/{list_id}", put(edit_item))

            .route("/list/{list_id}/item", post(add_item))
            .route("/list/{list_id}/item/{item_id}", delete(remove_item))
            .route("/list/{list_id}/item/{item_id}", put(edit_item))
            .route("/list/{list_id}/item/swap/{item_id}/with/{other_item_id}", post(swap_item_placement))

            .route("/list/{list_id}/item/{item_id}/fulfillment", post(add_fulfilled))
            .route("/list/{list_id}/item/{item_id}/fulfillment", delete(remove_fulfilled))
    }
}

type Result<T> = std::result::Result<T, BackError>;

HTTPError!( backend BackError {
    SQLX = sqlx::Error,
    URL = url::ParseError
} );

impl APIError for BackError { }

impl IntoResponse for BackError {
    fn into_response(self) -> axum::response::Response {
        self.build_error()
    }
}

