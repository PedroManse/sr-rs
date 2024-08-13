#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    AccountError(#[from] crate::accounts::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use self::Error::*;
        match self {
            AccountError(_)=>"Account not found",
            SqlxError(_)=>"Entry not found",
        }.into_response()
    }
}

use crate::*;
use axum::{
    extract::*,
    response::{IntoResponse, Json as JSON},
    routing::{get, post, delete, patch, put},
    Router,
};
use tower_cookies::Cookies;

async fn tdep() -> impl IntoResponse {
    html!{"TODO"}
}

pub fn service() -> Router<PgPool> {
    Router::new()
        .route("/calendar/:id", get(get_calendar_entry))
        .route("/calendar/:id", patch(update_calendar_entry))
        .route("/calendar/:id", delete(delete_calendar_entry))
        .route("/calendar", post(make_calendar_entry))
        .route("/calendar", get(get_calendar_entries))

        .route("/note", post(make_note))
        .route("/note/:id", get(get_note))
        .route("/note/:id", put(update_note))
        .route("/note/:id", delete(delete_note))
        .route("/note", get(get_notes))

        .route("/groups", get(tdep))
        .route("/group", get(tdep))
        .route("/group", post(tdep))
        .route("/group", delete(tdep))
        .route("/group", patch(tdep))
}

#[derive(serde::Serialize, Debug)]
struct CalendarEntry {
    id: i32,
    time: chrono::NaiveDate,
    title: String,
    description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CalendarCreator {
    time: chrono::NaiveDate, // .toISOString().substr(0, 10)
    title: String,
    description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CalendarUpdator {
    time: Option<chrono::NaiveDate>,
    title: Option<String>,
    description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct IDResult {
    id: i32,
}

#[derive(serde::Serialize, Debug)]
struct Paginate<T> {
    taken: i64,
    total_count: i64,
    total_pages: i64,
    page_index: i64,
    content: Vec<T>,
}

#[derive(serde::Deserialize, Debug)]
struct GetCalendarEntries {
    page: i64,
    page_size: i64,
}

impl<T> Paginate<T> {
    fn contain(
        content: Vec<T>,
        total_count: i64,
        page_size: i64,
        page_index: i64,
    ) -> Self {
        Self {
            taken: content.len() as i64,
            total_count,
            total_pages: total_count/page_size,
            page_index,
            content,
        }
    }
}

async fn get_calendar_entry(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<CalendarEntry>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(CalendarEntry, "
SELECT id, time, title, description
FROM notebook.user_calendar_entries
WHERE owner_id=$1 AND id=$2
", owner_id, entry_id)
        .fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn make_calendar_entry(
    cookies: Cookies,
    State(pool): State<PgPool>,
    Json(entry): JSON<CalendarCreator>,
) -> Result<JSON<IDResult>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(IDResult, "
INSERT INTO notebook.user_calendar_entries
(time, title, description, owner_id)
VALUES ($1, $2, $3, $4)
RETURNING id", entry.time, entry.title, entry.description, owner_id)
        .fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn delete_calendar_entry(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<CalendarEntry>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(CalendarEntry, "
DELETE FROM notebook.user_calendar_entries
WHERE id=$1 AND owner_id=$2
RETURNING id, time, title, description", entry_id, owner_id)
        .fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn update_calendar_entry(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(entry): JSON<CalendarUpdator>,
) -> Result<JSON<CalendarEntry>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(CalendarEntry, "
UPDATE notebook.user_calendar_entries as e
SET
    time=COALESCE(e.time, $1),
    title=COALESCE(e.title, $2),
    description=COALESCE(e.description, $3)
WHERE id=$4 AND owner_id=$5
RETURNING id, time, title, description
", entry.time, entry.title, entry.description, entry_id, owner_id)
        .fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn get_calendar_entries(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Query(calendar_query): Query<GetCalendarEntries>,
) -> Result<Json<Paginate<CalendarEntry>>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    let page_index = calendar_query.page;
    let page_size = calendar_query.page_size;
    let offset = page_index * page_size;
    let entries_count = sqlx::query!("
SELECT COUNT(*)
FROM notebook.user_calendar_entries
WHERE owner_id=$1
", owner_id).fetch_one(&pool).await?.count.unwrap_or(0);
    let entries = sqlx::query_as!(CalendarEntry, "
SELECT id, title, description, time
FROM notebook.user_calendar_entries
WHERE owner_id=$1
LIMIT $2 OFFSET $3
", owner_id, page_size, offset).fetch_all(&pool).await?;
    Ok(axum::Json(Paginate::contain(
        entries, entries_count, page_size, page_index
    )))
}

#[derive(serde::Serialize, Debug)]
struct Note {
    id: i32,
    content: String,
}

#[derive(serde::Deserialize, Debug)]
struct NoteCreator {
    content: String,
}

#[derive(serde::Deserialize, Debug)]
struct NoteUpdator {
    content: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct GetNoteEntries {
    page: i64,
    page_size: i64,
}

async fn make_note(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(note): Json<NoteCreator>,
) -> Result<Json<Note>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(Note, "
INSERT INTO notebook.notes
(content, owner_id) VALUES ($1, $2)
RETURNING id, content;
", note.content, owner_id).fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn get_note(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<Json<Note>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(Note, "
SELECT content, id
FROM notebook.notes
WHERE id=$1 AND owner_id=$2
", entry_id, owner_id).fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn update_note(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(entry): JSON<NoteUpdator>,
) -> Result<JSON<Note>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(Note, "
UPDATE notebook.notes
SET content=COALESCE(content, $1)
WHERE id=$2 AND owner_id=$3
RETURNING content, id
", entry.content, entry_id, owner_id)
        .fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn delete_note(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<Note>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(Note, "
DELETE FROM notebook.notes
WHERE id=$1 AND owner_id=$2
RETURNING content, id
", entry_id, owner_id)
        .fetch_one(&pool).await
        .map(axum::Json)
        .map_err(Error::from)
}


async fn get_notes(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Query(notes_query): Query<GetNoteEntries>,
) -> Result<Json<Paginate<Note>>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    let page_index = notes_query.page;
    let page_size = notes_query.page_size;
    let offset = page_index * page_size;
    let entries_count = sqlx::query!("
SELECT COUNT(*)
FROM notebook.notes
WHERE owner_id=$1
", owner_id).fetch_one(&pool).await?.count.unwrap_or(0);
    let entries = sqlx::query_as!(Note, "
SELECT id, content
FROM notebook.notes
WHERE owner_id=$1
LIMIT $2 OFFSET $3
", owner_id, page_size, offset).fetch_all(&pool).await?;
    Ok(axum::Json(Paginate::contain(
        entries, entries_count, page_size, page_index
    )))
}
