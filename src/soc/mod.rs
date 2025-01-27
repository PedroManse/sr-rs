pub mod filters;
pub mod items;
use items::*;

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

pub trait Item {
    fn get_link(&self) -> String;
    fn get_poster(&self) -> UserId;
    fn get_id(&self) -> ItemId;
    fn get_body(&self) -> &str;
    fn get_publish_date(&self) -> &chrono::DateTime<chrono::Utc>;
}

pub trait ItemFilter<I: Item> {
    fn filter(&self, item: &I) -> bool;
}

