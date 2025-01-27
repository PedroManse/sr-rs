use std::marker::PhantomData;

use super::*;

pub struct OrFilter<I: Item, F: ItemFilter<I>> {
    pub filters: Vec<F>,
    pub item_type:PhantomData<I>,
}

impl<I, F> ItemFilter<I> for OrFilter<I, F>
where
    I: Item,
    F: ItemFilter<I>
{
    fn filter(&self, item: &I) -> bool {
        self.filters.iter().any(|f|f.filter(item))
    }
}


pub struct AndFilter<I: Item, F: ItemFilter<I>> {
    pub filters: Vec<F>,
    pub item_type:PhantomData<I>,
}

impl<I, F> ItemFilter<I> for AndFilter<I, F>
where
    I: Item,
    F: ItemFilter<I>
{
    fn filter(&self, item: &I) -> bool {
        self.filters.iter().all(|f|f.filter(item))
    }
}

pub struct HasText {
    pub text: String,
}

impl ItemFilter<Post> for HasText {
    fn filter(&self, item: &Post) -> bool {
        item.get_body().contains(&self.text)
    }
}

