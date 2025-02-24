use axum::extract::{Path, State};
use axum::{debug_handler, Json};

use super::*;
use models::*;

pub async fn get_list(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<List>> {
    List::get(&pool, id).await.map(Json)
}

pub async fn make_list() { }
pub async fn delete_list() { }
pub async fn edit_list() { }

pub async fn add_item() { }
pub async fn remove_item() { }
pub async fn edit_item() { }
pub async fn swap_item_placement() { }
pub async fn add_fulfilled() { }
pub async fn remove_fulfilled() { }

