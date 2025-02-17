use super::*;
use sqlx::PgPool;
use url::Url;

pub struct UserRef(pub i64);
pub struct ItemRef(pub i64);

pub enum Item {
    Post(Post),
    Comment(Comment),
    GalleryPost(GalleryPost),
    ArticlePost(ArticlePost),
    Repost(Repost),
}

macro_rules! defItem {
    ($name:ident { $( $t:tt )* } )=> {
        pub struct $name {
            pub posted: chrono::DateTime<chrono::Utc>,
            pub poster: UserRef,
            pub id: ItemRef,
            $( $t )*
        }
    };
}

pub struct ItemCommonInfo{
    pub posted: chrono::DateTime<chrono::Utc>,
    pub poster: UserRef,
    pub id: ItemRef,
}

impl Item {
    async fn new_item(poster: UserRef) -> Result<ItemCommonInfo>
    pub(super) async fn new_post(poster: UserRef, text: &str) -> Result<Post> { todo!() }
    pub(super) async fn new_comment(poster: UserRef, parent: ItemRef, text: &str) -> Result<Post> { todo!() }
    pub(super) async fn new_gallery(poster: UserRef, text: &str, photos: Vec<Url>) -> Result<Post> { todo!() }
    pub(super) async fn new_article(poster: UserRef, text: &str, article: Url) -> Result<Post> { todo!() }
    pub(super) async fn new_repost(poster: UserRef, text: &str, parent: ItemRef) -> Result<Post> { todo!() }
}

defItem!(Post {
    pub text: String,
});

defItem!(Comment {
    pub parent: ItemRef,
    pub text: String,
});

impl Comment {
    pub async fn get_parent(id: ItemRef, pool: &PgPool) -> Result<Item> {
        todo!()
    }
}

// id -> gallery's photos
defItem!(GalleryPost {
    pub title: String,
});

struct GalleryPhoto {
    url: Url,
}

impl Comment {
    pub async fn get_photos(id: ItemRef, pool: &PgPool) -> Result<Vec<GalleryPhoto>> {
        todo!()
    }
}

defItem!(ArticlePost {
    pub title: String,
    pub article: String, // -> URL
});

defItem!(Repost {
    pub comment: String,
    pub original: ItemRef,
});

impl Repost {
    pub async fn get_original(id: ItemRef, pool: &PgPool) -> Result<Item> {
        todo!()
    }
}
