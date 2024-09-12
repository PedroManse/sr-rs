#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    AccountError(#[from] crate::accounts::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Authorization Error: {0}")]
    AuthError(String),
    #[error("Business logic Error: {0}")]
    LogicError(String),
}
use self::Error::*;
use axum::http::StatusCode;

//TODO SqlxError actually handle different db errors
impl DescribeError for Error {
    fn describe(&self) -> (StatusCode, String) {
        // for special handling of errors
        let code = match self {
            AccountError(e) => return e.describe(),
            SqlxError(_) => StatusCode::BAD_REQUEST,
            AuthError(_) => StatusCode::FORBIDDEN,
            LogicError(_) => StatusCode::FORBIDDEN,
        };
        ( code, format!("{self:?}") )
    }
}

//TODO SqlxError actually handle different db errors
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        format!("{:?}", self).into_response()
    }
}

use crate::*;
use axum::{
    extract::*,
    response::{IntoResponse, Json as JSON},
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_cookies::Cookies;

async fn tdep() -> impl IntoResponse {
    html! {"TODO"}
}

pub fn service() -> Router<PgPool> {
    Router::new()
        .route("/", get(index))
        .route("/self", get(tdep))

        .route("/calendar/:id", get(get_calendar_entry))
        .route("/calendar/:id", patch(update_calendar_entry))
        .route("/calendar/:id", delete(delete_calendar_entry))
        .route("/calendar", post(make_calendar_entry))
        .route("/calendar", get(get_calendar_entries))

        .route("/note", get(get_notes))
        .route("/note", post(make_note))
        .route("/note/:id", get(get_note))
        .route("/note/:id", put(update_note))
        .route("/note/:id", delete(delete_note))

        .route("/groups", get(list_groups))
        .route("/group", post(create_group))
        .route("/group/:id", get(get_group))
        .route("/group/:id", delete(delete_group))
        .route("/group/:id", patch(update_group))
        .route("/group/:id/users", get(list_group_users))
        .route("/group/:id/user/:uid", delete(remove_user))
        .route("/group/:id/user/:uid", post(invite_to_group))
        .route("/group/invites", get(list_invites))
        .route("/group/invite/:id", post(answer_invite))
}


#[derive(serde::Serialize)]
struct GroupInvite {
    group_id: Uuid,
    group_name: String,
}

#[derive(serde::Deserialize)]
#[serde(tag = "action")]
enum InviteAction {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "refuse")]
    Refuse,
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
    fn contain(content: Vec<T>, total_count: i64, page_size: i64, page_index: i64) -> Self {
        Self {
            taken: content.len() as i64,
            total_count,
            total_pages: total_count / page_size,
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
    sqlx::query_as!(
        CalendarEntry,
        "
SELECT id, time, title, description
FROM meet.user_calendar_entries
WHERE owner_id=$1 AND id=$2
",
        owner_id,
        entry_id
    )
    .fetch_one(&pool)
    .await
    .map(axum::Json)
    .map_err(Error::from)
}

async fn make_calendar_entry(
    cookies: Cookies,
    State(pool): State<PgPool>,
    Json(entry): JSON<CalendarCreator>,
) -> Result<JSON<IDResult>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(
        IDResult,
        "
INSERT INTO meet.user_calendar_entries
(time, title, description, owner_id)
VALUES ($1, $2, $3, $4)
RETURNING id",
        entry.time,
        entry.title,
        entry.description,
        owner_id
    )
    .fetch_one(&pool)
    .await
    .map(axum::Json)
    .map_err(Error::from)
}

async fn delete_calendar_entry(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<CalendarEntry>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(
        CalendarEntry,
        "
DELETE FROM meet.user_calendar_entries
WHERE id=$1 AND owner_id=$2
RETURNING id, time, title, description",
        entry_id,
        owner_id
    )
    .fetch_one(&pool)
    .await
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
    sqlx::query_as!(
        CalendarEntry,
        "
UPDATE meet.user_calendar_entries as e
SET
    time=COALESCE(e.time, $1),
    title=COALESCE(e.title, $2),
    description=COALESCE(e.description, $3)
WHERE id=$4 AND owner_id=$5
RETURNING id, time, title, description
",
        entry.time,
        entry.title,
        entry.description,
        entry_id,
        owner_id
    )
    .fetch_one(&pool)
    .await
    .map(axum::Json)
    .map_err(Error::from)
}

async fn get_calendar_entries(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Query(calendar_query): Query<GetCalendarEntries>,
) -> Result<JSON<Paginate<CalendarEntry>>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    let page_index = calendar_query.page;
    let page_size = calendar_query.page_size;
    let offset = page_index * page_size;
    let entries_count = sqlx::query!(
        "
SELECT COUNT(*)
FROM meet.user_calendar_entries
WHERE owner_id=$1
",
        owner_id
    )
    .fetch_one(&pool)
    .await?
    .count
    .unwrap_or(0);
    let entries = sqlx::query_as!(
        CalendarEntry,
        "
SELECT id, title, description, time
FROM meet.user_calendar_entries
WHERE owner_id=$1
LIMIT $2 OFFSET $3
",
        owner_id,
        page_size,
        offset
    )
    .fetch_all(&pool)
    .await?;
    Ok(axum::Json(Paginate::contain(
        entries,
        entries_count,
        page_size,
        page_index,
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
    sqlx::query_as!(
        Note,
        "
INSERT INTO meet.notes
(content, owner_id) VALUES ($1, $2)
RETURNING id, content;
",
        note.content,
        owner_id
    )
    .fetch_one(&pool)
    .await
    .map(axum::Json)
    .map_err(Error::from)
}

async fn get_note(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<Json<Note>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(
        Note,
        "
SELECT content, id
FROM meet.notes
WHERE id=$1 AND owner_id=$2
",
        entry_id,
        owner_id
    )
    .fetch_one(&pool)
    .await
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
    sqlx::query_as!(
        Note,
        "
UPDATE meet.notes
SET content=COALESCE($1, content)
WHERE id=$2 AND owner_id=$3
RETURNING content, id
",
        entry.content,
        entry_id,
        owner_id
    )
    .fetch_one(&pool)
    .await
    .map(axum::Json)
    .map_err(Error::from)
}

async fn delete_note(
    Path(entry_id): Path<i32>,
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<Note>, Error> {
    let owner_id = crate::accounts::get_acc(&cookies, &pool).await?.id;
    sqlx::query_as!(
        Note,
        "
DELETE FROM meet.notes
WHERE id=$1 AND owner_id=$2
RETURNING content, id
",
        entry_id,
        owner_id
    )
    .fetch_one(&pool)
    .await
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
    let entries_count = sqlx::query!(
        "
SELECT COUNT(*)
FROM meet.notes
WHERE owner_id=$1
",
        owner_id
    )
    .fetch_one(&pool)
    .await?
    .count
    .unwrap_or(0);
    let entries = sqlx::query_as!(
        Note,
        "
SELECT id, content
FROM meet.notes
WHERE owner_id=$1
LIMIT $2 OFFSET $3
",
        owner_id,
        page_size,
        offset
    )
    .fetch_all(&pool)
    .await?;
    Ok(axum::Json(Paginate::contain(
        entries,
        entries_count,
        page_size,
        page_index,
    )))
}

#[derive(serde::Deserialize)]
struct GroupCreator {
    name: String,
    description: Option<String>,
}

#[derive(serde::Deserialize)]
struct GroupUpdator {
    owner_id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
}

#[derive(serde::Serialize)]
struct Group {
    id: Uuid,
    name: String,
    description: Option<String>,
}

async fn list_groups(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<Vec<Group>>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    let ids: Vec<Uuid> = sqlx::query!(r#"
SELECT group_id FROM meet.group_users
WHERE user_id=$1"#, acc.id)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|s|s.group_id)
        .collect();
    sqlx::query_as!(Group, r#"
SELECT g.id, name, description
FROM meet.groups as g
INNER JOIN meet.group_users as u ON g.id=u.group_id
WHERE id IN (SELECT unnest($1::uuid[])) AND u.user_id=$2
"#, &ids, acc.id)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn get_group(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(group_query): Path<Uuid>,
) -> Result<JSON<Group>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    sqlx::query_as!(Group, r#"
SELECT id, name, description
FROM meet.groups as g
INNER JOIN meet.group_users as u ON g.id=u.group_id
WHERE g.id=$1 AND u.user_id=$2
"#, group_query, acc.id)
        .fetch_one(&pool)
        .await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn update_group(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(group_id): Path<Uuid>,
    Json(entry): JSON<GroupUpdator>,
) -> Result<JSON<Group>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    sqlx::query_as!(Group, r#"
UPDATE meet.groups as g SET
  owner_id=COALESCE(g.owner_id, $1),
  name=COALESCE(g.name, $2),
  description=COALESCE(g.description, $3)
WHERE g.id=$4 AND g.owner_id=$5
RETURNING
  id, name, description
"#, entry.owner_id, entry.name,
entry.description, group_id, acc.id)
        .fetch_one(&pool)
        .await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn create_group(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(entry): JSON<GroupCreator>,
) -> Result<JSON<Group>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    let group = sqlx::query_as!(Group, r#"
INSERT INTO meet.groups
  (owner_id, name, description)
VALUES
  ($1, $2, $3)
RETURNING
  id, name, description
"#, acc.id, entry.name, entry.description)
        .fetch_one(&pool)
        .await?;
    sqlx::query!(r#"
INSERT INTO meet.group_users
  (user_id, group_id)
VALUES
  ($1, $2)
"#, acc.id, group.id).execute(&pool).await?;
    Ok(axum::Json(group))
}

async fn delete_group(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(group_query): Path<Uuid>,
) -> Result<JSON<Group>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    sqlx::query_as!(Group, r#"
DELETE FROM meet.groups as g
WHERE g.id=$1 AND g.owner_id=$2
RETURNING id, name, description
"#, group_query, acc.id)
        .fetch_one(&pool)
        .await
        .map(axum::Json)
        .map_err(Error::from)
}

use accounts::Account;
async fn list_group_users(
    State(pool): State<PgPool>,
    Path(group_id): Path<Uuid>,
    cookies: Cookies,
) -> Result<JSON<Vec<Account>>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    let on_group = sqlx::query!(r#"
SELECT COALESCE(COUNT(*), 0) as "count!" FROM meet.group_users as g
WHERE g.group_id=$1 AND g.user_id=$2
LIMIT 1
"#, group_id, acc.id)
        .fetch_one(&pool)
        .await?
        .count == 1;
    if !on_group {
        Err(AuthError("Only members can list group users".to_owned()))?;
    }
    sqlx::query_as!(Account, r#"
SELECT id, name FROM inter.accounts as a
INNER JOIN meet.group_users as g ON g.user_id=a.id
WHERE g.group_id=$1
"#, group_id)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn invite_to_group(
    State(pool): State<PgPool>,
    Path((group_id, user_id)): Path<(Uuid, Uuid)>,
    cookies: Cookies,
) -> Result<StatusCode, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    let is_group_owner = sqlx::query!(r#"
SELECT owner_id FROM meet.groups
WHERE id=$1
"#, group_id)
        .fetch_one(&pool)
        .await?
        .owner_id == acc.id;
    if !is_group_owner {
        Err(AuthError("Only owners can invite users to group".to_owned()))?;
    }
    sqlx::query!(r#"
INSERT into meet.group_invites
  (user_id, group_id)
VALUES
  ($1, $2)
"#, user_id, group_id)
        .execute(&pool)
        .await?;
    Ok(StatusCode::CREATED)
}

async fn remove_user(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((group_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    let is_group_owner = sqlx::query!(r#"
SELECT owner_id FROM meet.groups
WHERE id=$1
"#, group_id)
        .fetch_one(&pool)
        .await?
        .owner_id == acc.id;
    if !is_group_owner {
        Err(AuthError("Only owners can remove users from group".to_owned()))?;
    }
    if acc.id == user_id {
        Err(LogicError("Can't remove self from group".to_owned()))?;
    }
    sqlx::query!(r#"
DELETE FROM meet.group_users
WHERE user_id=$1 AND group_id=$2
"#, user_id, group_id).execute(&pool).await?;
    Ok(StatusCode::OK)
}

async fn list_invites(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<JSON<Vec<GroupInvite>>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    sqlx::query_as!(GroupInvite, r#"
SELECT g.id as "group_id", g.name as "group_name"
FROM meet.groups as g
INNER JOIN meet.group_invites as i ON i.group_id=g.id
WHERE i.user_id=$1
"#, acc.id)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(Error::from)
}

async fn answer_invite(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(invite_id): Path<i32>,
    Json(action): JSON<InviteAction>
) -> Result<JSON<Option<Group>>, Error> {
    let acc = crate::accounts::get_acc(&cookies, &pool).await?;
    let group_id = sqlx::query!(r#"
DELETE FROM meet.group_invites
WHERE user_id=$1 AND invite_id=$2
RETURNING group_id
"#, acc.id, invite_id)
        .fetch_one(&pool)
        .await?.group_id;
    if let InviteAction::Refuse = action {
        return Ok(axum::Json(None))
    }
    sqlx::query!(r#"
INSERT INTO meet.group_users
  (group_id, user_id)
VALUES
  ($1, $2)
"#, group_id, acc.id)
        .execute(&pool)
        .await?;
    sqlx::query_as!(Group, r#"
SELECT id, name, description
FROM meet.groups as g
WHERE g.id=$1
"#, group_id)
        .fetch_one(&pool)
        .await
        .map(Some)
        .map(axum::Json)
        .map_err(Error::from)
}

async fn index(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<Markup, axum::response::Redirect> {
    Ok(html!{
        (DOCTYPE)
        head {
            (CSS("/files/style.css"));
            (CSS("/files/css/meet.css"));
            (JS("/files/js/meet.js"));
        }
        body {
            (nav("/meet/user", &cookies, &pool).await);
            div id="container" {
                div id="groups" { "groups" }
                div id="calendar" { "calendar" }
                div id="notes" {
                    div id="note-creator-container" {}
                    div id="note-container" {}
                }
            }
        }
    })
}

