use url::Url;
use super::*;

struct List {
    owner: Uuid,
    wishes: Vec<Wish>,
    show_fullfillers: bool
}

struct Wish {
    id: i32,
    placement: i32,
    link: Url,
    text: String,
    total_amount: i16,
    amount_fulfilled: i16,
    disabled: bool,
}

