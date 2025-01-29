pub mod filters;
pub mod items;
use items::*;
use url::Url;
use uuid::Uuid;
use sqlx::PgPool;

pub struct User {
    pub id: UserId,
    pub manager: Uuid,
    pub username: String,
}

// impl Copy is Ok because repr is transparent
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UserId(i64);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ItemId(i64);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ResourceId(i64);

impl ResourceId {
    async fn get_link(&self, pool: &PgPool) -> Result<Url, ()> {
        let qr = sqlx::query!("SELECT url FROM soc.resource WHERE id=$1", 0).fetch_one(pool).await.unwrap();
        Ok(url::Url::parse(&qr.url).unwrap())
    }
}

pub trait PartialItem {
    fn _get_poster(&self) -> UserId;
    fn _get_id(&self) -> ItemId;
    fn _get_publish_date(&self) -> &chrono::DateTime<chrono::Utc>;
}

pub trait Item: PartialItem {
    fn get_poster(&self) -> UserId {
        self._get_poster()
    }
    fn get_id(&self) -> ItemId {
        self._get_id()
    }
    fn get_publish_date(&self) -> &chrono::DateTime<chrono::Utc> {
        self._get_publish_date()
    }
    fn get_link(&self) -> String;
    fn get_body(&self, db: &PgPool) -> impl std::future::Future<Output = String> + Send;
    // interactions
    // fn comment(&mut self, text: &str)
}

pub trait ItemFilter<I: Item> {
    fn filter(&self, item: &I, pool: &PgPool) -> impl std::future::Future<Output = bool> + Send;
}

