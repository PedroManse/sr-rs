pub mod filters;
pub mod items;
use items::*;
pub mod db;
use db::*;
use enum_dispatch::enum_dispatch;
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum SocError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
}
type Result<T> = std::result::Result<T, SocError>;

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

impl From<i64> for UserId {
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl From<i64> for ItemId {
    fn from(i: i64) -> Self {
        Self(i)
    }
}

#[enum_dispatch(AnyItem)]
#[enum_dispatch(AnyDBItem)]
pub trait PartialItem {
    fn get_poster(&self) -> UserId;
    fn get_id(&self) -> ItemId;
    fn get_publish_date(&self) -> &chrono::DateTime<chrono::Utc>;
}

pub trait DBItem {
    const ITEM_TYPE: ItemType;
}

pub trait GetItemInfo: Sized {
    fn get_info(
        common: CommonItem,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self>> + Send;
}

pub struct CommonItem {
    pub posted_time: chrono::DateTime<chrono::Utc>,
    pub poster: UserId,
    pub id: ItemId,
    pub item_type: ItemType,
}

pub trait GetById: DBItem + GetItemInfo {
    #[allow(async_fn_in_trait)]
    async fn _get(id: i64, pool: &PgPool) -> Result<Self> {
        let common = Self::get_common(id, pool).await?;
        Self::get_info(common, pool).await
    }
    #[allow(async_fn_in_trait)]
    async fn get_common(id: i64, pool: &PgPool) -> Result<CommonItem> {
        let r = sqlx::query!(
            r#"SELECT id, poster, posted_time FROM soc.item WHERE id=$1 AND item_type=$2"#,
            id,
            Self::ITEM_TYPE as ItemType,
        )
        .fetch_one(pool)
        .await?;
        Ok(CommonItem {
            id: ItemId(id),
            poster: UserId(r.poster),
            posted_time: r.posted_time,
            item_type: Self::ITEM_TYPE,
        })
    }
}

impl<I: DBItem + GetItemInfo> GetById for I {}

pub trait Item: PartialItem + Sized {
    type DBType: DBItem + IntoItem<Target = Self> + GetById + Send;
    #[allow(async_fn_in_trait)]
    async fn get(id: i64, pool: &PgPool) -> Result<Self> {
        Self::DBType::_get(id, pool)
            .await?
            .into_item(pool)
            .await
            .map_err(SocError::from)
    }
}

pub trait ItemInfo: PartialItem {
    fn get_link(&self) -> String; // TODO -> Result<URL>
    fn get_body(&self) -> String;
    // interactions
    // fn comment(&mut self, text: &str)
}

pub trait IntoItem: PartialItem {
    type Target: ItemInfo;
    fn into_item(
        self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self::Target>> + Send;
}

pub trait ItemFilter<I: ItemInfo> {
    fn filter(&self, item: &I) -> bool;
}
