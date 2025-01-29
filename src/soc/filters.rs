use std::marker::PhantomData;

use super::*;

pub struct OrFilter<I: Item, F: ItemFilter<I>> {
    pub filters: Vec<F>,
    pub item_type:PhantomData<I>,
}

impl<I, F> ItemFilter<I> for OrFilter<I, F>
where
    I: Item + Sync,
    F: ItemFilter<I> + Sync
{
    async fn filter(&self, item: &I, pool: &PgPool) -> bool {
        for f in self.filters.iter() {
            if f.filter(item, pool).await {
                return true;
            }
        }
        return false;
    }
}


pub struct AndFilter<I: Item, F: ItemFilter<I>> {
    pub filters: Vec<F>,
    pub item_type:PhantomData<I>,
}

impl<I, F> ItemFilter<I> for AndFilter<I, F>
where
    I: Item + Sync,
    F: ItemFilter<I> + Sync
{
    async fn filter(&self, item: &I, pool: &PgPool) -> bool {
        for f in self.filters.iter() {
            if !f.filter(item, pool).await {
                return false;
            }
        }
        return true;
    }
}

pub struct HasText {
    pub text: String,
}

impl ItemFilter<Post> for HasText {
    async fn filter(&self, item: &Post, pool: &PgPool) -> bool {
        item.get_body(pool).await.contains(&self.text)
    }
}

