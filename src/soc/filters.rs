use std::marker::PhantomData;

use super::*;

pub struct OrFilter<I: ItemInfo, F: ItemFilter<I>> {
    pub filters: Vec<F>,
    pub item_type: PhantomData<I>,
}

impl<I, F> ItemFilter<I> for OrFilter<I, F>
where
    I: ItemInfo + Sync,
    F: ItemFilter<I> + Sync,
{
    fn filter(&self, item: &I) -> bool {
        for f in self.filters.iter() {
            if f.filter(item) {
                return true;
            }
        }
        false
    }
}

pub struct AndFilter<I: ItemInfo, F: ItemFilter<I>> {
    pub filters: Vec<F>,
    pub item_type: PhantomData<I>,
}

impl<I, F> ItemFilter<I> for AndFilter<I, F>
where
    I: ItemInfo,
    F: ItemFilter<I>,
{
    fn filter(&self, item: &I) -> bool {
        for f in self.filters.iter() {
            if !f.filter(item) {
                return false;
            }
        }
        true
    }
}

pub struct HasText {
    pub text: String,
}

impl<I> ItemFilter<I> for HasText
where
    I: ItemInfo,
{
    fn filter(&self, item: &I) -> bool {
        item.get_body().contains(&self.text)
    }
}
