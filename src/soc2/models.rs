use super::*;
use sqlx::PgPool;

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

struct ItemCommonInfo {
    pub posted: chrono::DateTime<chrono::Utc>,
    pub poster: UserRef,
    pub id: ItemRef,
}

impl ItemCommonInfo {
    fn new(user: UserRef, pool: &PgPool) -> Result<Self> {
        todo!()
    }
}

defItem!(Post {
    pub text: String,
});

defItem!(Comment {
    pub parent: ItemRef,
    pub text: String,
});

impl Comment {
    pub async fn get_parent(&self, pool: &PgPool) -> Result<Item> {
        todo!()
    }
}

// id -> gallery's photos
defItem!(GalleryPost {
    pub title: String,
});

defItem!(ArticlePost {
    pub title: String,
    pub article: String, // -> URL
});

defItem!(Repost {
    pub comment: String,
    pub original: ItemRef,
});

impl Repost {
    pub async fn get_original(&self, pool: &PgPool) -> Result<Item> {
        todo!()
    }
}
