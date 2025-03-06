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

pub async fn list_lists(
    Path(uid): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<List>>> {
    List::get_all(&pool, uid).await.map(Json)
}

pub async fn make_list(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) { }

pub async fn delete_list(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) { }

pub async fn edit_list(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) { }

pub async fn add_item(
    Path((list_id, item_id)): Path<(i32, i32)>,
) { }

pub async fn remove_item(
    Path((list_id, item_id)): Path<(i32, i32)>,
) { }

pub async fn edit_item(
    Path((list_id, item_id)): Path<(i32, i32)>,
) { }

pub async fn swap_item_placement(
    Path((list_id, item_id, other_item_id)): Path<(i32, i32, i32)>,
) { }

pub async fn add_fulfilled(
    Path((list_id, item_id)): Path<(i32, i32)>,
) { }

pub async fn remove_fulfilled(
    Path((list_id, item_id)): Path<(i32, i32)>,
) { }

