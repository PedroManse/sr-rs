use super::*;
use sqlx::PgPool;
use url::Url;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemRef(pub i64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserRef(pub i64);

#[derive(sqlx::FromRow)]
pub struct User {
    id: UserRef,
    manager_id: accounts::AccountRef,
    username: String,
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Clone)]
#[sqlx(rename_all = "lowercase", type_name = "item_type")]
pub enum ItemType {
    Post,
    Comment,
    Gallery,
    Article,
    Repost,
}

pub enum AnyItem {
    Post(Post),
    Comment(Comment),
    GalleryPost(GalleryPost),
    ArticlePost(ArticlePost),
    Repost(Repost),
}

macro_rules! defItem {
    ($name:ident { $( $t:tt )* } )=> {
        pub struct $name {
            pub posted_time: chrono::DateTime<chrono::Utc>,
            pub poster: UserRef,
            pub id: ItemRef,
            $( $t )*
        }
        impl $name {
            pub async fn get_poster(&self, pool: &PgPool) -> Result<User> {
                let u = sqlx::query!("SELECT manager_id, username FROM soc.user WHERE id=$1", self.poster.0).fetch_one(pool).await?;
                Ok(User {
                    id: self.poster.clone(),
                    username: u.username,
                    manager_id: accounts::AccountRef(u.manager_id)
                })
            }
        }
    };
}

#[derive(sqlx::FromRow)]
pub struct ItemCommonInfo {
    pub posted_time: chrono::DateTime<chrono::Utc>,
    pub poster: UserRef,
    pub id: ItemRef,
    pub item_type: ItemType,
}

defItem!(Post {
    pub text: String,
});

defItem!(Comment {
    pub parent: ItemRef,
    pub text: String,
});

defItem!(GalleryPost {
    pub title: String,
    pub photos: Vec<Url>,
});

defItem!(ArticlePost {
    pub title: String,
    pub article: Url,
});

defItem!(Repost {
    pub comment: String,
    pub original: ItemRef,
});


