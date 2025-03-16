use std::collections::HashMap;

use serde::Serialize;
use url::Url;
use super::*;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct List {
    id: i32,
    owner: Uuid,
    list_name: String,
    wishes: Vec<Wish>,
    show_fullfillers: bool
}

impl List {
    pub async fn new(pool: &PgPool, owner_id: Uuid, name: String) -> Result<List> {
        let l = sqlx::query!(r#"
INSERT INTO wishes.list
    (owner_id, list_name)
VALUES
    ($1, $2)
RETURNING
    id, show_fullfillers
            "#, owner_id, name).fetch_one(pool).await?;
        Ok(List { id: l.id, owner: owner_id, list_name: name, wishes: vec![], show_fullfillers: l.show_fullfillers })
    }
    pub async fn get_all(pool: &PgPool, owner_id: Uuid) -> Result<Vec<List>> {
        let list_ids = sqlx::query!(r#"
SELECT id FROM wishes.list WHERE owner_id=$1
            "#, owner_id).fetch_all(pool).await?;
        let mut lists = vec![];
        for list in list_ids {
            lists.push(List::get(pool, list.id).await?);
        }
        Ok(lists)
    }
    pub async fn get(pool: &PgPool, id: i32) -> Result<List> {
        let ls = sqlx::query!(r#"
SELECT 
    owner_id, list_name, show_fullfillers
FROM
    wishes.list
WHERE
    id=$1
"#, id).fetch_one(pool).await?;
        let fulfillments = sqlx::query!(r#"
SELECT
    id,
    wish_id,
    wish_fullfiller as "account",
    person
FROM
    wishes.fulfillments
WHERE
    list_id=$1
            "#, id).fetch_all(pool).await?;
        let mut fulfillments_map: HashMap<i32, Vec<Fulfillment>> = HashMap::new();
        for f in fulfillments {
            let vc = fulfillments_map.entry(f.wish_id).or_default();
            vc.push(Fulfillment{
                id: f.id,
                person: f.person,
                account: f.account,
            });
        }
        let wishes = sqlx::query!(r#"
SELECT 
	id,
	list_id,
	placement,
	link,
	text,
	total_amount,
  price,
	disabled

FROM wishes.wish
WHERE list_id=$1
            "#, id).fetch_all(pool).await?;
        let wishes: Vec<_> = wishes.into_iter().map(|w|{
            Ok(Wish{
                id: w.id,
                placement: w.placement,
                link: w.link.parse()?,
                text: w.text,
                total_amount: w.total_amount,
                fulfillments: fulfillments_map.remove(&w.id).unwrap_or_default(),
                disabled: w.disabled,
                price: w.price,
            })
        }).collect::<Result<_>>()?;
        Ok(List {
            id,
            owner: ls.owner_id,
            list_name: ls.list_name,
            wishes,
            show_fullfillers: ls.show_fullfillers
        })
    }
}

#[derive(Serialize)]
struct Wish {
    id: i32,
    placement: i32,
    link: Url,
    text: String,
    total_amount: i32,
    fulfillments: Vec<Fulfillment>,
    disabled: bool,
    price: Option<String>,
}

#[derive(Serialize)]
struct Fulfillment {
    id: i32,
    person: Option<String>,
    account: Option<Uuid>,
}
